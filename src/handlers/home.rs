//! Handlers pour la page d'accueil et les sections générales.
use axum::{
    extract::{Query, State},
    response::Html,
};
use crate::app_state::AppState;
use crate::error::AppResult;
use crate::models::PlatformQuery;
use crate::auth::AuthUser;
use crate::utils::normalize_platform;
use crate::views;

/// Page d'accueil principale.
pub async fn home(AuthUser(user): AuthUser, State(state): State<AppState>) -> AppResult<Html<String>> {
    let selected = state.content_repo.get_contents(Some("youtube"), Some(2)).await?;
    let moment = state.content_repo.get_contents(None, Some(2)).await?;
    Ok(Html(views::render_home(&user, &selected, &moment)))
}

/// Partiel HTMX pour le filtrage par plateforme sur la home.
pub async fn home_partial(
    State(state): State<AppState>,
    Query(query): Query<PlatformQuery>,
) -> AppResult<Html<String>> {
    let platform = normalize_platform(query.platform.as_deref());
    let contents = state
        .content_repo
        .get_contents(Some(platform), Some(2))
        .await?;
    Ok(Html(views::render_home_platform_section(platform, &contents)))
}

/// Page "Science" expliquant la sélection.
pub async fn science(AuthUser(user): AuthUser, State(state): State<AppState>) -> AppResult<Html<String>> {
    let benefits = state.content_service.get_benefits();
    Ok(Html(views::render_science(&user, &benefits)))
}
