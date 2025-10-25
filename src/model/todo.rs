use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::db::DbPool;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub completed: bool,
}

// #[derive(Debug, Deserialize)]
// pub struct CreateTodo {
//     pub title: String,
// }

// #[derive(Debug, Deserialize)]
// pub struct UpdateTodo {
//     pub title: Option<String>,
//     pub completed: Option<bool>,
// }

impl Todo {
    /// Create a new todo for a user
    pub async fn create(pool: &DbPool, user_id: i64, title: &str) -> sqlx::Result<Self> {
        sqlx::query_as::<_, Todo>(
            "INSERT INTO todos (user_id, title) VALUES (?, ?) RETURNING *",
        )
        .bind(user_id)
        .bind(title)
        .fetch_one(pool)
        .await
    }

    /// Get all todos for a specific user
    pub async fn find_all_by_user(pool: &DbPool, user_id: i64) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Todo>(
            "SELECT * FROM todos WHERE user_id = ? ORDER BY id DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// Get a single todo by id (only if it belongs to the user)
    pub async fn find_by_id(pool: &DbPool, id: i64, user_id: i64) -> sqlx::Result<Self> {
        sqlx::query_as::<_, Todo>(
            "SELECT * FROM todos WHERE id = ? AND user_id = ?"
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    /// Update a todo (title and/or completed status)
    pub async fn update(
        pool: &DbPool,
        id: i64,
        user_id: i64,
        title: Option<&str>,
        completed: Option<bool>,
    ) -> sqlx::Result<Self> {
        // Build dynamic query based on what's being updated
        let mut query_parts = vec![];
        let mut bind_count = 0;

        if title.is_some() {
            query_parts.push("title = ?");
            bind_count += 1;
        }
        if completed.is_some() {
            query_parts.push("completed = ?");
            bind_count += 1;
        }

        if bind_count == 0 {
            // Nothing to update, just return the current todo
            return Self::find_by_id(pool, id, user_id).await;
        }

        let query_str = format!(
            "UPDATE todos SET {} WHERE id = ? AND user_id = ? RETURNING *",
            query_parts.join(", ")
        );

        let mut query = sqlx::query_as::<_, Todo>(&query_str);

        if let Some(t) = title {
            query = query.bind(t);
        }
        if let Some(c) = completed {
            query = query.bind(c);
        }

        query.bind(id).bind(user_id).fetch_one(pool).await
    }

    /// Delete a todo (only if it belongs to the user)
    pub async fn delete(pool: &DbPool, id: i64, user_id: i64) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM todos WHERE id = ? AND user_id = ?")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}


// use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Todo {
//     pub id: u64,
//     pub title: String,
//     pub completed: bool,
// }

// impl Todo {
//     pub fn new(id: u64, title: impl Into<String>) -> Self {
//         Self { 
//             id,
//             title: title.into(), 
//             completed: false, 
//         }
//     }
// }