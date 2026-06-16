//! Service pour l'envoi d'emails de vérification.
use async_trait::async_trait;
use crate::error::{AppError, AppResult};
use crate::repositories::EmailVerificationRepository;
use crate::views::email::{verification, verification_plain};
use chrono::{Duration, Utc};
use lettre::message::{MultiPart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait EmailService: Send + Sync {
    /// Génère un token, l'enregistre et envoie l'email de vérification.
    async fn send_verification_email(&self, email: &str) -> AppResult<()>;
}

pub struct EmailServiceImpl {
    repo: Arc<dyn EmailVerificationRepository>,
}

impl EmailServiceImpl {
    pub fn new(repo: Arc<dyn EmailVerificationRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl EmailService for EmailServiceImpl {
    async fn send_verification_email(&self, email: &str) -> AppResult<()> {
        let token = Uuid::new_v4().to_string();
        let expires_at = (Utc::now() + Duration::hours(24)).timestamp();

        // 1. Enregistrer le token
        if let Err(e) = self.repo.insert_token(&token, email, expires_at).await {
            tracing::error!("Erreur insertion token DB: {:?}", e);
            return Err(e);
        }

        // 2. Préparer l'email
        let app_url = env::var("APP_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let from_email = env::var("FROM_EMAIL").unwrap_or_else(|_| "noreply@kroissant.fr".to_string());
        let verify_url = format!("{}/inscription/verify?token={}", app_url, token);

        let smtp_host = env::var("SMTP_HOST").map_err(|_| {
            tracing::error!("SMTP_HOST non défini dans l'environnement");
            AppError::Internal(anyhow::anyhow!("SMTP_HOST non défini"))
        })?;
        let smtp_port = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "2525".to_string())
            .parse::<u16>()
            .map_err(|_| AppError::Internal(anyhow::anyhow!("SMTP_PORT invalide")))?;
        let smtp_user = env::var("SMTP_USER").map_err(|_| AppError::Internal(anyhow::anyhow!("SMTP_USER non défini")))?;
        let smtp_pass = env::var("SMTP_PASSWORD").map_err(|_| AppError::Internal(anyhow::anyhow!("SMTP_PASSWORD non défini")))?;

        let html_body = verification::render(&verify_url);
        let plain_body = verification_plain::render(&verify_url);

        let email_msg = Message::builder()
            .from(from_email.parse().map_err(|_| AppError::Internal(anyhow::anyhow!("FROM_EMAIL parse error")))?)
            .to(email.parse().map_err(|_| AppError::Internal(anyhow::anyhow!("To email parse error")))?)
            .subject("Verification de votre compte - Kroissant")
            .multipart(MultiPart::alternative_plain_html(plain_body, html_body))
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Message build error: {}", e)))?;

        // 3. Envoyer via SMTP
        let creds = Credentials::new(smtp_user, smtp_pass);
        
        let mailer = SmtpTransport::relay(&smtp_host)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Erreur relay SMTP: {}", e)))?
            .port(smtp_port)
            .credentials(creds)
            .tls(lettre::transport::smtp::client::Tls::None) // Désactiver TLS pour isoler le bug ContentType
            .build();

        if let Err(e) = mailer.send(&email_msg) {
            tracing::error!("ÉCHEC de l'envoi d'email SMTP: {:?}", e);
            return Err(AppError::Internal(anyhow::anyhow!("Erreur envoi email: {}", e)));
        }

        tracing::info!("Email de vérification envoyé avec succès à {}", email);
        Ok(())
    }
}
