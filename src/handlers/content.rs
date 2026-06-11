//! Handlers pour les contenus individuels.
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};
use crate::app_state::AppState;
use crate::error::AppResult;
use crate::auth::AuthUser;
use crate::views;

/// Page de détail d'un contenu.
pub async fn content_detail(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<Response> {
    let Some(content) = state.content_repo.get_by_slug(&slug).await? else {
        return Ok((
            StatusCode::NOT_FOUND,
            Html(views::render_error_page("Contenu introuvable.")),
        )
            .into_response());
    };

    let saved = match &user {
        Some(user) => state.user_repo.is_saved(user.id, content.id).await?,
        None => false,
    };

    let similar = state.content_repo.get_similar(&content).await?;
    let benefit = state.content_service.get_benefit_for(&content.skill);

    Ok(Html(views::render_content_detail(
        &user,
        &content,
        saved,
        &similar,
        &benefit,
    ))
    .into_response())
}

/// Alterne la sauvegarde d'un contenu en favoris.
pub async fn toggle_save(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Response> {
    let Some(user) = user else {
        return Ok(Html(r#"<p><a href="/inscription">Inscrivez-vous</a> pour sauvegarder ce contenu.</p>"#.to_string()).into_response());
    };

    let saved = state.content_service.toggle_save(user.id, id).await?;
    let content = state.content_repo.get_by_id(id).await?
        .ok_or_else(|| crate::error::AppError::NotFound("Contenu introuvable".to_string()))?;

    Ok(Html(views::render_save_panel(&content, Some(&user), saved)).into_response())
}

/// Redirige vers la source du contenu et log l'historique.
pub async fn go_to_source(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<Response> {
    let Some(content) = state.content_repo.get_by_slug(&slug).await? else {
        return Ok((
            StatusCode::NOT_FOUND,
            Html(views::render_error_page("Contenu introuvable.")),
        )
            .into_response());
    };

    if let Some(user) = user {
        state.content_service.log_watch_history(user.id, content.id).await?;
    }

    Ok(Redirect::to(&content.source_url).into_response())
}
