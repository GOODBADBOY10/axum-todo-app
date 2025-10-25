use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use std::time::Duration;

pub type DbPool = Pool<Sqlite>;

pub async fn init_db(database_url: &str) -> anyhow::Result<DbPool> {
    // Create the SQLite pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    println!("SQLite database initialized successfully");
    Ok(pool)
}
