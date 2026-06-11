//! Gestion des erreurs de l'application.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

/// Type d'erreur centralisé pour l'application Kroissant.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Erreur de base de données : {0}")]
    Database(#[from] sqlx::Error),

    #[error("Non autorisé : {0}")]
    Auth(String),

    #[error("Token invalide ou expiré")]
    InvalidToken,

    #[error("Ressource non trouvée : {0}")]
    NotFound(String),

    #[error("Erreur de validation : {0}")]
    Validation(String),

    #[error("Erreur interne : {0}")]
    Internal(#[from] anyhow::Error),

    #[error("Erreur de hachage : {0}")]
    HashError(String),

    #[error("Erreur JWT : {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}

pub type AppResult<T> = std::result::Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(ref e) => {
                eprintln!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Une erreur de base de données est survenue.".to_string(),
                )
            }
            AppError::Auth(m) => (StatusCode::UNAUTHORIZED, m),
            AppError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "Session expirée ou invalide.".to_string(),
            ),
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, m),
            AppError::Validation(m) => (StatusCode::BAD_REQUEST, m),
            AppError::Internal(ref e) => {
                eprintln!("Internal error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Une erreur interne est survenue.".to_string(),
                )
            }
            AppError::HashError(m) => {
                eprintln!("Hash error: {}", m);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Erreur lors du traitement du mot de passe.".to_string(),
                )
            }
            AppError::JwtError(ref e) => {
                eprintln!("JWT error: {:?}", e);
                (
                    StatusCode::UNAUTHORIZED,
                    "Erreur d'authentification.".to_string(),
                )
            }
        };

        (status, message).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    #[test]
    fn test_error_status_codes() {
        let err = AppError::NotFound("Contenu".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let err = AppError::Validation("Email invalide".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let err = AppError::Internal(anyhow!("Boom"));
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let err = AppError::InvalidToken;
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
