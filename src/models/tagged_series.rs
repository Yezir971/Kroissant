//! Modèle représentant une série TMDb avec ses tags.
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct TaggedSeries {
    pub id: i64,
    pub tmdb_id: i64,
    pub name: String,
    pub overview: String,
    pub first_air_date: Option<String>,
    pub poster_path: Option<String>,
    pub platform: String,
    pub duration: Option<i64>,
    pub age_range: String,
    pub episode_context_count: i64,
    pub llm_reason: String,
    pub confidence: Option<f64>,
    pub source_url: String,
    pub tags: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tagged_series_instantiation() {
        let series = TaggedSeries {
            id: 1,
            tmdb_id: 123,
            name: "Test".to_string(),
            overview: "Desc".to_string(),
            first_air_date: Some("2024".to_string()),
            poster_path: None,
            platform: "netflix".to_string(),
            duration: Some(22),
            age_range: "6-10".to_string(),
            episode_context_count: 5,
            llm_reason: "Reason".to_string(),
            confidence: Some(0.95),
            source_url: "url".to_string(),
            tags: Some("tag1,tag2".to_string()),
        };
        assert_eq!(series.name, "Test");
    }
}
