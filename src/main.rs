//! Kroissant - Curation de contenus développementaux pour enfants.
use anyhow::{Context, Result};
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::{env, net::SocketAddr, str::FromStr};
use tokio::{fs, net::TcpListener};
use std::sync::Arc;

use kroissant::{AppState, routes};
use kroissant::repositories::{SqliteContentRepository, SqliteUserRepository};
use kroissant::services::{AuthServiceImpl, ContentServiceImpl};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialisation du logging
    tracing_subscriber::fmt()
        .with_env_filter(
            env::var("RUST_LOG").unwrap_or_else(|_| "tower_http=debug,kroissant=debug".to_string()),
        )
        .init();

    // Création du dossier de données
    fs::create_dir_all("data")
        .await
        .context("creation du dossier data")?;

    // Connexion à la base de données
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

    // Migrations et Seed (pour le dev)
    migrate(&pool).await?;
    seed_fake_data(&pool).await?;

    // Initialisation des Repositories
    let content_repo = Arc::new(SqliteContentRepository::new(pool.clone()));
    let user_repo = Arc::new(SqliteUserRepository::new(pool.clone()));

    // Initialisation des Services
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-me-kroissant".to_string());
    let auth_service = Arc::new(AuthServiceImpl::new(user_repo.clone(), jwt_secret.clone()));
    let content_service = Arc::new(ContentServiceImpl::new(content_repo.clone(), user_repo.clone()));

    // État global
    let state = AppState::new(
        pool,
        jwt_secret,
        content_repo,
        user_repo,
        auth_service,
        content_service,
    );

    // Configuration du routeur
    let app = routes::create_router(state);

    // Démarrage du serveur
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
        r#"
        CREATE TABLE IF NOT EXISTS tmdb_series (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tmdb_id INTEGER NOT NULL UNIQUE,
            name TEXT NOT NULL,
            original_name TEXT NOT NULL DEFAULT '',
            overview TEXT NOT NULL DEFAULT '',
            first_air_date TEXT,
            poster_path TEXT,
            platform TEXT NOT NULL DEFAULT '',
            age_range TEXT NOT NULL DEFAULT '',
            episode_context_count INTEGER NOT NULL DEFAULT 0,
            llm_reason TEXT NOT NULL DEFAULT '',
            confidence REAL,
            source_url TEXT NOT NULL DEFAULT '',
            updated_at TEXT NOT NULL
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS tmdb_episodes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            series_id INTEGER NOT NULL,
            tmdb_episode_id INTEGER,
            season_number INTEGER NOT NULL,
            episode_number INTEGER NOT NULL,
            title TEXT NOT NULL,
            overview TEXT NOT NULL DEFAULT '',
            air_date TEXT,
            runtime INTEGER,
            still_path TEXT,
            updated_at TEXT NOT NULL,
            UNIQUE(series_id, season_number, episode_number),
            FOREIGN KEY (series_id) REFERENCES tmdb_series(id) ON DELETE CASCADE
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS tmdb_series_tags (
            series_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (series_id, tag_id),
            FOREIGN KEY (series_id) REFERENCES tmdb_series(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
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
