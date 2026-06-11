//! Modèles de données (Domain Models).

pub mod auth_mode;
pub mod benefit;
pub mod claims;
pub mod content;
pub mod query;
pub mod tagged_series;
pub mod user;

pub use auth_mode::AuthMode;
pub use benefit::Benefit;
pub use claims::Claims;
pub use content::Content;
pub use query::{AuthForm, AuthQuery, PlatformQuery};
pub use tagged_series::TaggedSeries;
pub use user::User;
