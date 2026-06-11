//! Modèle représentant un utilisateur.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_instantiation() {
        let user = User {
            id: 1,
            email: "test@example.com".to_string(),
        };
        assert_eq!(user.email, "test@example.com");
    }
}
