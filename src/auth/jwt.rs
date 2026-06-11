//! Gestion des tokens JWT (création et validation).
use crate::error::{AppError, AppResult};
use crate::models::Claims;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

/// Crée un nouveau token JWT.
pub fn create_token(claims: &Claims, secret: &str) -> AppResult<String> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(AppError::from)
}

/// Valide un token JWT et retourne les claims.
pub fn validate_token(token: &str, secret: &str) -> AppResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::InvalidToken)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[tokio::test]
    async fn test_jwt_creation_and_validation() {
        let secret = "secret_de_test";
        let exp = Utc::now() + Duration::days(1);
        let claims = Claims {
            sub: 1,
            email: "test@example.com".to_string(),
            exp: exp.timestamp() as usize,
        };

        let token =
            create_token(&claims, secret).expect("La création du token ne devrait pas échouer");
        let validated_claims =
            validate_token(&token, secret).expect("La validation devrait réussir");

        assert_eq!(validated_claims.sub, claims.sub);
        assert_eq!(validated_claims.email, claims.email);
    }

    #[tokio::test]
    async fn test_jwt_validation_failure() {
        let secret = "secret_de_test";
        let token = "token_invalide";
        let result = validate_token(token, secret);
        assert!(matches!(result, Err(AppError::InvalidToken)));
    }
}
