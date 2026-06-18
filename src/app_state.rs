//! État partagé de l'application.
use crate::config::Config;
use crate::repositories::content::ContentRepository;
use crate::repositories::email_verification_repository::EmailVerificationRepository;
use crate::repositories::user::UserRepository;
use crate::services::auth::AuthService;
use crate::services::content::ContentService;
use crate::services::email::EmailService;
use axum::extract::FromRef;
use sqlx::SqlitePool;
use std::sync::Arc;

/// État global de l'application Kroissant.
#[derive(Clone)]
pub struct AppState {
    /// Pool de connexion SQLite.
    pub db: SqlitePool,
    /// Configuration de l'application.
    pub config: Config,
    /// Secret utilisé pour signer les tokens JWT.
    pub jwt_secret: String,
    /// Repository pour l'accès aux contenus.
    pub content_repo: Arc<dyn ContentRepository>,
    /// Repository pour l'accès aux utilisateurs.
    pub user_repo: Arc<dyn UserRepository>,
    /// Repository pour la vérification d'email.
    pub email_verification_repo: Arc<dyn EmailVerificationRepository>,
    /// Service d'authentification.
    pub auth_service: Arc<dyn AuthService>,
    /// Service pour les contenus.
    pub content_service: Arc<dyn ContentService>,
    /// Service d'email.
    pub email_service: Arc<dyn EmailService>,
}

impl AppState {
    /// Crée une nouvelle instance de l'état de l'application.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        db: SqlitePool,
        config: Config,
        jwt_secret: String,
        content_repo: Arc<dyn ContentRepository>,
        user_repo: Arc<dyn UserRepository>,
        email_verification_repo: Arc<dyn EmailVerificationRepository>,
        auth_service: Arc<dyn AuthService>,
        content_service: Arc<dyn ContentService>,
        email_service: Arc<dyn EmailService>,
    ) -> Self {
        Self {
            db,
            config,
            jwt_secret,
            content_repo,
            user_repo,
            email_verification_repo,
            auth_service,
            content_service,
            email_service,
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

impl FromRef<AppState> for Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
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

impl FromRef<AppState> for Arc<dyn EmailVerificationRepository> {
    fn from_ref(state: &AppState) -> Self {
        state.email_verification_repo.clone()
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

impl FromRef<AppState> for Arc<dyn EmailService> {
    fn from_ref(state: &AppState) -> Self {
        state.email_service.clone()
    }
}
