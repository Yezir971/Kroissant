//! Handlers pour le parcours d'inscription multi-étapes.
use axum::{
    extract::{Path, State, Query, Form},
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
};
use crate::app_state::AppState;
use crate::error::AppResult;
use crate::models::{RegistrationStep, VerifyQuery};
use crate::auth::AuthUser;
use crate::views;
use crate::handlers::auth::REGISTRATION_COOKIE;
use std::str::FromStr;

/// Page shell complète pour l'inscription.
pub async fn registration_shell(
    AuthUser(user): AuthUser,
    State(_state): State<AppState>,
) -> AppResult<Html<String>> {
    let body = format!(
        r#"
        <main class="centered-page">
            <section class="auth-card" style="padding: 0; overflow: hidden;">
                <div id="registration-container"
                     hx-get="/inscription/step/entry"
                     hx-trigger="load"
                     hx-swap="innerHTML">
                     <div style="padding: 40px; text-align: center;">Chargement...</div>
                </div>
            </section>
        </main>
        "#
    );

    Ok(Html(views::render_page(
        "Inscription — Kroissant",
        "Accueil",
        Some("/"),
        &user,
        body,
    )))
}

/// Retourne le partial d'une étape d'inscription.
pub async fn get_step(
    Path(step): Path<String>,
    State(_state): State<AppState>,
) -> AppResult<Response> {
    let Ok(step) = RegistrationStep::from_str(&step) else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };

    let html = match step {
        RegistrationStep::Entry => views::registration::step_entry::render(),
        RegistrationStep::Email => views::registration::step_email::render(None),
        _ => format!("<div>Étape : {} (Bientôt disponible)</div>", step),
    };

    Ok(Html(html).into_response())
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

    // Valider le format (très basique)
    if !email.contains('@') || !email.contains('.') {
        return Ok(Html(views::registration::step_email::render(Some("Format d'adresse invalide."))).into_response());
    }

    // Vérifier l'unicité
    if state.user_repo.get_by_email(&email).await?.is_some() {
        return Ok(Html(views::registration::step_email::render(Some("Cette adresse est déjà utilisée."))).into_response());
    }

    // Envoyer l'email
    state.email_service.send_verification_email(&email).await?;

    Ok(Html(views::registration::step_email_sent::render(&email)).into_response())
}

/// Action POST pour l'étape Password.
pub async fn post_password(
    State(_state): State<AppState>,
) -> AppResult<Response> {
    // Implémentation au Prompt 5
    Ok(StatusCode::NOT_IMPLEMENTED.into_response())
}

/// Action POST pour l'étape Profile.
pub async fn post_profile(
    State(_state): State<AppState>,
) -> AppResult<Response> {
    // Implémentation au Prompt 6
    Ok(StatusCode::NOT_IMPLEMENTED.into_response())
}

/// Action de vérification d'email (anciennement dans handlers/auth.rs).
pub async fn verify_email(
    State(state): State<AppState>,
    Query(query): Query<VerifyQuery>,
) -> AppResult<Response> {
    match state.email_verification_repo.find_valid_token(&query.token).await? {
        Some(email) => {
            state.email_verification_repo.mark_token_used(&query.token).await?;

            let mut response = Redirect::to("/inscription/step/password").into_response();
            let cookie = format!("{REGISTRATION_COOKIE}={email}; HttpOnly; SameSite=Lax; Path=/; Max-Age=3600");
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
