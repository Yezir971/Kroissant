//! Modèles de données (Domain Models).

pub mod auth_mode;
pub mod benefit;
pub mod claims;
pub mod content;
pub mod user;
pub mod query;
pub mod tagged_series;

pub use auth_mode::AuthMode;
pub use benefit::Benefit;
pub use claims::Claims;
pub use content::Content;
pub use user::User;
pub use query::{PlatformQuery, AuthQuery, AuthForm};
pub use tagged_series::TaggedSeries;
