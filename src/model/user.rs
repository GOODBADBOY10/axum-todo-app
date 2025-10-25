// use serde::{Deserialize, Serialize};
use serde::{Serialize};
use sqlx::FromRow;
use crate::db::DbPool;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

// #[derive(Debug, Deserialize)]
// pub struct CreateUser {
//     pub username: String,
//     pub password: String,
// }

impl User {
    //  Create a new user
    pub async fn create(pool: &DbPool, username: &str, password_hash: &str) -> sqlx::Result<Self> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (username, password_hash) VALUES (?, ?) RETURNING *",
        )
        .bind(username)
        .bind(password_hash)
        .fetch_one(pool)
        .await
    }

    // Find a user by username
    pub async fn find_by_username(pool: &DbPool, username: &str) -> sqlx::Result<Self> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(pool)
            .await
    }

    // Find a user by id
    // pub async fn find_by_id(pool: &DbPool, id: i64) -> sqlx::Result<Self> {
    //     sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
    //         .bind(id)
    //         .fetch_one(pool)
    //         .await
    // }

    // Check if username already exists
    // pub async fn exists(pool: &DbPool, username: &str) -> sqlx::Result<bool> {
    //     let result: Option<i64> = sqlx::query_scalar("SELECT id FROM users WHERE username = ?")
    //         .bind(username)
    //         .fetch_optional(pool)
    //         .await?;
    //     Ok(result.is_some())
    // }
}