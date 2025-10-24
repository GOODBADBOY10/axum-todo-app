use crate::db::DbPool;
use crate::error::{ApiError, ApiResult};
use crate::model::Todo;

#[derive(Clone)]
pub struct TodoService {
    db: DbPool,
}

impl TodoService {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    pub async fn get_all_todos(&self, user_id: i64) -> ApiResult<Vec<Todo>> {
        Todo::find_all_by_user(&self.db, user_id)
            .await
            .map_err(ApiError::Database)
    }

    pub async fn add_todo(&self, user_id: i64, title: impl Into<String>) -> ApiResult<Todo> {
        let title = title.into();
        
        if title.trim().is_empty() {
            return Err(ApiError::BadRequest("Title cannot be empty".into()));
        }

        Todo::create(&self.db, user_id, &title)
            .await
            .map_err(ApiError::Database)
    }

    pub async fn get_todo(&self, id: i64, user_id: i64) -> ApiResult<Todo> {
        Todo::find_by_id(&self.db, id, user_id)
            .await
            .map_err(|_| ApiError::NotFound("Todo not found".into()))
    }

    pub async fn complete_todo(&self, id: i64, user_id: i64) -> ApiResult<Todo> {
        // First check if todo exists and belongs to user
        self.get_todo(id, user_id).await?;

        // Update to completed
        Todo::update(&self.db, id, user_id, None, Some(true))
            .await
            .map_err(ApiError::Database)
    }

    pub async fn update_todo(
        &self,
        id: i64,
        user_id: i64,
        title: Option<String>,
        completed: Option<bool>,
    ) -> ApiResult<Todo> {
        // Check if todo exists and belongs to user
        self.get_todo(id, user_id).await?;

        Todo::update(&self.db, id, user_id, title.as_deref(), completed)
            .await
            .map_err(ApiError::Database)
    }

    pub async fn delete_todo(&self, id: i64, user_id: i64) -> ApiResult<()> {
        // Check if todo exists and belongs to user
        self.get_todo(id, user_id).await?;

        Todo::delete(&self.db, id, user_id)
            .await
            .map_err(ApiError::Database)
    }
}

// use crate::db::DbPool;
// use crate::error::{ApiError, ApiResult};
// use crate::model::Todo;

// #[derive(Clone)]
// pub struct TodoService {
//     db: DbPool,
// }

// impl TodoService {
//     pub fn new(db: DbPool) -> Self {
//         Self { db }
//     }

//     pub async fn get_all_todos(&self, user_id: i64) -> ApiResult<Vec<Todo>> {
//         Todo::find_all_by_user(&self.db, user_id)
//             .await
//             .map_err(ApiError::Database)
//     }

//     pub async fn add_todo(&self, user_id: i64, title: impl Into<String>) -> ApiResult<Todo> {
//         let title = title.into();
        
//         if title.trim().is_empty() {
//             return Err(ApiError::BadRequest("Title cannot be empty".into()));
//         }

//         Todo::create(&self.db, user_id, &title)
//             .await
//             .map_err(ApiError::Database)
//     }

//     pub async fn get_todo(&self, id: i64, user_id: i64) -> ApiResult<Todo> {
//         Todo::find_by_id(&self.db, id, user_id)
//             .await
//             .map_err(|_| ApiError::NotFound("Todo not found".into()))
//     }

//     pub async fn complete_todo(&self, id: i64, user_id: i64) -> ApiResult<Todo> {
//         // First check if todo exists and belongs to user
//         self.get_todo(id, user_id).await?;

//         // Update to completed
//         Todo::update(&self.db, id, user_id, None, Some(true))
//             .await
//             .map_err(ApiError::Database)
//     }

//     pub async fn update_todo(
//         &self,
//         id: i64,
//         user_id: i64,
//         title: Option<String>,
//         completed: Option<bool>,
//     ) -> ApiResult<Todo> {
//         // Check if todo exists and belongs to user
//         self.get_todo(id, user_id).await?;

//         Todo::update(&self.db, id, user_id, title.as_deref(), completed)
//             .await
//             .map_err(ApiError::Database)
//     }

//     pub async fn delete_todo(&self, id: i64, user_id: i64) -> ApiResult<()> {
//         // Check if todo exists and belongs to user
//         self.get_todo(id, user_id).await?;

//         Todo::delete(&self.db, id, user_id)
//             .await
//             .map_err(ApiError::Database)
//     }
// }

// // use super::{Todo, TodoStore};
// // use std::sync::Arc;
// // use tokio::sync::RwLock;

// // #[derive(Clone)]
// // pub struct TodoService {
// //     store: Arc<RwLock<TodoStore>>,
// // }

// // impl TodoService {
// //     pub fn new() -> Self {
// //         Self {
// //             store: Arc::new(RwLock::new(TodoStore::new())),
// //         }
// //     }

// //     pub async fn get_all_todos(&self) -> Vec<Todo> {
// //         let store = self.store.read().await;
// //         store.get_all_todos()
// //     }

// //     pub async fn add_todo(&self, title: impl Into<String>) -> Todo {
// //         let mut store = self.store.write().await;
// //         store.add_todo(title)
// //     }

// //     pub async fn get_todo(&self, id: u64) -> Option<Todo> {
// //         let store = self.store.read().await;
// //         store.get_todo(id)
// //     }

// //     pub async fn complete_todo(&self, id: u64) -> Option<Todo> {
// //         let mut store = self.store.write().await;
// //         store.complete_todo(id)
// //     }

// //     pub async fn delete_todo(&self, id: u64) -> Option<Todo> {
// //         let mut store = self.store.write().await;
// //         store.delete_todo(id)
// //     }
// // }