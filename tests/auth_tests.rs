use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use kroissant::{
    AppState,
    auth::AUTH_COOKIE,
    repositories::{SqliteContentRepository, SqliteUserRepository},
    routes::create_router,
    services::{AuthServiceImpl, ContentServiceImpl},
};
use std::sync::Arc;
use tower::ServiceExt; // for `oneshot` and `ready`

mod common;

async fn setup_app() -> axum::Router {
    let pool = common::setup_test_db().await;
    let content_repo = Arc::new(SqliteContentRepository::new(pool.clone()));
    let user_repo = Arc::new(SqliteUserRepository::new(pool.clone()));

    let jwt_secret = "test-secret".to_string();
    let auth_service = Arc::new(AuthServiceImpl::new(user_repo.clone(), jwt_secret.clone()));
    let content_service = Arc::new(ContentServiceImpl::new(
        content_repo.clone(),
        user_repo.clone(),
    ));

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
async fn test_register_login_flow() {
    let app = setup_app().await;

    // 1. Register
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/inscription")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(
                    "email=test@example.com&password=password123&next=/",
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(cookie.contains(AUTH_COOKIE));

    // 2. Login
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/connexion")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(
                    "email=test@example.com&password=password123&next=/",
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(cookie.contains(AUTH_COOKIE));
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    let app = setup_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/connexion")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from("email=wrong@example.com&password=wrong&next=/"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_account_requires_auth() {
    let app = setup_app().await;

    // Without auth
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/compte")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
