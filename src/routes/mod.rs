//! Configuration du routage et des middlewares.
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{services::ServeDir, trace::TraceLayer};
use crate::app_state::AppState;
use crate::handlers;

/// Crée le routeur principal de l'application.
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Accueil & Général
        .route("/", get(handlers::home::home))
        .route("/science", get(handlers::home::science))
        
        // Bibliothèque
        .route("/bibliotheque", get(handlers::library::library))
        
        // Contenus
        .route("/contenu/{slug}", get(handlers::content::content_detail))
        .route("/contenu/{id}/save", post(handlers::content::toggle_save))
        .route("/go/{slug}", get(handlers::content::go_to_source))
        
        // Authentification & Compte
        .route("/inscription", get(handlers::auth::register_page).post(handlers::auth::register))
        .route("/connexion", get(handlers::auth::login_page).post(handlers::auth::login))
        .route("/deconnexion", post(handlers::auth::logout))
        .route("/compte", get(handlers::auth::account))
        
        // Partiels HTMX
        .route("/partials/home", get(handlers::home::home_partial))
        .route("/partials/library", get(handlers::library::library_partial))
        
        // Fichiers statiques
        .nest_service("/static", ServeDir::new("static"))
        
        // Middlewares globaux
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
