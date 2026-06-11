//! Logique de hachage et vérification des mots de passe via Argon2.
use crate::error::{AppError, AppResult};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand_core::OsRng;

/// Hache un mot de passe en utilisant Argon2.
pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::HashError(e.to_string()))
        .map(|hash| hash.to_string())
}

/// Vérifie un mot de passe par rapport à son hachage.
pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(password_hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_password_hashing_and_verification() {
        let password = "password123";
        let hash = hash_password(password).expect("Le hachage ne devrait pas échouer");

        assert!(verify_password(password, &hash));
        assert!(!verify_password("wrong_password", &hash));
    }
}
