use anyhow::{Context, Result};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use axum::{
    Form, Router,
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
use chrono::{Duration, Utc};
use html_escape::{encode_double_quoted_attribute, encode_text};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow, SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::{env, net::SocketAddr, str::FromStr};
use tokio::{fs, net::TcpListener};
use tower_http::services::ServeDir;

const AUTH_COOKIE: &str = "kroissant_token";

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
    jwt_secret: String,
}

#[derive(Debug)]
struct AppError(anyhow::Error);

type AppResult<T> = std::result::Result<T, AppError>;

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        eprintln!("application error: {:?}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(render_error_page("Une erreur est survenue.")),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, FromRow)]
struct Content {
    id: i64,
    slug: String,
    title: String,
    platform: String,
    duration: String,
    age_range: String,
    description: String,
    skill: String,
    image_url: String,
    source_url: String,
}

impl Content {
    fn platform_label(&self) -> &'static str {
        platform_label(&self.platform)
    }
}

#[derive(Debug, Clone, FromRow)]
struct User {
    id: i64,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i64,
    email: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
struct PlatformQuery {
    platform: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AuthQuery {
    next: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AuthForm {
    email: String,
    password: String,
    next: Option<String>,
}

#[derive(Clone)]
struct Benefit {
    key: &'static str,
    label: &'static str,
    summary: &'static str,
    detail: &'static str,
    source: &'static str,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            env::var("RUST_LOG").unwrap_or_else(|_| "tower_http=debug,kroissant=debug".to_string()),
        )
        .init();

    fs::create_dir_all("data")
        .await
        .context("creation du dossier data")?;

    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data/kroissant.sqlite".to_string());
    let connect_options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .context("connexion sqlite")?;

    migrate(&pool).await?;
    seed_fake_data(&pool).await?;

    let state = AppState {
        pool,
        jwt_secret: env::var("JWT_SECRET")
            .unwrap_or_else(|_| "dev-secret-change-me-kroissant".to_string()),
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/bibliotheque", get(library))
        .route("/partials/home", get(home_partial))
        .route("/partials/library", get(library_partial))
        .route("/contenu/{slug}", get(content_detail))
        .route("/contenu/{id}/save", post(toggle_save))
        .route("/go/{slug}", get(go_to_source))
        .route("/science", get(science))
        .route("/inscription", get(register_page).post(register))
        .route("/connexion", get(login_page).post(login))
        .route("/deconnexion", post(logout))
        .route("/compte", get(account))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let port = env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(3000);
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let addr: SocketAddr = format!("{host}:{port}")
        .parse()
        .context("adresse d'ecoute invalide")?;
    let listener = TcpListener::bind(addr).await?;

    println!("Kroissant dev server: http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn migrate(pool: &SqlitePool) -> Result<()> {
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(pool)
        .await?;

    let statements = [
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS contents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            slug TEXT NOT NULL UNIQUE,
            title TEXT NOT NULL,
            platform TEXT NOT NULL,
            duration TEXT NOT NULL,
            age_range TEXT NOT NULL,
            description TEXT NOT NULL,
            skill TEXT NOT NULL,
            image_url TEXT NOT NULL,
            source_url TEXT NOT NULL,
            sort_order INTEGER NOT NULL
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS saved_items (
            user_id INTEGER NOT NULL,
            content_id INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            PRIMARY KEY (user_id, content_id),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS watch_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            content_id INTEGER NOT NULL,
            watched_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE
        )
        "#,
    ];

    for statement in statements {
        sqlx::query(statement).execute(pool).await?;
    }

    Ok(())
}

async fn seed_fake_data(pool: &SqlitePool) -> Result<()> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contents")
        .fetch_one(pool)
        .await?;

    if count > 0 {
        return Ok(());
    }

    let rows = [
        (
            "bluey-saison-1",
            "Bluey - Saison 1",
            "disney",
            "7 min / episode",
            "3-7 ans",
            "Bluey est une petite chienne bouvier australien dotee d'une imagination sans limite. Elle fait de la vie de famille une aventure quotidienne et ludique.",
            "resilience",
            "/static/img/bluey.svg",
            "https://www.disneyplus.com/",
            1,
        ),
        (
            "puffin-rock",
            "Puffin Rock",
            "netflix",
            "12 min",
            "3-5 ans",
            "Oona et Baba explorent une ile paisible ou l'entraide, l'observation de la nature et les petites responsabilites rythment les histoires.",
            "empathie",
            "/static/img/puffin-rock.svg",
            "https://www.netflix.com/search?q=Puffin%20Rock",
            2,
        ),
        (
            "tumble-leaf",
            "Tumble Leaf",
            "youtube",
            "10 min",
            "3-6 ans",
            "Fig decouvre des objets et teste des hypotheses simples. Chaque episode transforme la curiosite en experimentation calme.",
            "creativite",
            "/static/img/tumble-leaf.svg",
            "https://www.youtube.com/results?search_query=Tumble+Leaf",
            3,
        ),
        (
            "hilda-saison-1",
            "Hilda - Saison 1",
            "netflix",
            "22 min",
            "6-10 ans",
            "Hilda traverse des mondes etranges avec curiosite. La serie encourage l'ecoute, le vocabulaire et la resolution non violente des conflits.",
            "language",
            "/static/img/hilda.svg",
            "https://www.netflix.com/search?q=Hilda",
            4,
        ),
        (
            "le-petit-prince",
            "Le Petit Prince",
            "youtube",
            "11 min",
            "4-8 ans",
            "Des recits courts qui valorisent la nuance, l'attention a l'autre et l'expression des emotions sans surstimulation visuelle.",
            "empathie",
            "/static/img/petit-prince.svg",
            "https://www.youtube.com/results?search_query=Le+Petit+Prince+dessin+anime",
            5,
        ),
        (
            "sarah-and-duck",
            "Sarah & Duck",
            "youtube",
            "8 min",
            "3-6 ans",
            "Sarah et son canard vivent des situations absurdes mais tres douces. Le rythme lent aide a nommer les petits problemes du quotidien.",
            "resilience",
            "/static/img/sarah-duck.svg",
            "https://www.youtube.com/results?search_query=Sarah+and+Duck",
            6,
        ),
        (
            "il-etait-une-fois-la-vie",
            "Il etait une fois... la vie",
            "youtube",
            "7 min",
            "5-9 ans",
            "Une porte d'entree ludique vers le corps humain, avec des sequences qui donnent envie de poser des questions scientifiques.",
            "science",
            "/static/img/once-life.svg",
            "https://www.youtube.com/results?search_query=Il+etait+une+fois+la+vie",
            7,
        ),
        (
            "motown-magic",
            "Motown Magic",
            "netflix",
            "15 min",
            "4-8 ans",
            "Les personnages apprennent a reguler frustration, joie et timidite grace a la musique et aux interactions de groupe.",
            "regulation",
            "/static/img/motown.svg",
            "https://www.netflix.com/search?q=Motown%20Magic",
            8,
        ),
        (
            "ada-twist",
            "Ada Twist, scientifique",
            "netflix",
            "12 min",
            "4-7 ans",
            "Ada observe, formule une question et teste une piste. Les episodes structurent la curiosite avec un vocabulaire accessible.",
            "science",
            "/static/img/ada-twist.svg",
            "https://www.netflix.com/search?q=Ada%20Twist",
            9,
        ),
        (
            "baymax",
            "Baymax!",
            "disney",
            "8 min",
            "5-9 ans",
            "De petites histoires de soin, d'ecoute et de soutien. Le format court facilite la discussion apres l'episode.",
            "empathie",
            "/static/img/baymax.svg",
            "https://www.disneyplus.com/",
            10,
        ),
        (
            "ernest-celestine",
            "Ernest et Celestine",
            "youtube",
            "13 min",
            "4-8 ans",
            "Un duo tendre ou les malentendus se resolvent par les mots. Bon support pour parler d'amitie et de points de vue.",
            "language",
            "/static/img/ernest.svg",
            "https://www.youtube.com/results?search_query=Ernest+et+Celestine+dessin+anime",
            11,
        ),
        (
            "storybots",
            "StoryBots",
            "netflix",
            "20 min",
            "5-8 ans",
            "Des reponses rapides aux grandes questions des enfants. Le contenu encourage a verifier, comparer et reformuler.",
            "science",
            "/static/img/storybots.svg",
            "https://www.netflix.com/search?q=StoryBots",
            12,
        ),
    ];

    for row in rows {
        sqlx::query(
            r#"
            INSERT INTO contents
                (slug, title, platform, duration, age_range, description, skill, image_url, source_url, sort_order)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(row.0)
        .bind(row.1)
        .bind(row.2)
        .bind(row.3)
        .bind(row.4)
        .bind(row.5)
        .bind(row.6)
        .bind(row.7)
        .bind(row.8)
        .bind(row.9)
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn home(State(state): State<AppState>, headers: HeaderMap) -> AppResult<Html<String>> {
    let user = current_user(&state, &headers).await;
    let selected = get_contents(&state.pool, Some("youtube"), Some(2)).await?;
    let moment = get_contents(&state.pool, None, Some(2)).await?;

    let body = format!(
        r#"
        <section class="hero">
            <div class="hero-inner">
                <h1>Des dessins animes<br>qui developpent vraiment<br>votre enfant.</h1>
                <p>Une bibliotheque de contenus selectionnes pour leurs benefices developpementaux prouves: langage, empathie, resilience, creativite.</p>
                <div class="hero-actions">
                    <a class="button button-light" href="/bibliotheque">Decouvrir les contenus</a>
                    <a class="button button-outline-light" href="/science">Comment ca developpe</a>
                </div>
            </div>
        </section>

        <main class="page-flow">
            <section class="section-block">
                <div class="section-heading split-heading">
                    <div>
                        <h2>Choisissez votre plateforme</h2>
                    </div>
                    <a class="text-link" href="/bibliotheque">Voir tout</a>
                </div>
                {}
            </section>

            <section class="section-block">
                <div class="section-heading">
                    <h2>Selection du moment</h2>
                    <p>Tous selectionnes pour leur valeur developpementale</p>
                </div>
                <div class="card-grid two-cols">
                    {}
                </div>
            </section>
        </main>

        <section class="stat-band">
            <strong>38%</strong> des enfants 4-14 ans regardent des dessins animes
            <span>Certains developpent autant que des activites structurees</span>
        </section>

        <section class="how-it-works">
            <h2>Comment ca fonctionne</h2>
            <div class="steps">
                <div><strong>1</strong><h3>Choisissez votre plateforme</h3><p>YouTube, Netflix ou Disney+</p></div>
                <div><strong>2</strong><h3>Parcourez la bibliotheque</h3><p>Chaque contenu explique ce qu'il developpe</p></div>
                <div><strong>3</strong><h3>Redirige en 1 clic</h3><p>Directement sur la plateforme</p></div>
            </div>
        </section>
        "#,
        render_home_platform_section("youtube", &selected),
        render_cards(&moment),
    );

    Ok(Html(render_page("Kroissant", "Accueil", None, &user, body)))
}

async fn home_partial(
    State(state): State<AppState>,
    Query(query): Query<PlatformQuery>,
) -> AppResult<Html<String>> {
    let platform = normalize_platform(query.platform.as_deref());
    let contents = get_contents(&state.pool, Some(platform), Some(2)).await?;
    Ok(Html(render_home_platform_section(platform, &contents)))
}

async fn library(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<PlatformQuery>,
) -> AppResult<Html<String>> {
    let user = current_user(&state, &headers).await;
    let active = normalize_platform_or_all(query.platform.as_deref());
    let contents = get_contents(
        &state.pool,
        if active == "all" { None } else { Some(active) },
        None,
    )
    .await?;
    let body = format!(
        r#"
        <main class="library-shell">
            <section id="library-section">
                {}
            </section>
        </main>
        "#,
        render_library_section(active, &contents)
    );

    Ok(Html(render_page(
        "Bibliotheque - Kroissant",
        "Accueil",
        Some("/"),
        &user,
        body,
    )))
}

async fn library_partial(
    State(state): State<AppState>,
    Query(query): Query<PlatformQuery>,
) -> AppResult<Html<String>> {
    let active = normalize_platform_or_all(query.platform.as_deref());
    let contents = get_contents(
        &state.pool,
        if active == "all" { None } else { Some(active) },
        None,
    )
    .await?;
    Ok(Html(render_library_section(active, &contents)))
}

async fn content_detail(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> AppResult<Response> {
    let user = current_user(&state, &headers).await;
    let Some(content) = get_content_by_slug(&state.pool, &slug).await? else {
        return Ok((
            StatusCode::NOT_FOUND,
            Html(render_error_page("Contenu introuvable.")),
        )
            .into_response());
    };

    let saved = match &user {
        Some(user) => is_saved(&state.pool, user.id, content.id).await?,
        None => false,
    };
    let similar = get_similar_contents(&state.pool, &content).await?;
    let benefit = benefit_for(&content.skill);

    let body = format!(
        r#"
        <section class="detail-hero">
            <div>
                <p class="eyebrow">{} · {} · {}</p>
                <h1>{}</h1>
                <p class="detail-description">{}</p>
                <span class="pill {}">{}</span>
            </div>
        </section>

        <main class="detail-layout">
            <section class="detail-actions">
                {}
                <a class="button button-primary watch-wide" href="/go/{}">Regarder sur {}</a>
            </section>

            <section class="benefit-panel">
                <h2>Ce que ce contenu developpe chez votre enfant</h2>
                <ul>
                    <li><strong>{}</strong> - {}</li>
                    <li><strong>Regulation attentionnelle</strong> - Le rythme laisse de la place pour commenter et anticiper.</li>
                    <li><strong>Dialogue parent-enfant</strong> - Les themes sont faciles a reprendre apres l'episode.</li>
                </ul>
            </section>

            <p class="source-note">Sources: analyse interne Kroissant · {}</p>

            <section class="section-block similar-block">
                <div class="section-heading">
                    <h2>Contenus similaires</h2>
                </div>
                <div class="card-grid two-cols">
                    {}
                </div>
            </section>
        </main>
        "#,
        h(content.platform_label()),
        h(&content.duration),
        h(&content.age_range),
        h(&content.title),
        h(&content.description),
        h(benefit.key),
        h(benefit.label),
        render_save_panel(&content, user.as_ref(), saved),
        a(&content.slug),
        h(content.platform_label()),
        h(benefit.label),
        h(benefit.summary),
        h(benefit.source),
        render_cards(&similar)
    );

    Ok(Html(render_page(
        &format!("{} - Kroissant", content.title),
        "Bibliotheque",
        Some("/bibliotheque"),
        &user,
        body,
    ))
    .into_response())
}

async fn toggle_save(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> AppResult<Response> {
    let Some(content) = get_content_by_id(&state.pool, id).await? else {
        return Ok((
            StatusCode::NOT_FOUND,
            Html("Contenu introuvable".to_string()),
        )
            .into_response());
    };

    let Some(user) = current_user(&state, &headers).await else {
        return Ok(Html(render_save_panel(&content, None, false)).into_response());
    };

    let already_saved = is_saved(&state.pool, user.id, content.id).await?;
    if already_saved {
        sqlx::query("DELETE FROM saved_items WHERE user_id = ? AND content_id = ?")
            .bind(user.id)
            .bind(content.id)
            .execute(&state.pool)
            .await?;
    } else {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO saved_items (user_id, content_id, created_at)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(user.id)
        .bind(content.id)
        .bind(Utc::now().to_rfc3339())
        .execute(&state.pool)
        .await?;
    }

    Ok(Html(render_save_panel(&content, Some(&user), !already_saved)).into_response())
}

async fn go_to_source(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> AppResult<Response> {
    let Some(content) = get_content_by_slug(&state.pool, &slug).await? else {
        return Ok((
            StatusCode::NOT_FOUND,
            Html(render_error_page("Contenu introuvable.")),
        )
            .into_response());
    };

    if let Some(user) = current_user(&state, &headers).await {
        sqlx::query(
            r#"
            INSERT INTO watch_history (user_id, content_id, watched_at)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(user.id)
        .bind(content.id)
        .bind(Utc::now().to_rfc3339())
        .execute(&state.pool)
        .await?;
    }

    Ok(Redirect::to(&content.source_url).into_response())
}

async fn science(State(state): State<AppState>, headers: HeaderMap) -> AppResult<Html<String>> {
    let user = current_user(&state, &headers).await;
    let mut cards = String::new();
    for benefit in benefits() {
        cards.push_str(&format!(
            r#"
            <article class="science-card">
                <span class="pill {}">{}</span>
                <p>{}</p>
                <small>Source: {}</small>
            </article>
            "#,
            h(benefit.key),
            h(benefit.label),
            h(benefit.detail),
            h(benefit.source),
        ));
    }

    let body = format!(
        r#"
        <section class="science-hero">
            <h1>Comment on selectionne<br>nos contenus</h1>
            <p>Chaque contenu est valide selon des criteres pediatriques scientifiquement etablis.</p>
        </section>
        <main class="science-flow">
            {}
            <div class="warning-box">
                <strong>54% des parents pensent que leur enfant est accro aux ecrans.</strong>
                <span>Certains dessins animes developpent les competences que vous cherchez a cultiver - sans activite structuree.</span>
            </div>
            <a class="button button-primary full-width" href="/inscription?next=/bibliotheque">Je fais confiance a cette selection - S'inscrire gratuitement</a>
            <p class="source-note centered">Sources completees: Lurie Children's Hospital 2025 · INSERM · Arcom · LeadingTree 2023</p>
        </main>
        "#,
        cards
    );

    Ok(Html(render_page(
        "Selection scientifique - Kroissant",
        "Accueil",
        Some("/"),
        &user,
        body,
    )))
}

async fn register_page(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<AuthQuery>,
) -> AppResult<Html<String>> {
    let user = current_user(&state, &headers).await;
    Ok(Html(render_auth_page(
        AuthMode::Register,
        query.next,
        None,
        user.as_ref(),
    )))
}

async fn register(
    State(state): State<AppState>,
    Form(form): Form<AuthForm>,
) -> AppResult<Response> {
    let email = form.email.trim().to_lowercase();
    let next = clean_next(form.next.clone());

    if email.is_empty() || form.password.len() < 8 {
        return Ok((
            StatusCode::BAD_REQUEST,
            Html(render_auth_page(
                AuthMode::Register,
                form.next,
                Some("Utilisez un email valide et un mot de passe de 8 caracteres minimum."),
                None,
            )),
        )
            .into_response());
    }

    let exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM users WHERE email = ?")
        .bind(&email)
        .fetch_optional(&state.pool)
        .await?;

    if exists.is_some() {
        return Ok((
            StatusCode::CONFLICT,
            Html(render_auth_page(
                AuthMode::Register,
                form.next,
                Some("Un compte existe deja pour cet email. Connectez-vous."),
                None,
            )),
        )
            .into_response());
    }

    let password_hash = hash_password(&form.password)?;
    let result = sqlx::query(
        r#"
        INSERT INTO users (email, password_hash, created_at)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(&email)
    .bind(password_hash)
    .bind(Utc::now().to_rfc3339())
    .execute(&state.pool)
    .await?;

    let token = create_jwt(result.last_insert_rowid(), &email, &state.jwt_secret)?;
    Ok(redirect_with_cookie(&next, &token))
}

async fn login_page(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<AuthQuery>,
) -> AppResult<Html<String>> {
    let user = current_user(&state, &headers).await;
    Ok(Html(render_auth_page(
        AuthMode::Login,
        query.next,
        None,
        user.as_ref(),
    )))
}

async fn login(State(state): State<AppState>, Form(form): Form<AuthForm>) -> AppResult<Response> {
    let email = form.email.trim().to_lowercase();
    let next = clean_next(form.next.clone());

    let record: Option<(i64, String)> =
        sqlx::query_as("SELECT id, password_hash FROM users WHERE email = ?")
            .bind(&email)
            .fetch_optional(&state.pool)
            .await?;

    let Some((user_id, password_hash)) = record else {
        return Ok((
            StatusCode::UNAUTHORIZED,
            Html(render_auth_page(
                AuthMode::Login,
                form.next,
                Some("Email ou mot de passe incorrect."),
                None,
            )),
        )
            .into_response());
    };

    if !verify_password(&form.password, &password_hash) {
        return Ok((
            StatusCode::UNAUTHORIZED,
            Html(render_auth_page(
                AuthMode::Login,
                form.next,
                Some("Email ou mot de passe incorrect."),
                None,
            )),
        )
            .into_response());
    }

    let token = create_jwt(user_id, &email, &state.jwt_secret)?;
    Ok(redirect_with_cookie(&next, &token))
}

async fn logout() -> Response {
    let mut response = Redirect::to("/").into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_static("kroissant_token=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0"),
    );
    response
}

async fn account(State(state): State<AppState>, headers: HeaderMap) -> AppResult<Response> {
    let Some(user) = current_user(&state, &headers).await else {
        return Ok(Redirect::to("/connexion?next=/compte").into_response());
    };

    let saved = saved_contents(&state.pool, user.id).await?;
    let history = history_contents(&state.pool, user.id).await?;
    let body = format!(
        r#"
        <main class="account-shell">
            <section class="account-header">
                <p class="eyebrow">Compte parent</p>
                <h1>{}</h1>
                <p>Vos contenus sauvegardes et votre historique restent dans la base SQLite locale de dev.</p>
            </section>

            <section class="section-block">
                <div class="section-heading">
                    <h2>Sauvegardes</h2>
                    <p>{} contenu(x)</p>
                </div>
                <div class="card-grid two-cols">
                    {}
                </div>
            </section>

            <section class="section-block">
                <div class="section-heading">
                    <h2>Historique</h2>
                    <p>{} contenu(x) lance(s)</p>
                </div>
                <div class="card-grid two-cols">
                    {}
                </div>
            </section>
        </main>
        "#,
        h(&user.email),
        saved.len(),
        render_cards_or_empty(&saved, "Aucun contenu sauvegarde pour l'instant."),
        history.len(),
        render_cards_or_empty(&history, "Aucun contenu lance pour l'instant."),
    );

    Ok(Html(render_page(
        "Compte - Kroissant",
        "Accueil",
        Some("/"),
        &Some(user),
        body,
    ))
    .into_response())
}

async fn get_contents(
    pool: &SqlitePool,
    platform: Option<&str>,
    limit: Option<i64>,
) -> Result<Vec<Content>> {
    let limit = limit.unwrap_or(100);
    let rows = match platform {
        Some(platform) => {
            sqlx::query_as::<_, Content>(
                "SELECT * FROM contents WHERE platform = ? ORDER BY sort_order LIMIT ?",
            )
            .bind(platform)
            .bind(limit)
            .fetch_all(pool)
            .await?
        }
        None => {
            sqlx::query_as::<_, Content>("SELECT * FROM contents ORDER BY sort_order LIMIT ?")
                .bind(limit)
                .fetch_all(pool)
                .await?
        }
    };
    Ok(rows)
}

async fn get_content_by_slug(pool: &SqlitePool, slug: &str) -> Result<Option<Content>> {
    Ok(
        sqlx::query_as::<_, Content>("SELECT * FROM contents WHERE slug = ?")
            .bind(slug)
            .fetch_optional(pool)
            .await?,
    )
}

async fn get_content_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Content>> {
    Ok(
        sqlx::query_as::<_, Content>("SELECT * FROM contents WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?,
    )
}

async fn get_similar_contents(pool: &SqlitePool, content: &Content) -> Result<Vec<Content>> {
    let mut rows = sqlx::query_as::<_, Content>(
        r#"
        SELECT * FROM contents
        WHERE id != ? AND skill = ?
        ORDER BY sort_order
        LIMIT 2
        "#,
    )
    .bind(content.id)
    .bind(&content.skill)
    .fetch_all(pool)
    .await?;

    if rows.len() < 2 {
        let fallback = sqlx::query_as::<_, Content>(
            r#"
            SELECT * FROM contents
            WHERE id != ?
            ORDER BY sort_order
            LIMIT ?
            "#,
        )
        .bind(content.id)
        .bind(2_i64 - rows.len() as i64)
        .fetch_all(pool)
        .await?;
        rows.extend(fallback);
    }

    Ok(rows)
}

async fn is_saved(pool: &SqlitePool, user_id: i64, content_id: i64) -> Result<bool> {
    let exists: Option<(i64,)> =
        sqlx::query_as("SELECT 1 FROM saved_items WHERE user_id = ? AND content_id = ?")
            .bind(user_id)
            .bind(content_id)
            .fetch_optional(pool)
            .await?;
    Ok(exists.is_some())
}

async fn saved_contents(pool: &SqlitePool, user_id: i64) -> Result<Vec<Content>> {
    Ok(sqlx::query_as::<_, Content>(
        r#"
        SELECT c.* FROM contents c
        INNER JOIN saved_items s ON s.content_id = c.id
        WHERE s.user_id = ?
        ORDER BY s.created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?)
}

async fn history_contents(pool: &SqlitePool, user_id: i64) -> Result<Vec<Content>> {
    Ok(sqlx::query_as::<_, Content>(
        r#"
        SELECT c.* FROM contents c
        INNER JOIN watch_history h ON h.content_id = c.id
        WHERE h.user_id = ?
        GROUP BY c.id
        ORDER BY MAX(h.watched_at) DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?)
}

async fn current_user(state: &AppState, headers: &HeaderMap) -> Option<User> {
    let token = cookie_value(headers, AUTH_COOKIE)?;
    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .ok()?
    .claims;

    sqlx::query_as::<_, User>("SELECT id, email FROM users WHERE id = ?")
        .bind(claims.sub)
        .fetch_optional(&state.pool)
        .await
        .ok()
        .flatten()
}

fn cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    let cookies = headers.get_all(header::COOKIE);
    for value in cookies {
        let Ok(raw) = value.to_str() else {
            continue;
        };
        for cookie in raw.split(';') {
            let mut parts = cookie.trim().splitn(2, '=');
            let Some(cookie_name) = parts.next() else {
                continue;
            };
            let Some(cookie_value) = parts.next() else {
                continue;
            };
            if cookie_name == name {
                return Some(cookie_value.to_string());
            }
        }
    }
    None
}

fn create_jwt(user_id: i64, email: &str, secret: &str) -> Result<String> {
    let exp = Utc::now() + Duration::days(7);
    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        exp: exp.timestamp() as usize,
    };
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|error| anyhow::anyhow!("hash password: {error}"))?
        .to_string())
}

fn verify_password(password: &str, password_hash: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(password_hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

fn redirect_with_cookie(location: &str, token: &str) -> Response {
    let mut response = Redirect::to(location).into_response();
    let cookie = format!("{AUTH_COOKIE}={token}; HttpOnly; SameSite=Lax; Path=/; Max-AAge=604800")
        .replace("Max-AAge", "Max-Age");
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie).expect("valid set-cookie header"),
    );
    response
}

fn clean_next(next: Option<String>) -> String {
    next.filter(|value| value.starts_with('/') && !value.starts_with("//") && !value.contains('\n'))
        .unwrap_or_else(|| "/bibliotheque".to_string())
}

fn normalize_platform(platform: Option<&str>) -> &str {
    match platform {
        Some("netflix") => "netflix",
        Some("disney") => "disney",
        _ => "youtube",
    }
}

fn normalize_platform_or_all(platform: Option<&str>) -> &str {
    match platform {
        Some("youtube") => "youtube",
        Some("netflix") => "netflix",
        Some("disney") => "disney",
        _ => "all",
    }
}

fn platform_label(platform: &str) -> &'static str {
    match platform {
        "youtube" => "YouTube",
        "netflix" => "Netflix",
        "disney" => "Disney+",
        _ => "Tous",
    }
}

fn benefits() -> Vec<Benefit> {
    vec![
        Benefit {
            key: "resilience",
            label: "Resilience",
            summary: "Apprendre a surmonter les obstacles en maintenant l'effort.",
            detail: "Capacite a surmonter les obstacles. Modelisee par les personnages via l'echec, la perseverance et la recuperation emotionnelle.",
            source: "Analyse 160 episodes Bluey - Tandfonline, 2025",
        },
        Benefit {
            key: "empathie",
            label: "Empathie",
            summary: "Comprendre le point de vue de l'autre dans des situations simples.",
            detail: "Comprehension du point de vue de l'autre. Les contenus mettent en scene des situations ou le personnage doit se mettre a la place d'autrui.",
            source: "Pritchard et al., 2024 - Impact of cartoons on childhood development",
        },
        Benefit {
            key: "language",
            label: "Developpement du langage",
            summary: "Nommer, raconter et reformuler ce qui vient d'etre vu.",
            detail: "Vocabulaire, narration, comprehension orale. Les programmes educatifs favorisent activement ces dimensions.",
            source: "Cohorte EFe / INSERM 2023 - Etude longitudinale sur 14 000 enfants francais",
        },
        Benefit {
            key: "regulation",
            label: "Regulation emotionnelle",
            summary: "Reconnaître ses emotions et choisir une reponse adaptee.",
            detail: "Nommer, reconnaître et gerer ses emotions. Reduit la reactivite emotionnelle a long terme.",
            source: "JAMA Pediatrics - Radesky et al., 2023",
        },
        Benefit {
            key: "creativite",
            label: "Creativite",
            summary: "Explorer plusieurs idees sans chercher une seule bonne reponse.",
            detail: "Les recits ouverts encouragent les enfants a inventer, comparer et tester des variantes.",
            source: "LeadingTree 2023 - Early creative learning review",
        },
        Benefit {
            key: "science",
            label: "Science",
            summary: "Observer, poser une question et verifier une hypothese simple.",
            detail: "Les contenus scientifiques de qualite structurent la curiosite: observation, prediction, verification et vocabulaire precis.",
            source: "Arcom 2024 - Jeunesse et contenus educatifs",
        },
    ]
}

fn benefit_for(key: &str) -> Benefit {
    benefits()
        .into_iter()
        .find(|benefit| benefit.key == key)
        .unwrap_or(Benefit {
            key: "resilience",
            label: "Resilience",
            summary: "Apprendre a surmonter les obstacles en maintenant l'effort.",
            detail: "Capacite a surmonter les obstacles.",
            source: "Analyse Kroissant",
        })
}

fn render_page(
    title: &str,
    left_label: &str,
    back_href: Option<&str>,
    user: &Option<User>,
    body: String,
) -> String {
    let left = match back_href {
        Some(href) => format!(
            r#"<a class="back-link" href="{}">← {}</a>"#,
            a(href),
            h(left_label)
        ),
        None => format!(r#"<span class="muted-nav">{}</span>"#, h(left_label)),
    };

    let nav = match user {
        Some(user) => format!(
            r#"
            <a class="nav-link" href="/compte">{}</a>
            <form method="post" action="/deconnexion"><button class="nav-link ghost-button" type="submit">Deconnexion</button></form>
            "#,
            h(&user.email)
        ),
        None => r#"
            <a class="nav-link" href="/connexion">Connexion</a>
            <a class="nav-button" href="/inscription">S'inscrire</a>
        "#
        .to_string(),
    };

    format!(
        r#"<!doctype html>
        <html lang="fr">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>{}</title>
            <link rel="stylesheet" href="/static/app.css">
            <script src="/static/htmx.min.js" defer></script>
        </head>
        <body>
            <header class="topbar">
                <div class="topbar-left">{}</div>
                <a class="brand" href="/">Kroissant</a>
                <nav class="topbar-nav">{}</nav>
            </header>
            {}
        </body>
        </html>"#,
        h(title),
        left,
        nav,
        body
    )
}

fn render_error_page(message: &str) -> String {
    format!(
        r#"<!doctype html>
        <html lang="fr">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>Kroissant</title>
            <link rel="stylesheet" href="/static/app.css">
        </head>
        <body>
            <main class="centered-page">
                <section class="auth-card">
                    <h1>{}</h1>
                    <a class="button button-primary full-width" href="/">Retour a l'accueil</a>
                </section>
            </main>
        </body>
        </html>"#,
        h(message)
    )
}

fn render_platform_tabs(active: &str, context: &str) -> String {
    let target = if context == "library" {
        "#library-section"
    } else if context == "home" {
        "#home-platform"
    } else {
        "#platform-results"
    };
    let route = if context == "library" {
        "/partials/library"
    } else {
        "/partials/home"
    };
    let swap = if context == "home" {
        "outerHTML"
    } else {
        "innerHTML"
    };

    let mut tabs = String::from(r#"<div class="platform-tabs">"#);
    for (key, label) in [
        ("youtube", "YouTube"),
        ("netflix", "Netflix"),
        ("disney", "Disney+"),
    ] {
        tabs.push_str(&format!(
            r#"
            <button
                class="platform-tab {}"
                hx-get="{}?platform={}"
                hx-target="{}"
                hx-swap="{}"
                {}
                type="button">{}</button>
            "#,
            if active == key { "active" } else { "" },
            route,
            key,
            target,
            swap,
            if context == "library" {
                format!(r#"hx-push-url="/bibliotheque?platform={}""#, key)
            } else {
                String::new()
            },
            label
        ));
    }
    tabs.push_str("</div>");
    tabs
}

fn render_home_platform_section(active: &str, contents: &[Content]) -> String {
    format!(
        r#"
        <div id="home-platform">
            {}
            <div id="platform-results">
                {}
            </div>
        </div>
        "#,
        render_platform_tabs(active, "home"),
        render_cards(contents)
    )
}

fn render_library_tabs(active: &str) -> String {
    let mut tabs = String::from(r#"<div class="platform-tabs library-tabs">"#);
    for (key, label) in [
        ("youtube", "YouTube"),
        ("netflix", "Netflix"),
        ("disney", "Disney+"),
        ("all", "Tous"),
    ] {
        tabs.push_str(&format!(
            r##"
            <button
                class="platform-tab {}"
                hx-get="/partials/library?platform={}"
                hx-target="#library-section"
                hx-swap="innerHTML"
                hx-push-url="/bibliotheque?platform={}"
                type="button">{}</button>
            "##,
            if active == key { "active" } else { "" },
            key,
            key,
            label
        ));
    }
    tabs.push_str("</div>");
    tabs
}

fn render_library_section(active: &str, contents: &[Content]) -> String {
    format!(
        r#"
        {}
        <div class="section-heading library-title">
            <h1>{} contenus selectionnes - {}</h1>
            <p>Chaque contenu developpe une competence reelle chez votre enfant</p>
        </div>
        <div id="library-results" class="card-grid library-grid">
            {}
        </div>
        "#,
        render_library_tabs(active),
        contents.len(),
        h(platform_label(active)),
        render_cards(contents)
    )
}

fn render_cards(contents: &[Content]) -> String {
    contents
        .iter()
        .map(render_content_card)
        .collect::<Vec<_>>()
        .join("")
}

fn render_cards_or_empty(contents: &[Content], empty: &str) -> String {
    if contents.is_empty() {
        format!(r#"<p class="empty-state">{}</p>"#, h(empty))
    } else {
        render_cards(contents)
    }
}

fn render_content_card(content: &Content) -> String {
    let benefit = benefit_for(&content.skill);
    format!(
        r#"
        <article class="content-card">
            <a class="thumb-link" href="/contenu/{}">
                <img src="{}" alt="">
                <span class="pill floating-pill {}">{}</span>
            </a>
            <div class="card-body">
                <h3><a href="/contenu/{}">{}</a></h3>
                <p>{} · {}</p>
                <a class="button button-primary card-watch" href="/go/{}">Regarder sur {}</a>
                <a class="develop-link" href="/science">→ Developpe : {}</a>
            </div>
        </article>
        "#,
        a(&content.slug),
        a(&content.image_url),
        h(benefit.key),
        h(benefit.label),
        a(&content.slug),
        h(&content.title),
        h(&content.duration),
        h(&content.age_range),
        a(&content.slug),
        h(content.platform_label()),
        h(benefit.label)
    )
}

fn render_save_panel(content: &Content, user: Option<&User>, saved: bool) -> String {
    match user {
        Some(_) => format!(
            r##"
            <form id="save-panel" class="save-panel" hx-post="/contenu/{}/save" hx-target="#save-panel" hx-swap="outerHTML">
                <button class="button {}" type="submit">{}</button>
                <p>{}</p>
            </form>
            "##,
            content.id,
            if saved {
                "button-saved"
            } else {
                "button-secondary"
            },
            if saved { "Sauvegarde" } else { "Sauvegarder" },
            if saved {
                "Ce contenu est dans votre compte parent."
            } else {
                "Ajoutez ce contenu a vos favoris."
            }
        ),
        None => format!(
            r#"
            <div id="save-panel" class="save-panel">
                <a class="button button-secondary" href="/inscription?next=/contenu/{}">Sauvegarder</a>
                <p><a href="/inscription?next=/contenu/{}">Creez votre compte</a> pour retrouver vos favoris a chaque visite.</p>
            </div>
            "#,
            a(&content.slug),
            a(&content.slug)
        ),
    }
}

#[derive(Copy, Clone)]
enum AuthMode {
    Register,
    Login,
}

fn render_auth_page(
    mode: AuthMode,
    next: Option<String>,
    error: Option<&str>,
    user: Option<&User>,
) -> String {
    if user.is_some() {
        return render_page(
            "Kroissant",
            "Accueil",
            Some("/"),
            &user.cloned(),
            r#"<main class="centered-page"><section class="auth-card"><h1>Vous etes deja connecte.</h1><a class="button button-primary full-width" href="/compte">Ouvrir mon compte</a></section></main>"#.to_string(),
        );
    }

    let (title, subtitle, action, primary, secondary_href, secondary_text) = match mode {
        AuthMode::Register => (
            "Creez votre compte gratuit",
            "Sauvegardez vos contenus et retrouvez-les a chaque visite.",
            "/inscription",
            "Creer mon compte - gratuit",
            "/connexion",
            "Se connecter",
        ),
        AuthMode::Login => (
            "Connectez-vous",
            "Reprenez la ou vous vous etiez arrete.",
            "/connexion",
            "Se connecter",
            "/inscription",
            "Creer un compte",
        ),
    };

    let next_value = next.unwrap_or_else(|| "/bibliotheque".to_string());
    let secondary = format!("{}?next={}", secondary_href, a(&next_value));
    let error_html = error
        .map(|message| format!(r#"<p class="form-error">{}</p>"#, h(message)))
        .unwrap_or_default();

    let body = format!(
        r#"
        <main class="centered-page">
            <section class="auth-card">
                <h1>{}</h1>
                <p>{}</p>
                {}
                <form method="post" action="{}" class="auth-form">
                    <input type="hidden" name="next" value="{}">
                    <label>Adresse email<input name="email" type="email" placeholder="votre@email.com" autocomplete="email" required></label>
                    <label>Mot de passe<input name="password" type="password" placeholder="••••••••" autocomplete="current-password" minlength="8" required></label>
                    <button class="button button-primary full-width" type="submit">{}</button>
                </form>
                <div class="divider">ou</div>
                <a class="button button-secondary full-width" href="{}">{}</a>
                <small>Gratuit · Aucune carte requise · Donnees securisees</small>
            </section>
        </main>
        "#,
        h(title),
        h(subtitle),
        error_html,
        action,
        a(&next_value),
        primary,
        secondary,
        secondary_text,
    );

    render_page(
        title,
        "Fiche contenu",
        Some(&next_value),
        &None::<User>,
        body,
    )
}

fn h(value: &str) -> String {
    encode_text(value).into_owned()
}

fn a(value: &str) -> String {
    encode_double_quoted_attribute(value).into_owned()
}
