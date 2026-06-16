//! Couche d'accès aux données (Repositories).

pub mod content;
pub mod email_verification_repository;
pub mod user;

pub use content::{ContentRepository, SqliteContentRepository};
pub use email_verification_repository::{EmailVerificationRepository, SqliteEmailVerificationRepository};
pub use user::{SqliteUserRepository, UserRepository};
