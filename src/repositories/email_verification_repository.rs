//! Repository pour la gestion des tokens de vérification d'email.
use crate::error::AppResult;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;

#[async_trait]
pub trait EmailVerificationRepository: Send + Sync {
    async fn insert_token(&self, token: &str, email: &str, expires_at: i64) -> AppResult<()>;
    async fn find_valid_token(&self, token: &str) -> AppResult<Option<String>>;
    async fn mark_token_used(&self, token: &str) -> AppResult<()>;
}

pub struct SqliteEmailVerificationRepository {
    pool: SqlitePool,
}

impl SqliteEmailVerificationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EmailVerificationRepository for SqliteEmailVerificationRepository {
    async fn insert_token(&self, token: &str, email: &str, expires_at: i64) -> AppResult<()> {
        sqlx::query("INSERT INTO email_verifications (token, email, expires_at) VALUES (?, ?, ?)")
            .bind(token)
            .bind(email)
            .bind(expires_at)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn find_valid_token(&self, token: &str) -> AppResult<Option<String>> {
        let now = Utc::now().timestamp();
        let row = sqlx::query_as::<_, (String,)>(
            "SELECT email FROM email_verifications WHERE token = ? AND used = 0 AND expires_at > ?",
        )
        .bind(token)
        .bind(now)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.0))
    }

    async fn mark_token_used(&self, token: &str) -> AppResult<()> {
        sqlx::query("UPDATE email_verifications SET used = 1 WHERE token = ?")
            .bind(token)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
