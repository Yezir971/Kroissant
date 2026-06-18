//! Handlers pour la bibliothèque de contenus.
use crate::app_state::AppState;
use crate::auth::AuthUser;
use crate::error::AppResult;
use crate::models::PlatformQuery;
use crate::views;
use axum::{
    extract::{Query, State},
    response::Html,
};

/// Page de la bibliothèque complète.
pub async fn library(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(query): Query<PlatformQuery>,
) -> AppResult<Html<String>> {
    let tags = state.content_repo.available_tags().await?;
    let series = state.content_repo.tagged_series(&query).await?;
    Ok(Html(views::render_library(&user, &query, &tags, &series)))
}

/// Partiel HTMX pour le filtrage de la bibliothèque.
pub async fn library_partial(
    State(state): State<AppState>,
    Query(query): Query<PlatformQuery>,
) -> AppResult<Html<String>> {
    let tags = state.content_repo.available_tags().await?;
    let series = state.content_repo.tagged_series(&query).await?;
    Ok(Html(views::render_library_section(&query, &tags, &series)))
}
