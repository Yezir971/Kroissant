//! Service d'authentification.
use crate::auth;
use crate::error::{AppError, AppResult};
use crate::models::Claims;
use crate::repositories::UserRepository;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use std::sync::Arc;

#[async_trait]
pub trait AuthService: Send + Sync {
    /// Inscrit un nouvel utilisateur et retourne un token JWT.
    async fn register(&self, email: &str, password: &str, pseudo: &str) -> AppResult<String>;

    /// Connecte un utilisateur existant et retourne un token JWT.
    async fn login(&self, email: &str, password: &str) -> AppResult<String>;
}

pub struct AuthServiceImpl {
    user_repo: Arc<dyn UserRepository>,
    jwt_secret: String,
}

impl AuthServiceImpl {
    pub fn new(user_repo: Arc<dyn UserRepository>, jwt_secret: String) -> Self {
        Self {
            user_repo,
            jwt_secret,
        }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn register(&self, email: &str, password: &str, pseudo: &str) -> AppResult<String> {
        let email = email.trim().to_lowercase();

        if email.is_empty() || password.len() < 8 {
            return Err(AppError::Auth(
                "Email invalide ou mot de passe trop court (min 8 car.)".to_string(),
            ));
        }

        if self.user_repo.get_by_email(&email).await?.is_some() {
            return Err(AppError::Auth("Cet email est déjà utilisé".to_string()));
        }

        let hash = auth::password::hash_password(password)?;
        let user_id = self.user_repo.create_user(&email, &hash, pseudo).await?;

        let claims = Claims {
            sub: user_id,
            email,
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
        };

        auth::jwt::create_token(&claims, &self.jwt_secret)
    }

    async fn login(&self, email: &str, password: &str) -> AppResult<String> {
        let email = email.trim().to_lowercase();

        let Some((id, hash)) = self.user_repo.get_by_email(&email).await? else {
            return Err(AppError::Auth(
                "Email ou mot de passe incorrect".to_string(),
            ));
        };

        if !auth::password::verify_password(password, &hash) {
            return Err(AppError::Auth(
                "Email ou mot de passe incorrect".to_string(),
            ));
        }

        let claims = Claims {
            sub: id,
            email,
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
        };

        auth::jwt::create_token(&claims, &self.jwt_secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Content, User};

    struct MockUserRepo {
        user_exists: bool,
    }

    #[async_trait]
    impl UserRepository for MockUserRepo {
        async fn create_user(&self, _email: &str, _hash: &str, _pseudo: &str) -> AppResult<i64> {
            Ok(1)
        }
        async fn get_by_email(&self, email: &str) -> AppResult<Option<(i64, String)>> {
            if self.user_exists && email == "exists@test.com" {
                Ok(Some((
                    1,
                    auth::password::hash_password("password123").unwrap(),
                )))
            } else {
                Ok(None)
            }
        }
        async fn get_by_id(&self, _id: i64) -> AppResult<Option<User>> {
            Ok(None)
        }
        async fn is_saved(&self, _u: i64, _c: i64) -> AppResult<bool> {
            Ok(false)
        }
        async fn save_item(&self, _u: i64, _c: i64) -> AppResult<()> {
            Ok(())
        }
        async fn unsave_item(&self, _u: i64, _c: i64) -> AppResult<()> {
            Ok(())
        }
        async fn get_saved_contents(&self, _u: i64) -> AppResult<Vec<Content>> {
            Ok(vec![])
        }
        async fn add_to_history(&self, _u: i64, _c: i64) -> AppResult<()> {
            Ok(())
        }
        async fn get_history_contents(&self, _u: i64) -> AppResult<Vec<Content>> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn test_register_success() {
        let repo = Arc::new(MockUserRepo { user_exists: false });
        let service = AuthServiceImpl::new(repo, "secret".to_string());

        let result = service
            .register("new@test.com", "password123", "Zeppa")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_validation_failure() {
        let repo = Arc::new(MockUserRepo { user_exists: false });
        let service = AuthServiceImpl::new(repo, "secret".to_string());

        let result = service.register("", "short", "").await;
        assert!(matches!(result, Err(AppError::Auth(_))));
    }

    #[tokio::test]
    async fn test_register_already_exists() {
        let repo = Arc::new(MockUserRepo { user_exists: true });
        let service = AuthServiceImpl::new(repo, "secret".to_string());

        let result = service
            .register("exists@test.com", "password123", "Zeppa")
            .await;
        assert!(matches!(result, Err(AppError::Auth(_))));
    }

    #[tokio::test]
    async fn test_login_success() {
        let repo = Arc::new(MockUserRepo { user_exists: true });
        let service = AuthServiceImpl::new(repo, "secret".to_string());

        let result = service.login("exists@test.com", "password123").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let repo = Arc::new(MockUserRepo { user_exists: true });
        let service = AuthServiceImpl::new(repo, "secret".to_string());

        let result = service.login("exists@test.com", "wrong").await;
        assert!(matches!(result, Err(AppError::Auth(_))));
    }
}
