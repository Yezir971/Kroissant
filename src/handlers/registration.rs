//! Handlers pour le parcours d'inscription multi-étapes.
use crate::app_state::AppState;
use crate::auth::password::hash_password;
use crate::auth::{AUTH_COOKIE, AuthUser, jwt::create_token};
use crate::error::AppResult;
use crate::handlers::auth::REGISTRATION_COOKIE;
use crate::models::{Claims, RegistrationStep, VerifyQuery};
use crate::views;
use axum::{
    extract::{Form, Path, Query, State},
    http::{HeaderValue, StatusCode, header},
    response::{Html, IntoResponse, Redirect, Response},
};
use chrono::{Duration, Utc};
use std::str::FromStr;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegistrationData {
    pub email: String,
    pub password_hash: Option<String>,
}

/// Page shell complète pour l'inscription.
pub async fn registration_shell(
    AuthUser(user): AuthUser,
    State(_state): State<AppState>,
) -> AppResult<Html<String>> {
    let initial_content = views::registration::step_entry::render();

    let body = format!(
        r#"
        <main class="centered-page">
            <section class="auth-card" style="padding: 0; overflow: hidden;">
                <div id="registration-container"
                     hx-swap="innerHTML">
                     {}
                </div>
            </section>
        </main>
        "#,
        initial_content
    );

    Ok(Html(views::render_page(
        "Inscription — Kroissant",
        "Accueil",
        Some("/"),
        &user,
        body,
    )))
}

/// Handler générique pour le GET d'une étape (via route dynamique).
pub async fn get_step(
    headers: axum::http::HeaderMap,
    Path(step_name): Path<String>,
    state: State<AppState>,
) -> AppResult<Response> {
    get_step_internal(&step_name, headers, state).await
}

/// Handlers spécifiques pour les routes statiques (évite les conflits 405).
pub async fn get_step_email(
    headers: axum::http::HeaderMap,
    state: State<AppState>,
) -> AppResult<Response> {
    get_step_internal("email", headers, state).await
}

pub async fn get_step_password(
    headers: axum::http::HeaderMap,
    state: State<AppState>,
) -> AppResult<Response> {
    get_step_internal("password", headers, state).await
}

pub async fn get_step_profile(
    headers: axum::http::HeaderMap,
    state: State<AppState>,
) -> AppResult<Response> {
    get_step_internal("profile", headers, state).await
}

/// Logique commune de rendu des étapes.
async fn get_step_internal(
    step_name: &str,
    headers: axum::http::HeaderMap,
    State(state): State<AppState>,
) -> AppResult<Response> {
    tracing::debug!("Rendu de l'étape: {}", step_name);

    let Ok(step) = RegistrationStep::from_str(step_name) else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };

    let partial_html = match step {
        RegistrationStep::Entry => views::registration::step_entry::render(),
        RegistrationStep::Email => views::registration::step_email::render(None),
        RegistrationStep::EmailSent => {
            let email = get_cookie(&headers, REGISTRATION_COOKIE)
                .and_then(|c| serde_json::from_str::<RegistrationData>(&c).ok())
                .map(|d| d.email)
                .unwrap_or_else(|| "votre e-mail".to_string());
            views::registration::step_email_sent::render(&email)
        }
        RegistrationStep::Password => views::registration::step_password::render(None),
        RegistrationStep::Profile => views::registration::step_profile::render(None),
        _ => format!("<div>Étape : {} (Bientôt disponible)</div>", step),
    };

    // Si la requête NE vient PAS de HTMX, on enveloppe dans le shell complet
    if headers.get("HX-Request").is_none() {
        let body = format!(
            r#"
            <main class="centered-page">
                <section class="auth-card" style="padding: 0; overflow: hidden;">
                    <div id="registration-container" hx-swap="innerHTML">
                         {}
                    </div>
                </section>
            </main>
            "#,
            partial_html
        );

        let user = crate::auth::AuthUser(None); // On est en phase d'inscription
        Ok(Html(views::render_page(
            "Inscription — Kroissant",
            "Accueil",
            Some("/"),
            &user.0,
            body,
        ))
        .into_response())
    } else {
        // Sinon on renvoie juste le fragment
        Ok(Html(partial_html).into_response())
    }
}

#[derive(serde::Deserialize)]
pub struct EmailForm {
    pub email: String,
}

/// Action POST pour l'étape Email.
pub async fn post_email(
    State(state): State<AppState>,
    Form(form): Form<EmailForm>,
) -> AppResult<Response> {
    let email = form.email.trim().to_lowercase();

    if !email.contains('@') || !email.contains('.') {
        return Ok(Html(views::registration::step_email::render(Some(
            "Format d'adresse invalide.",
        )))
        .into_response());
    }

    if state.user_repo.get_by_email(&email).await?.is_some() {
        return Ok(Html(views::registration::step_email::render(Some(
            "Cette adresse est déjà utilisée.",
        )))
        .into_response());
    }

    let data = RegistrationData {
        email: email.clone(),
        password_hash: None,
    };
    let cookie_value = serde_json::to_string(&data).unwrap_or_default();

    state.email_service.send_verification_email(&email).await?;

    let mut response = Html(views::registration::step_email_sent::render(&email)).into_response();
    let cookie = format!(
        "{REGISTRATION_COOKIE}={}; HttpOnly; SameSite=Lax; Path=/; Max-Age=3600",
        cookie_value
    );
    if let Ok(value) = HeaderValue::from_str(&cookie) {
        response.headers_mut().insert(header::SET_COOKIE, value);
    }
    Ok(response)
}

