//! État partagé de l'application.
use crate::repositories::content::ContentRepository;
use crate::repositories::user::UserRepository;
use crate::services::auth::AuthService;
use crate::services::content::ContentService;
use axum::extract::FromRef;
use sqlx::SqlitePool;
use std::sync::Arc;

/// État global de l'application Kroissant.
#[derive(Clone)]
pub struct AppState {
    /// Pool de connexion SQLite.
    pub db: SqlitePool,
    /// Secret utilisé pour signer les tokens JWT.
    pub jwt_secret: String,
    /// Repository pour l'accès aux contenus.
    pub content_repo: Arc<dyn ContentRepository>,
    /// Repository pour l'accès aux utilisateurs.
    pub user_repo: Arc<dyn UserRepository>,
    /// Service d'authentification.
    pub auth_service: Arc<dyn AuthService>,
    /// Service pour les contenus.
    pub content_service: Arc<dyn ContentService>,
}

impl AppState {
    /// Crée une nouvelle instance de l'état de l'application.
    pub fn new(
        db: SqlitePool,
        jwt_secret: String,
        content_repo: Arc<dyn ContentRepository>,
        user_repo: Arc<dyn UserRepository>,
        auth_service: Arc<dyn AuthService>,
        content_service: Arc<dyn ContentService>,
    ) -> Self {
        Self {
            db,
            jwt_secret,
            content_repo,
            user_repo,
            auth_service,
            content_service,
        }
    }
}

// Implémentations de FromRef pour permettre l'extraction granulaire via State<T>
// Note: Nécessite que T implémente Clone. Les Repositories/Services sont sous Arc donc Clone est OK.

impl FromRef<AppState> for SqlitePool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for Arc<dyn ContentRepository> {
    fn from_ref(state: &AppState) -> Self {
        state.content_repo.clone()
    }
}

impl FromRef<AppState> for Arc<dyn UserRepository> {
    fn from_ref(state: &AppState) -> Self {
        state.user_repo.clone()
    }
}

impl FromRef<AppState> for Arc<dyn AuthService> {
    fn from_ref(state: &AppState) -> Self {
        state.auth_service.clone()
    }
}

impl FromRef<AppState> for Arc<dyn ContentService> {
    fn from_ref(state: &AppState) -> Self {
        state.content_service.clone()
    }
}
