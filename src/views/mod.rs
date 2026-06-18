//! Rendu HTML et composants.

pub mod account;
pub mod auth;
pub mod components;
pub mod content_detail;
pub mod email;
pub mod error;
pub mod home;
pub mod layout;
pub mod legal;
pub mod library;
pub mod registration;
pub mod science;
pub mod utils;

pub use account::render_account;
pub use auth::render_auth_page;
pub use components::*;
pub use content_detail::render_content_detail;
pub use error::{render_404, render_error_page};
pub use home::render_home;
pub use layout::{render_page, render_page_with_class};
pub use legal::{render_privacy, render_terms};
pub use library::render_library;
pub use science::render_science;
pub use utils::{a, h};
