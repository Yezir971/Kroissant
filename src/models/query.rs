//! Modèles de requêtes et formulaires.
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlatformQuery {
    pub platform: Option<String>,
    pub tag: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AuthQuery {
    pub next: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AuthForm {
    pub email: String,
    pub password: String,
    pub next: Option<String>,
}