#[derive(serde::Deserialize)]
pub struct PasswordForm {
    pub password: String,
    pub password_confirm: String,
}

/// Action POST pour l'étape Password.
pub async fn post_password(
    headers: axum::http::HeaderMap,
    Form(form): Form<PasswordForm>,
) -> AppResult<Response> {
    let Some(cookie_str) = get_cookie(&headers, REGISTRATION_COOKIE) else {
        return Ok(Html(views::render_error_page(
            "Session d'inscription expirée. Veuillez recommencer.",
        ))
        .into_response());
    };

    let Ok(mut data) = serde_json::from_str::<RegistrationData>(&cookie_str) else {
        return Ok(Html(views::render_error_page(
            "Données d'inscription corrompues. Veuillez recommencer.",
        ))
        .into_response());
    };

    if form.password.len() < 8 {
        return Ok(Html(views::registration::step_password::render(Some(
            "Le mot de passe doit faire au moins 8 caractères.",
        )))
        .into_response());
    }

    if form.password != form.password_confirm {
        return Ok(Html(views::registration::step_password::render(Some(
            "Les mots de passe ne correspondent pas.",
        )))
        .into_response());
    }

    let hash = hash_password(&form.password)?;
    data.password_hash = Some(hash);

    let updated_cookie = serde_json::to_string(&data).unwrap_or_default();

    let mut response = Html(views::registration::step_profile::render(None)).into_response();
    let cookie_header = format!(
        "{REGISTRATION_COOKIE}={}; HttpOnly; SameSite=Lax; Path=/; Max-Age=3600",
        updated_cookie
    );
    if let Ok(value) = HeaderValue::from_str(&cookie_header) {
        response.headers_mut().insert(header::SET_COOKIE, value);
    }

    Ok(response)
}

#[derive(serde::Deserialize)]
pub struct ProfileForm {
    pub pseudo: String,
}

/// Action POST pour l'étape Profile.
pub async fn post_profile(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Form(form): Form<ProfileForm>,
) -> AppResult<Response> {
    let Some(cookie_str) = get_cookie(&headers, REGISTRATION_COOKIE) else {
        return Ok(Html(views::render_error_page(
            "Session d'inscription expirée. Veuillez recommencer.",
        ))
        .into_response());
    };

    let Ok(data) = serde_json::from_str::<RegistrationData>(&cookie_str) else {
        return Ok(Html(views::render_error_page(
            "Données d'inscription corrompues. Veuillez recommencer.",
        ))
        .into_response());
    };

    let Some(password_hash) = data.password_hash else {
        return Ok(Html(views::render_error_page(
            "Mot de passe manquant. Veuillez recommencer.",
        ))
        .into_response());
    };

    let pseudo = form.pseudo.trim();
    if pseudo.len() < 2 || pseudo.len() > 30 {
        return Ok(Html(views::registration::step_profile::render(Some(
            "Le pseudo doit faire entre 2 et 30 caractères.",
        )))
        .into_response());
    }

    let user_id = state
        .user_repo
        .create_user(&data.email, &password_hash, pseudo)
        .await?;

    let claims = Claims {
        sub: user_id,
        email: data.email.clone(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };
    let token = create_token(&claims, &state.jwt_secret)?;

    let mut response = Html(views::registration::step_success::render(pseudo)).into_response();

    let auth_cookie =
        format!("{AUTH_COOKIE}={token}; HttpOnly; SameSite=Lax; Path=/; Max-Age=604800");
    if let Ok(value) = HeaderValue::from_str(&auth_cookie) {
        response.headers_mut().insert(header::SET_COOKIE, value);
    }

    let clear_reg_cookie =
        format!("{REGISTRATION_COOKIE}=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0");
    if let Ok(value) = HeaderValue::from_str(&clear_reg_cookie) {
        response.headers_mut().append(header::SET_COOKIE, value);
    }

    Ok(response)
}

/// Action de vérification d'email.
pub async fn verify_email(
    State(state): State<AppState>,
    Query(query): Query<VerifyQuery>,
) -> AppResult<Response> {
    match state.email_verification_repo.find_valid_token(&query.token).await? {
        Some(email) => {
            state.email_verification_repo.mark_token_used(&query.token).await?;

            let mut response = Redirect::to("/inscription/step/password").into_response();
            let data = RegistrationData { email, password_hash: None };
            let cookie_value = serde_json::to_string(&data).unwrap_or_default();
            let cookie = format!("{REGISTRATION_COOKIE}={}; HttpOnly; SameSite=Lax; Path=/; Max-Age=3600", cookie_value);
            if let Ok(value) = HeaderValue::from_str(&cookie) {
                response.headers_mut().insert(header::SET_COOKIE, value);
            }
            Ok(response)
        }
        None => {
            Ok((
                StatusCode::BAD_REQUEST,
                Html(views::render_error_page(
                    "Lien de vérification invalide ou expiré. Veuillez demander un nouveau lien d'inscription.",
                )),
            )
                .into_response())
        }
    }
}

fn get_cookie(headers: &axum::http::HeaderMap, name: &str) -> Option<String> {
    headers
        .get(header::COOKIE)?
        .to_str()
        .ok()?
        .split(';')
        .find_map(|c| {
            let mut parts = c.trim().splitn(2, '=');
            let key = parts.next()?;
            let val = parts.next()?;
            if key == name {
                Some(val.to_string())
            } else {
                None
            }
        })
}
