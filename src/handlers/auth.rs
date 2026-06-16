//! Handlers pour l'authentification et le compte utilisateur.
use axum::{
    extract::{Form, Query, State},
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
};
use crate::app_state::AppState;
use crate::error::{AppError, AppResult};
use crate::models::{AuthForm, AuthMode, AuthQuery, User};
use crate::auth::{AuthUser, AUTH_COOKIE};
use crate::utils::clean_next;
use crate::views;

pub const REGISTRATION_COOKIE: &str = "kroissant_registration";

/// Action d'inscription (dépréciée).
pub async fn register_deprecated() -> impl IntoResponse {
    (
        StatusCode::GONE,
        Html(views::render_error_page(
            "Cet endpoint d'inscription n'est plus disponible. Veuillez utiliser le nouveau parcours sur /inscription.",
        )),
    )
}

/// Page de connexion.
pub async fn login_page(
    AuthUser(user): AuthUser,
    State(_state): State<AppState>,
    Query(query): Query<AuthQuery>,
) -> AppResult<Html<String>> {
    Ok(Html(views::render_auth_page(
        AuthMode::Login,
        query.next,
        None,
        user.as_ref(),
    )))
}

/// Action de connexion.
pub async fn login(State(state): State<AppState>, Form(form): Form<AuthForm>) -> AppResult<Response> {
    let next = clean_next(form.next.clone());

    match state.auth_service.login(&form.email, &form.password).await {
        Ok(token) => Ok(redirect_with_cookie(&next, &token)),
        Err(AppError::Auth(msg)) => Ok((
            StatusCode::UNAUTHORIZED,
            Html(views::render_auth_page(
                AuthMode::Login,
                form.next,
                Some(&msg),
                None,
            )),
        )
            .into_response()),
        Err(e) => Err(e),
    }
}

/// Action de déconnexion.
pub async fn logout() -> Response {
    let mut response = Redirect::to("/").into_response();
    if let Ok(value) = HeaderValue::from_str(&format!(
        "{}=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0",
        AUTH_COOKIE
    )) {
        response.headers_mut().insert(header::SET_COOKIE, value);
    }
    response
}

/// Page de compte (authentification requise).
pub async fn account(user: User, State(state): State<AppState>) -> AppResult<Response> {
    let saved = state.user_repo.get_saved_contents(user.id).await?;
    let history = state.user_repo.get_history_contents(user.id).await?;
    Ok(Html(views::render_account(&user, &saved, &history)).into_response())
}

/// Utilitaire pour rediriger en posant un cookie d'auth.
fn redirect_with_cookie(location: &str, token: &str) -> Response {
    let mut response = Redirect::to(location).into_response();
    let cookie = format!("{AUTH_COOKIE}={token}; HttpOnly; SameSite=Lax; Path=/; Max-Age=604800");
    if let Ok(value) = HeaderValue::from_str(&cookie) {
        response.headers_mut().insert(header::SET_COOKIE, value);
    }
    response
}
