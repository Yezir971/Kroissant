//! Middleware Axum pour l'authentification.
use crate::app_state::AppState;
use crate::auth::jwt::validate_token;
use crate::error::AppError;
use crate::models::User;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::header,
    http::request::Parts,
};

/// Constante pour le nom du cookie d'authentification.
pub const AUTH_COOKIE: &str = "kroissant_token";

impl<S> FromRequestParts<S> for User
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        let token =
            cookie_value(parts, AUTH_COOKIE).ok_or(AppError::Auth("Non connecté".to_string()))?;

        let claims = validate_token(&token, &state.jwt_secret)?;

        let user = state
            .user_repo
            .get_by_id(claims.sub)
            .await?
            .ok_or_else(|| AppError::Auth("Utilisateur introuvable".to_string()))?;

        Ok(user)
    }
}

/// Wrapper pour l'extraction optionnelle de l'utilisateur.
/// Si le token est absent ou invalide, l'option interne sera None.
pub struct AuthUser(pub Option<User>);

/// Implémentation pour l'extraction optionnelle de l'utilisateur.
impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        let Some(token) = cookie_value(parts, AUTH_COOKIE) else {
            return Ok(AuthUser(None));
        };

        let Ok(claims) = validate_token(&token, &state.jwt_secret) else {
            return Ok(AuthUser(None));
        };

        let user = state.user_repo.get_by_id(claims.sub).await.ok().flatten();
        Ok(AuthUser(user))
    }
}

/// Extrait la valeur d'un cookie à partir des headers de la requête.
fn cookie_value(parts: &Parts, name: &str) -> Option<String> {
    let cookies = parts.headers.get_all(header::COOKIE);
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
