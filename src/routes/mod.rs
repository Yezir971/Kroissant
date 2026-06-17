//! Configuration du routage et des middlewares.
use axum::{
    routing::{get, post, get_service},
    Router,
};
use tower_http::{services::{ServeDir, ServeFile}, trace::TraceLayer};
use crate::app_state::AppState;
use crate::handlers;

/// Crée le routeur principal de l'application.
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Accueil & Général
        .route("/", get(handlers::home::home))
        .route("/science", get(handlers::home::science))
        
        // PWA
        .route("/manifest.json", get_service(ServeFile::new("static/manifest.json")))
        .route("/sw.js", get_service(ServeFile::new("static/sw.js")))
        
        // Bibliothèque
        .route("/bibliotheque", get(handlers::library::library))
        
        // Contenus
        .route("/contenu/{slug}", get(handlers::content::content_detail))
        .route("/contenu/{id}/save", post(handlers::content::toggle_save))
        .route("/go/{slug}", get(handlers::content::go_to_source))
        
        // Authentification & Compte
        .route("/auth/google", get(handlers::auth::google_auth))
        .route("/auth/google/callback", get(handlers::auth::google_callback))
        .route("/inscription", get(handlers::registration::registration_shell).post(handlers::auth::register_deprecated))
        .route("/inscription/verify", get(handlers::registration::verify_email))
        .route("/inscription/step/email", get(handlers::registration::get_step_email).post(handlers::registration::post_email))
        .route("/inscription/step/password", get(handlers::registration::get_step_password).post(handlers::registration::post_password))
        .route("/inscription/step/profile", get(handlers::registration::get_step_profile).post(handlers::registration::post_profile))
        .route("/inscription/step/{step}", get(handlers::registration::get_step))
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
        .fallback(handlers::home::not_found)
        .with_state(state)
}
