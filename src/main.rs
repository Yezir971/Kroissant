//! Kroissant - Curation de contenus développementaux pour enfants.
use anyhow::{Context, Result};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::{env, net::SocketAddr, str::FromStr};
use tokio::net::TcpListener;
use std::sync::Arc;

use kroissant::{AppState, routes, db};
use kroissant::repositories::{SqliteContentRepository, SqliteUserRepository};
use kroissant::services::{AuthServiceImpl, ContentServiceImpl};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialisation du logging
    tracing_subscriber::fmt()
        .with_env_filter(env::var("RUST_LOG").unwrap_or_else(|_| "tower_http=debug,kroissant=debug".to_string()))
        .init();

    // Connexion à la base de données
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data/kroissant.sqlite".to_string());
    let connect_options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .context("connexion sqlite")?;

    // Migrations et Seed (pour le dev)
    db::migrate(&pool).await?;
    db::seed_fake_data(&pool).await?;

    // Initialisation des Repositories
    let content_repo = Arc::new(SqliteContentRepository::new(pool.clone()));
    let user_repo = Arc::new(SqliteUserRepository::new(pool.clone()));

    // Initialisation des Services
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-me-kroissant".to_string());
    let auth_service = Arc::new(AuthServiceImpl::new(user_repo.clone(), jwt_secret.clone()));
    let content_service = Arc::new(ContentServiceImpl::new(content_repo.clone(), user_repo.clone()));

    // État global
    let state = AppState::new(pool, jwt_secret, content_repo, user_repo, auth_service, content_service);

    // Démarrage du serveur
    let addr = SocketAddr::from(([127, 0, 0, 1], env::var("PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(3000)));
    let listener = TcpListener::bind(addr).await?;
    println!("Kroissant dev server: http://{addr}");
    axum::serve(listener, routes::create_router(state)).await?;
    
    Ok(())
}
