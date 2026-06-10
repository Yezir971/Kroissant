//! État partagé de l'application.
use std::sync::Arc;
use sqlx::SqlitePool;
use crate::repositories::content::ContentRepository;
use crate::repositories::user::UserRepository;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub jwt_secret: String,
    pub content_repo: Arc<dyn ContentRepository>,
    pub user_repo: Arc<dyn UserRepository>,
}
