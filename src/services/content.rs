//! Service pour la gestion des contenus et des bénéfices.
use async_trait::async_trait;
use crate::error::{AppError, AppResult};
use crate::models::Benefit;
use crate::repositories::{ContentRepository, UserRepository};
use std::sync::Arc;

#[async_trait]
pub trait ContentService: Send + Sync {
    /// Retourne la liste des bénéfices scientifiques.
    fn get_benefits(&self) -> Vec<Benefit>;
    
    /// Retourne un bénéfice spécifique pour une compétence.
    fn get_benefit_for(&self, skill: &str) -> Benefit;
    
    /// Alterne l'état de sauvegarde d'un contenu pour un utilisateur.
    /// Retourne true si le contenu est maintenant sauvegardé, false sinon.
    async fn toggle_save(&self, user_id: i64, content_id: i64) -> AppResult<bool>;
    
    /// Enregistre une lecture dans l'historique.
    async fn log_watch_history(&self, user_id: i64, content_id: i64) -> AppResult<()>;
}

pub struct ContentServiceImpl {
    content_repo: Arc<dyn ContentRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl ContentServiceImpl {
    pub fn new(content_repo: Arc<dyn ContentRepository>, user_repo: Arc<dyn UserRepository>) -> Self {
        Self { content_repo, user_repo }
    }
}

#[async_trait]
impl ContentService for ContentServiceImpl {
    fn get_benefits(&self) -> Vec<Benefit> {
        Benefit::all()
    }

    fn get_benefit_for(&self, skill: &str) -> Benefit {
        Benefit::for_skill(skill)
    }

    async fn toggle_save(&self, user_id: i64, content_id: i64) -> AppResult<bool> {
        // Vérifier si le contenu existe
        if self.content_repo.get_by_id(content_id).await?.is_none() {
            return Err(AppError::NotFound("Contenu introuvable".to_string()));
        }

        let is_saved = self.user_repo.is_saved(user_id, content_id).await?;
        if is_saved {
            self.user_repo.unsave_item(user_id, content_id).await?;
            Ok(false)
        } else {
            self.user_repo.save_item(user_id, content_id).await?;
            Ok(true)
        }
    }

    async fn log_watch_history(&self, user_id: i64, content_id: i64) -> AppResult<()> {
        if self.content_repo.get_by_id(content_id).await?.is_none() {
            return Err(AppError::NotFound("Contenu introuvable".to_string()));
        }
        self.user_repo.add_to_history(user_id, content_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Content, User, TaggedSeries};

    struct MockContentRepo;
    #[async_trait]
    impl ContentRepository for MockContentRepo {
        async fn get_contents(&self, _p: Option<&str>, _l: Option<i64>) -> AppResult<Vec<Content>> { Ok(vec![]) }
        async fn get_by_slug(&self, _s: &str) -> AppResult<Option<Content>> { Ok(None) }
        async fn get_by_id(&self, id: i64) -> AppResult<Option<Content>> {
            if id == 1 {
                Ok(Some(Content { id: 1, slug: "test".into(), title: "Test".into(), platform: "yt".into(), duration: "10m".into(), age_range: "3-5".into(), description: "Desc".into(), skill: "science".into(), image_url: "".into(), source_url: "".into(), sort_order: 1 }))
            } else {
                Ok(None)
            }
        }
        async fn get_similar(&self, _c: &Content) -> AppResult<Vec<Content>> { Ok(vec![]) }
        async fn available_tags(&self) -> AppResult<Vec<String>> { Ok(vec![]) }
        async fn tagged_series(&self, _t: Option<&str>) -> AppResult<Vec<TaggedSeries>> { Ok(vec![]) }
    }

    struct MockUserRepo {
        is_saved: bool,
    }
    #[async_trait]
    impl UserRepository for MockUserRepo {
        async fn create_user(&self, _e: &str, _h: &str) -> AppResult<i64> { Ok(1) }
        async fn get_by_email(&self, _e: &str) -> AppResult<Option<(i64, String)>> { Ok(None) }
        async fn get_by_id(&self, _id: i64) -> AppResult<Option<User>> { Ok(None) }
        async fn is_saved(&self, _u: i64, _c: i64) -> AppResult<bool> { Ok(self.is_saved) }
        async fn save_item(&self, _u: i64, _c: i64) -> AppResult<()> { Ok(()) }
        async fn unsave_item(&self, _u: i64, _c: i64) -> AppResult<()> { Ok(()) }
        async fn get_saved_contents(&self, _u: i64) -> AppResult<Vec<Content>> { Ok(vec![]) }
        async fn add_to_history(&self, _u: i64, _c: i64) -> AppResult<()> { Ok(()) }
        async fn get_history_contents(&self, _u: i64) -> AppResult<Vec<Content>> { Ok(vec![]) }
    }

    #[tokio::test]
    async fn test_toggle_save_to_saved() {
        let content_repo = Arc::new(MockContentRepo);
        let user_repo = Arc::new(MockUserRepo { is_saved: false });
        let service = ContentServiceImpl::new(content_repo, user_repo);
        
        let result = service.toggle_save(1, 1).await;
        assert_eq!(result.unwrap(), true);
    }

    #[tokio::test]
    async fn test_toggle_save_to_unsaved() {
        let content_repo = Arc::new(MockContentRepo);
        let user_repo = Arc::new(MockUserRepo { is_saved: true });
        let service = ContentServiceImpl::new(content_repo, user_repo);
        
        let result = service.toggle_save(1, 1).await;
        assert_eq!(result.unwrap(), false);
    }

    #[tokio::test]
    async fn test_toggle_save_not_found() {
        let content_repo = Arc::new(MockContentRepo);
        let user_repo = Arc::new(MockUserRepo { is_saved: false });
        let service = ContentServiceImpl::new(content_repo, user_repo);
        
        let result = service.toggle_save(1, 999).await;
        assert!(matches!(result, Err(AppError::NotFound(_))));
    }

    #[test]
    fn test_get_benefits() {
        let content_repo = Arc::new(MockContentRepo);
        let user_repo = Arc::new(MockUserRepo { is_saved: false });
        let service = ContentServiceImpl::new(content_repo, user_repo);
        
        let benefits = service.get_benefits();
        assert!(!benefits.is_empty());
    }
}
