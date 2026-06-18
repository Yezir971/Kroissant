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
    async fn tagged_series(
        &self,
        query: &crate::models::PlatformQuery,
    ) -> AppResult<Vec<TaggedSeries>>;
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

    async fn tagged_series(
        &self,
        query: &crate::models::PlatformQuery,
    ) -> AppResult<Vec<TaggedSeries>> {
        let mut query_builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
            r#"
            SELECT
                s.id,
                s.tmdb_id,
                s.name,
                s.overview,
                s.first_air_date,
                s.poster_path,
                s.platform,
                CAST(AVG(e.runtime) AS INTEGER) AS duration,
                s.age_range,
                s.episode_context_count,
                s.llm_reason,
                s.confidence,
                s.source_url,
                GROUP_CONCAT(t.name, ',') AS tags
            FROM tmdb_series s
            LEFT JOIN tmdb_series_tags st ON st.series_id = s.id
            LEFT JOIN tags t ON t.id = st.tag_id
            LEFT JOIN tmdb_episodes e ON e.series_id = s.id AND e.runtime IS NOT NULL
            "#,
        );

        query_builder.push(" WHERE 1=1 ");

        // Filtre de recherche textuelle (Nom ou Tag par défaut si on veut garder la polyvalence)
        if let Some(search) = query.tag.as_deref() {
            if !search.is_empty() && search != "all" {
                let pattern = format!("%{}%", search);
                query_builder.push(" AND (s.name LIKE ");
                query_builder.push_bind(pattern.clone());
                query_builder.push(" OR s.original_name LIKE ");
                query_builder.push_bind(pattern);
                // On peut aussi chercher dans les tags via le champ texte pour plus de flexibilité
                query_builder.push(" OR EXISTS (SELECT 1 FROM tmdb_series_tags search_st INNER JOIN tags search_t ON search_t.id = search_st.tag_id WHERE search_st.series_id = s.id AND search_t.name LIKE ");
                query_builder.push_bind(format!("%{}%", search));
                query_builder.push(")) ");
            }
        }

        // Filtre par Skill (Puce de compétence)
        if let Some(skill) = query.skill.as_deref() {
            if !skill.is_empty() && skill != "all" {
                query_builder.push(" AND EXISTS (SELECT 1 FROM tmdb_series_tags tst INNER JOIN tags tt ON tt.id = tst.tag_id WHERE tst.series_id = s.id AND tt.name = ");
                query_builder.push_bind(skill);
                query_builder.push(") ");
            }
        }

        // Filtre par Plateforme
        if let Some(platform) = query.platform.as_deref() {
            if !platform.is_empty() && platform != "all" {
                query_builder.push(" AND s.platform = ");
                query_builder.push_bind(platform);
            }
        }

        // Filtre par Âge
        if let Some(age) = query.age.as_deref() {
            if !age.is_empty() && age != "all" {
                query_builder.push(" AND s.age_range = ");
                query_builder.push_bind(age);
            }
        }

        query_builder.push(" GROUP BY s.id ORDER BY s.name LIMIT 100 ");

        let rows = query_builder
            .build_query_as::<TaggedSeries>()
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }
}
