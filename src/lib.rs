//! Kroissant - Curation de contenus développementaux pour enfants.

pub mod app_state;
pub mod auth;
pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod utils;
pub mod views;

pub use app_state::AppState;
pub use error::{AppError, AppResult};
