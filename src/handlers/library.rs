//! Handlers pour la bibliothèque de contenus.
use axum::{
    extract::{Query, State},
    response::Html,
};
use crate::app_state::AppState;
use crate::error::AppResult;
use crate::models::PlatformQuery;
use crate::auth::AuthUser;
use crate::utils::normalize_tag;
use crate::views;

/// Page de la bibliothèque complète.
pub async fn library(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(query): Query<PlatformQuery>,
) -> AppResult<Html<String>> {
    let active_tag = normalize_tag(query.tag.as_deref());
    let tags = state.content_repo.available_tags().await?;
    let series = state.content_repo.tagged_series(active_tag.as_deref()).await?;
    Ok(Html(views::render_library(&user, active_tag.as_deref(), &tags, &series)))
}

/// Partiel HTMX pour le filtrage de la bibliothèque.
pub async fn library_partial(
    State(state): State<AppState>,
    Query(query): Query<PlatformQuery>,
) -> AppResult<Html<String>> {
    let active_tag = normalize_tag(query.tag.as_deref());
    let tags = state.content_repo.available_tags().await?;
    let series = state.content_repo.tagged_series(active_tag.as_deref()).await?;
    Ok(Html(views::render_library_section(
        active_tag.as_deref(),
        &tags,
        &series,
    )))
}
