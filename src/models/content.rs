//! Modèle représentant un contenu (film, série, etc.)
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Content {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub platform: String,
    pub duration: String,
    pub age_range: String,
    pub description: String,
    pub skill: String,
    pub image_url: String,
    pub source_url: String,
    pub sort_order: i64,
}

impl Content {
    pub fn platform_label(&self) -> &'static str {
        match self.platform.as_str() {
            "youtube" => "YouTube",
            "netflix" => "Netflix",
            "disney" => "Disney+",
            _ => "Tous",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_instantiation() {
        let content = Content {
            id: 1,
            slug: "test".to_string(),
            title: "Test".to_string(),
            platform: "youtube".to_string(),
            duration: "10 min".to_string(),
            age_range: "3-5 ans".to_string(),
            description: "Desc".to_string(),
            skill: "skill".to_string(),
            image_url: "img".to_string(),
            source_url: "url".to_string(),
            sort_order: 1,
        };
        assert_eq!(content.title, "Test");
        assert_eq!(content.platform_label(), "YouTube");
    }
}
