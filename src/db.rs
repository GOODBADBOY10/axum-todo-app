use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::time::Duration;

pub type DbPool = Pool<Sqlite>;


pub async fn init_db(database_url: &str) -> anyhow::Result<DbPool> {
    
    // Create the SQLite pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await?;

    // Run migrations or create tables manually (for now)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            completed BOOLEAN DEFAULT false,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
        "#,
    )
    .execute(&pool)
    .await?;

    println!("✅ SQLite database initialized successfully");
    Ok(pool)
}