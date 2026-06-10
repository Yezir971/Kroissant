use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use kroissant::{
    AppState,
    routes::create_router,
    repositories::{SqliteContentRepository, SqliteUserRepository},
    services::{AuthServiceImpl, ContentServiceImpl},
};
use std::sync::Arc;
use tower::ServiceExt;

mod common;

async fn setup_app() -> axum::Router {
    let pool = common::setup_test_db().await;
    
    // Seed test data
    sqlx::query(
        "INSERT INTO contents (slug, title, platform, duration, age_range, description, skill, image_url, source_url, sort_order) \
         VALUES ('test-slug', 'Test Content', 'youtube', '10m', '3-5', 'Desc', 'science', 'img', 'url', 1)"
    )
    .execute(&pool)
    .await
    .unwrap();

    let content_repo = Arc::new(SqliteContentRepository::new(pool.clone()));
    let user_repo = Arc::new(SqliteUserRepository::new(pool.clone()));

    let jwt_secret = "test-secret".to_string();
    let auth_service = Arc::new(AuthServiceImpl::new(user_repo.clone(), jwt_secret.clone()));
    let content_service = Arc::new(ContentServiceImpl::new(content_repo.clone(), user_repo.clone()));

    let state = AppState::new(
        pool,
        jwt_secret,
        content_repo,
        user_repo,
        auth_service,
        content_service,
    );

    create_router(state)
}

#[tokio::test]
async fn test_home_page() {
    let app = setup_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_content_detail_found() {
    let app = setup_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/contenu/test-slug")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_content_detail_not_found() {
    let app = setup_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/contenu/non-existent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_toggle_save_unauthorized_returns_prompt() {
    let app = setup_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/contenu/1/save")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // The handler returns a partial HTML prompt if not logged in
    assert_eq!(response.status(), StatusCode::OK);
}
