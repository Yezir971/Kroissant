//! Repository pour la gestion des contenus.
use crate::error::AppResult;
use crate::models::{Content, TaggedSeries};
use async_trait::async_trait;
use sqlx::SqlitePool;

#[async_trait]
pub trait ContentRepository: Send + Sync {
    async fn get_contents(
        &self,
        platform: Option<&str>,
        limit: Option<i64>,
    ) -> AppResult<Vec<Content>>;
    async fn get_by_slug(&self, slug: &str) -> AppResult<Option<Content>>;
    async fn get_by_id(&self, id: i64) -> AppResult<Option<Content>>;
    async fn get_similar(&self, content: &Content) -> AppResult<Vec<Content>>;

    async fn available_tags(&self) -> AppResult<Vec<String>>;
    async fn tagged_series(&self, tag: Option<&str>) -> AppResult<Vec<TaggedSeries>>;
}

pub struct SqliteContentRepository {
    pool: SqlitePool,
}

impl SqliteContentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ContentRepository for SqliteContentRepository {
    async fn get_contents(
        &self,
        platform: Option<&str>,
        limit: Option<i64>,
    ) -> AppResult<Vec<Content>> {
        let limit = limit.unwrap_or(100);
        let rows = match platform {
            Some(p) => {
                sqlx::query_as::<_, Content>(
                    "SELECT * FROM contents WHERE platform = ? ORDER BY sort_order LIMIT ?",
                )
                .bind(p)
                .bind(limit)
                .fetch_all(&self.pool)
                .await?
            }
            None => {
                sqlx::query_as::<_, Content>("SELECT * FROM contents ORDER BY sort_order LIMIT ?")
                    .bind(limit)
                    .fetch_all(&self.pool)
                    .await?
            }
        };
        Ok(rows)
    }

    async fn get_by_slug(&self, slug: &str) -> AppResult<Option<Content>> {
        let row = sqlx::query_as::<_, Content>("SELECT * FROM contents WHERE slug = ?")
            .bind(slug)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    async fn get_by_id(&self, id: i64) -> AppResult<Option<Content>> {
        let row = sqlx::query_as::<_, Content>("SELECT * FROM contents WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    async fn get_similar(&self, content: &Content) -> AppResult<Vec<Content>> {
        let mut rows = sqlx::query_as::<_, Content>(
            r#"
            SELECT * FROM contents
            WHERE id != ? AND skill = ?
            ORDER BY sort_order
            LIMIT 2
            "#,
        )
        .bind(content.id)
        .bind(&content.skill)
        .fetch_all(&self.pool)
        .await?;

        if rows.len() < 2 {
            let fallback = sqlx::query_as::<_, Content>(
                r#"
                SELECT * FROM contents
                WHERE id != ?
                ORDER BY sort_order
                LIMIT ?
                "#,
            )
            .bind(content.id)
            .bind(2_i64 - rows.len() as i64)
            .fetch_all(&self.pool)
            .await?;
            rows.extend(fallback);
        }

        Ok(rows)
    }

    async fn available_tags(&self) -> AppResult<Vec<String>> {
        let rows: Vec<(String,)> = sqlx::query_as(
            r#"
            SELECT t.name
            FROM tags t
            INNER JOIN tmdb_series_tags st ON st.tag_id = t.id
            GROUP BY t.id, t.name
            ORDER BY t.name
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|row| row.0).collect())
    }

    async fn tagged_series(&self, tag: Option<&str>) -> AppResult<Vec<TaggedSeries>> {
        let rows = match tag {
            Some(search) => {
                let title_search = format!("%{}%", search);
                sqlx::query_as::<_, TaggedSeries>(
                    r#"
                    SELECT
                        s.id,
                        s.tmdb_id,
                        s.name,
                        s.overview,
                        s.first_air_date,
                        s.poster_path,
                        s.platform,
                        s.age_range,
                        s.episode_context_count,
                        s.llm_reason,
                        s.confidence,
                        s.source_url,
                        GROUP_CONCAT(t.name, ',') AS tags
                    FROM tmdb_series s
                    LEFT JOIN tmdb_series_tags st ON st.series_id = s.id
                    LEFT JOIN tags t ON t.id = st.tag_id
                    WHERE
                        s.name LIKE ?
                        OR s.original_name LIKE ?
                        OR EXISTS (
                            SELECT 1
                            FROM tmdb_series_tags selected_st
                            INNER JOIN tags selected_t ON selected_t.id = selected_st.tag_id
                            WHERE selected_st.series_id = s.id
                                AND selected_t.name = ?
                        )
                    GROUP BY s.id
                    ORDER BY s.name
                    LIMIT 100
                    "#,
                )
                .bind(&title_search)
                .bind(&title_search)
                .bind(search)
                .fetch_all(&self.pool)
                .await?
            }
            None => {
                sqlx::query_as::<_, TaggedSeries>(
                    r#"
                    SELECT
                        s.id,
                        s.tmdb_id,
                        s.name,
                        s.overview,
                        s.first_air_date,
                        s.poster_path,
                        s.platform,
                        s.age_range,
                        s.episode_context_count,
                        s.llm_reason,
                        s.confidence,
                        s.source_url,
                        GROUP_CONCAT(t.name, ',') AS tags
                    FROM tmdb_series s
                    LEFT JOIN tmdb_series_tags st ON st.series_id = s.id
                    LEFT JOIN tags t ON t.id = st.tag_id
                    GROUP BY s.id
                    ORDER BY s.name
                    LIMIT 100
                    "#,
                )
                .fetch_all(&self.pool)
                .await?
            }
        };

        Ok(rows)
    }
}
