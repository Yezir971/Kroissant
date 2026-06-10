//! Repository pour la gestion des utilisateurs et de leurs interactions.
use crate::error::AppResult;
use crate::models::{Content, User};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, email: &str, password_hash: &str) -> AppResult<i64>;
    async fn get_by_email(&self, email: &str) -> AppResult<Option<(i64, String)>>;
    async fn get_by_id(&self, id: i64) -> AppResult<Option<User>>;

    async fn is_saved(&self, user_id: i64, content_id: i64) -> AppResult<bool>;
    async fn save_item(&self, user_id: i64, content_id: i64) -> AppResult<()>;
    async fn unsave_item(&self, user_id: i64, content_id: i64) -> AppResult<()>;
    async fn get_saved_contents(&self, user_id: i64) -> AppResult<Vec<Content>>;

    async fn add_to_history(&self, user_id: i64, content_id: i64) -> AppResult<()>;
    async fn get_history_contents(&self, user_id: i64) -> AppResult<Vec<Content>>;
}

pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn create_user(&self, email: &str, password_hash: &str) -> AppResult<i64> {
        let result =
            sqlx::query("INSERT INTO users (email, password_hash, created_at) VALUES (?, ?, ?)")
                .bind(email)
                .bind(password_hash)
                .bind(Utc::now().to_rfc3339())
                .execute(&self.pool)
                .await?;

        Ok(result.last_insert_rowid())
    }

    async fn get_by_email(&self, email: &str) -> AppResult<Option<(i64, String)>> {
        let row = sqlx::query_as::<sqlx::Sqlite, (i64, String)>(
            "SELECT id, password_hash FROM users WHERE email = ?",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn get_by_id(&self, id: i64) -> AppResult<Option<User>> {
        let row = sqlx::query_as::<_, User>("SELECT id, email FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    async fn is_saved(&self, user_id: i64, content_id: i64) -> AppResult<bool> {
        let exists: Option<(i64,)> =
            sqlx::query_as("SELECT 1 FROM saved_items WHERE user_id = ? AND content_id = ?")
                .bind(user_id)
                .bind(content_id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(exists.is_some())
    }

    async fn save_item(&self, user_id: i64, content_id: i64) -> AppResult<()> {
        sqlx::query(
            "INSERT OR IGNORE INTO saved_items (user_id, content_id, created_at) VALUES (?, ?, ?)",
        )
        .bind(user_id)
        .bind(content_id)
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn unsave_item(&self, user_id: i64, content_id: i64) -> AppResult<()> {
        sqlx::query("DELETE FROM saved_items WHERE user_id = ? AND content_id = ?")
            .bind(user_id)
            .bind(content_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_saved_contents(&self, user_id: i64) -> AppResult<Vec<Content>> {
        let rows = sqlx::query_as::<_, Content>(
            r#"
            SELECT c.* FROM contents c
            INNER JOIN saved_items s ON s.content_id = c.id
            WHERE s.user_id = ?
            ORDER BY s.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn add_to_history(&self, user_id: i64, content_id: i64) -> AppResult<()> {
        sqlx::query("INSERT INTO watch_history (user_id, content_id, watched_at) VALUES (?, ?, ?)")
            .bind(user_id)
            .bind(content_id)
            .bind(Utc::now().to_rfc3339())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_history_contents(&self, user_id: i64) -> AppResult<Vec<Content>> {
        let rows = sqlx::query_as::<_, Content>(
            r#"
            SELECT c.* FROM contents c
            INNER JOIN watch_history h ON h.content_id = c.id
            WHERE h.user_id = ?
            GROUP BY c.id
            ORDER BY MAX(h.watched_at) DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }
}
