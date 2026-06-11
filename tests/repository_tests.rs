mod common;

use common::setup_test_db;
use kroissant::repositories::content::{ContentRepository, SqliteContentRepository};
use kroissant::repositories::user::{SqliteUserRepository, UserRepository};

#[tokio::test]
async fn test_content_repository() {
    let pool = setup_test_db().await;
    let repo = SqliteContentRepository::new(pool.clone());

    // Seed
    sqlx::query("INSERT INTO contents (slug, title, platform, duration, age_range, description, skill, image_url, source_url, sort_order) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind("slug1").bind("Title 1").bind("youtube").bind("10m").bind("3-5").bind("Desc").bind("resilience").bind("img").bind("url").bind(1)
        .execute(&pool).await.unwrap();

    let contents = repo.get_contents(None, None).await.unwrap();
    assert_eq!(contents.len(), 1);
    assert_eq!(contents[0].slug, "slug1");

    let by_slug = repo.get_by_slug("slug1").await.unwrap().unwrap();
    assert_eq!(by_slug.title, "Title 1");

    let similar = repo.get_similar(&by_slug).await.unwrap();
    // Only 1 content, so similar should be empty
    assert_eq!(similar.len(), 0);

    // TMDb series
    sqlx::query("INSERT INTO tmdb_series (tmdb_id, name, updated_at) VALUES (?, ?, ?)")
        .bind(100)
        .bind("Series 1")
        .bind("now")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO tags (name) VALUES (?)")
        .bind("tag1")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO tmdb_series_tags (series_id, tag_id) VALUES (1, 1)")
        .execute(&pool)
        .await
        .unwrap();

    let tags = repo.available_tags().await.unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0], "tag1");

    let series = repo.tagged_series(Some("tag1")).await.unwrap();
    assert_eq!(series.len(), 1);
    assert_eq!(series[0].name, "Series 1");

    let series_by_title = repo.tagged_series(Some("ries 1")).await.unwrap();
    assert_eq!(series_by_title.len(), 1);
    assert_eq!(series_by_title[0].name, "Series 1");
}

#[tokio::test]
async fn test_user_repository() {
    let pool = setup_test_db().await;
    let repo = SqliteUserRepository::new(pool.clone());

    let user_id = repo.create_user("test@example.com", "hash").await.unwrap();
    assert!(user_id > 0);

    let user = repo.get_by_id(user_id).await.unwrap().unwrap();
    assert_eq!(user.email, "test@example.com");

    let auth = repo
        .get_by_email("test@example.com")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(auth.1, "hash");

    // Content for relationship tests
    sqlx::query("INSERT INTO contents (id, slug, title, platform, duration, age_range, description, skill, image_url, source_url, sort_order) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(1).bind("slug1").bind("Title 1").bind("youtube").bind("10m").bind("3-5").bind("Desc").bind("resilience").bind("img").bind("url").bind(1)
        .execute(&pool).await.unwrap();

    // Saved items
    assert!(!repo.is_saved(user_id, 1).await.unwrap());
    repo.save_item(user_id, 1).await.unwrap();
    assert!(repo.is_saved(user_id, 1).await.unwrap());

    let saved = repo.get_saved_contents(user_id).await.unwrap();
    assert_eq!(saved.len(), 1);

    repo.unsave_item(user_id, 1).await.unwrap();
    assert!(!repo.is_saved(user_id, 1).await.unwrap());

    // History
    repo.add_to_history(user_id, 1).await.unwrap();
    let history = repo.get_history_contents(user_id).await.unwrap();
    assert_eq!(history.len(), 1);
}
