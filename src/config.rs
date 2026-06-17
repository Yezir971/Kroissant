use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub jwt_secret: String,
    pub app_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            google_client_id: env::var("GOOGLE_CLIENT_ID")
                .expect("GOOGLE_CLIENT_ID must be set"),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET")
                .expect("GOOGLE_CLIENT_SECRET must be set"),
            google_redirect_uri: env::var("GOOGLE_REDIRECT_URI")
                .expect("GOOGLE_REDIRECT_URI must be set"),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-me-kroissant".to_string()),
            app_url: env::var("APP_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
        }
    }
}
