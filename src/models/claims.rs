//! Payload du token JWT (Claims).
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub email: String,
    pub exp: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_instantiation() {
        let claims = Claims {
            sub: 1,
            email: "test@example.com".to_string(),
            exp: 123456789,
        };
        assert_eq!(claims.sub, 1);
    }
}
