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

use uuid::Uuid;
use serde::Deserialize;
use chrono::{Duration, Utc};
use crate::models::Claims;
use crate::auth::jwt::create_token;

pub const GOOGLE_STATE_COOKIE: &str = "google_oauth_state";

#[derive(Debug, Deserialize)]
pub struct GoogleCallbackQuery {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct GoogleUserInfo {
    email: String,
    name: String,
}

/// Callback de Google OAuth2.
pub async fn google_callback(
    State(state): State<AppState>,
    headers: header::HeaderMap,
    Query(query): Query<GoogleCallbackQuery>,
) -> AppResult<Response> {
    // 1. Vérifier le state CSRF
    let cookie_state = get_cookie_value(&headers, GOOGLE_STATE_COOKIE)
        .ok_or_else(|| AppError::Auth("Cookie de session OAuth manquant".to_string()))?;

    if cookie_state != query.state {
        return Err(AppError::Auth("Erreur de sécurité CSRF : state mismatch".to_string()));
    }

    // 2. Échanger le code contre un token
    let client = reqwest::Client::new();
    let params = [
        ("code", query.code.as_str()),
        ("client_id", state.config.google_client_id.as_str()),
        ("client_secret", state.config.google_client_secret.as_str()),
        ("redirect_uri", state.config.google_redirect_uri.as_str()),
        ("grant_type", "authorization_code"),
    ];

    let token_res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Erreur échange token Google: {}", e)))?
        .json::<GoogleTokenResponse>()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Erreur parsing token Google: {}", e)))?;

    // 3. Récupérer les infos utilisateur
    let user_info = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token_res.access_token)
        .send()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Erreur récup infos Google: {}", e)))?
        .json::<GoogleUserInfo>()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Erreur parsing infos Google: {}", e)))?;

    // 4. Logique métier : Login ou Création
    let email = user_info.email.to_lowercase();
    
    let user_id = if let Some((id, _)) = state.user_repo.get_by_email(&email).await? {
        id
    } else {
        // Nouvel utilisateur via Google
        state.user_repo.create_user(&email, "OAUTH_EXTERNAL_USER", &user_info.name).await?
    };

    // 5. Générer JWT
    let claims = Claims {
        sub: user_id,
        email: email.clone(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };
    let token = create_token(&claims, &state.jwt_secret)?;

    // 6. Préparer la réponse
    let mut response = Redirect::to("/bibliotheque").into_response();
    
    // Poser le cookie d'auth
    let auth_cookie = format!("{AUTH_COOKIE}={token}; HttpOnly; SameSite=Lax; Path=/; Max-Age=604800");
    if let Ok(value) = HeaderValue::from_str(&auth_cookie) {
        response.headers_mut().insert(header::SET_COOKIE, value);
    }

    // Supprimer le cookie state
    let clear_state_cookie = format!("{GOOGLE_STATE_COOKIE}=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0");
    if let Ok(value) = HeaderValue::from_str(&clear_state_cookie) {
        response.headers_mut().append(header::SET_COOKIE, value);
    }

    Ok(response)
}

fn get_cookie_value(headers: &header::HeaderMap, name: &str) -> Option<String> {
    headers.get(header::COOKIE)?
        .to_str().ok()?
        .split(';')
        .find_map(|c| {
            let mut parts = c.trim().splitn(2, '=');
            let key = parts.next()?;
            let val = parts.next()?;
            if key == name { Some(val.to_string()) } else { None }
        })
}

/// Redirige vers Google pour l'authentification.
pub async fn google_auth(State(state): State<AppState>) -> AppResult<Response> {
    let state_token = Uuid::new_v4().to_string();

    // Construction manuelle de l'URL Google OAuth2
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=email%20profile&state={}",
        state.config.google_client_id,
        urlencoding::encode(&state.config.google_redirect_uri),
        state_token
    );

    let mut response = Redirect::to(&auth_url).into_response();
    
    // Cookie d'état (CSRF) valide 5 minutes
    let cookie = format!("{GOOGLE_STATE_COOKIE}={state_token}; HttpOnly; SameSite=Lax; Path=/; Max-Age=300");
    if let Ok(value) = HeaderValue::from_str(&cookie) {
        response.headers_mut().insert(header::SET_COOKIE, value);
    }

    Ok(response)
}

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
