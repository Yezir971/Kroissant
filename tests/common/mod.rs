use sqlx::SqlitePool;

pub async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    let statements = [
        r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#,
        r#"
        CREATE TABLE contents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            slug TEXT NOT NULL UNIQUE,
            title TEXT NOT NULL,
            platform TEXT NOT NULL,
            duration TEXT NOT NULL,
            age_range TEXT NOT NULL,
            description TEXT NOT NULL,
            skill TEXT NOT NULL,
            image_url TEXT NOT NULL,
            source_url TEXT NOT NULL,
            sort_order INTEGER NOT NULL
        )
        "#,
        r#"
        CREATE TABLE saved_items (
            user_id INTEGER NOT NULL,
            content_id INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            PRIMARY KEY (user_id, content_id),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE
        )
        "#,
        r#"
        CREATE TABLE watch_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            content_id INTEGER NOT NULL,
            watched_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE
        )
        "#,
        r#"
        CREATE TABLE tmdb_series (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tmdb_id INTEGER NOT NULL UNIQUE,
            name TEXT NOT NULL,
            original_name TEXT NOT NULL DEFAULT '',
            overview TEXT NOT NULL DEFAULT '',
            first_air_date TEXT,
            poster_path TEXT,
            platform TEXT NOT NULL DEFAULT '',
            age_range TEXT NOT NULL DEFAULT '',
            episode_context_count INTEGER NOT NULL DEFAULT 0,
            llm_reason TEXT NOT NULL DEFAULT '',
            confidence REAL,
            source_url TEXT NOT NULL DEFAULT '',
            updated_at TEXT NOT NULL
        )
        "#,
        r#"
        CREATE TABLE tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )
        "#,
        r#"
        CREATE TABLE tmdb_series_tags (
            series_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (series_id, tag_id),
            FOREIGN KEY (series_id) REFERENCES tmdb_series(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )
        "#,
    ];

    for statement in statements {
        sqlx::query(statement).execute(&pool).await.unwrap();
    }

    pool
}
