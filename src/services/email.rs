//! Service pour l'envoi d'emails de vérification.
use async_trait::async_trait;
use crate::error::{AppError, AppResult};
use crate::repositories::EmailVerificationRepository;
use chrono::{Duration, Utc};
use lettre::message::{header, MultiPart, SinglePart};
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
        self.repo.insert_token(&token, email, expires_at).await?;

        // 2. Préparer l'email
        let app_url = env::var("APP_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let from_email = env::var("FROM_EMAIL").unwrap_or_else(|_| "noreply@kroissant.fr".to_string());
        let verify_url = format!("{}/inscription/verify?token={}", app_url, token);

        let smtp_host = env::var("SMTP_HOST").map_err(|_| AppError::Internal(anyhow::anyhow!("SMTP_HOST non défini")))?;
        let smtp_port = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "2525".to_string())
            .parse::<u16>()
            .map_err(|_| AppError::Internal(anyhow::anyhow!("SMTP_PORT invalide")))?;
        let smtp_user = env::var("SMTP_USER").map_err(|_| AppError::Internal(anyhow::anyhow!("SMTP_USER non défini")))?;
        let smtp_pass = env::var("SMTP_PASSWORD").map_err(|_| AppError::Internal(anyhow::anyhow!("SMTP_PASSWORD non défini")))?;

        let email_msg = Message::builder()
            .from(from_email.parse().map_err(|_| AppError::Internal(anyhow::anyhow!("FROM_EMAIL invalide")))?)
            .to(email.parse().map_err(|_| AppError::Internal(anyhow::anyhow!("Email destinataire invalide")))?)
            .subject("Vérifiez votre adresse email - Kroissant")
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(format!(
                                "Bonjour,\n\nMerci de vous être inscrit sur Kroissant. Veuillez vérifier votre adresse email en cliquant sur le lien suivant :\n\n{}\n\nCe lien expirera dans 24 heures.",
                                verify_url
                            )),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(format!(
                                "<html><body><p>Bonjour,</p><p>Merci de vous être inscrit sur Kroissant. Veuillez vérifier votre adresse email en cliquant sur le bouton ci-dessous :</p><p><a href='{}' style='display:inline-block;background:#ff6b6b;color:white;padding:10px 20px;text-decoration:none;border-radius:5px;'>Vérifier mon email</a></p><p>Ce lien expirera dans 24 heures.</p></body></html>",
                                verify_url
                            )),
                    ),
            )
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Erreur construction email: {}", e)))?;

        // 3. Envoyer via SMTP
        let creds = Credentials::new(smtp_user, smtp_pass);
        let mailer = SmtpTransport::relay(&smtp_host)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Erreur relay SMTP: {}", e)))?
            .port(smtp_port)
            .credentials(creds)
            .build();

        mailer.send(&email_msg).map_err(|e| AppError::Internal(anyhow::anyhow!("Erreur envoi email: {}", e)))?;

        Ok(())
    }
}
