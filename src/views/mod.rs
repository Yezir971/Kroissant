//! Rendu HTML et composants.

pub mod utils;
pub mod layout;
pub mod components;
pub mod home;
pub mod library;
pub mod content_detail;
pub mod science;
pub mod auth;
pub mod account;
pub mod error;

pub use utils::{a, h};
pub use layout::{render_page, render_page_with_class};
pub use components::*;
pub use home::render_home;
pub use library::render_library;
pub use content_detail::render_content_detail;
pub use science::render_science;
pub use auth::render_auth_page;
pub use account::render_account;
pub use error::{render_error_page, render_404};
