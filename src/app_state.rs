//! État partagé de l'application.
use std::sync::Arc;
use sqlx::SqlitePool;
use axum::extract::FromRef;
use crate::repositories::content::ContentRepository;
use crate::repositories::user::UserRepository;

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
}

impl AppState {
    /// Crée une nouvelle instance de l'état de l'application.
    pub fn new(
        db: SqlitePool,
        jwt_secret: String,
        content_repo: Arc<dyn ContentRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            db,
            jwt_secret,
            content_repo,
            user_repo,
        }
    }
}

// Implémentations de FromRef pour permettre l'extraction granulaire via State<T>
// Note: Nécessite que T implémente Clone. Les Repositories sont sous Arc donc Clone est OK.

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
