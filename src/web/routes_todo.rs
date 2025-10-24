use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::Deserialize;

use crate::model::{AppState, Todo, TodoService};

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    title: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/todos", get(get_all_todos))
        .route("/todos", post(create_todo))
        .route("/todos/:id", get(get_todo))
        .route("/todos/:id/complete", patch(complete_todo))
        .route("/todos/:id", delete(delete_todo))
}

// Handlers - Each extracts only TodoService, not entire AppState
async fn get_all_todos(
    State(todo_service): State<TodoService>,
) -> Json<Vec<Todo>> {
    let todos = todo_service.get_all_todos().await;
    Json(todos)
}

async fn create_todo(
    State(todo_service): State<TodoService>,
    Json(payload): Json<CreateTodoRequest>,
) -> Json<Todo> {
    let todo = todo_service.add_todo(payload.title).await;
    Json(todo)
}

async fn get_todo(
    State(todo_service): State<TodoService>,
    Path(id): Path<u64>,
) -> Result<Json<Todo>, StatusCode> {
    todo_service
        .get_todo(id)
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn complete_todo(
    State(todo_service): State<TodoService>,
    Path(id): Path<u64>,
) -> Result<Json<Todo>, StatusCode> {
    todo_service
        .complete_todo(id)
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn delete_todo(
    State(todo_service): State<TodoService>,
    Path(id): Path<u64>,
) -> Result<Json<Todo>, StatusCode> {
    todo_service
        .delete_todo(id)
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}


// use axum::{
//     Json, Router,
//     extract::{Path, State},
//     http::StatusCode,
//     response::IntoResponse,
//     routing::{delete, get, post},
// };

// use crate::error::{Result, TodoAppError};
// use crate::model::{Todo, TodoStore};
// use serde::Deserialize;
// use serde_json::json;
// use std::sync::{Arc, Mutex};

// pub fn routes() -> Router<Arc<Mutex<TodoStore>>> {
//     Router::new()
//         .route("/api/test", post(api_todo))
//         .route("/api/todos", get(get_all_todos).post(add_todo))
//         .route("/api/todos/:id/complete", post(complete_todo))
//         .route("/api/todos/:id/delete", delete(delete_todo))
// }

// async fn api_todo() -> impl IntoResponse {
//     println!("✅ TODO testing endpoint hit");
//     (StatusCode::OK, "Todo testing endpoint reached")
// }

// async fn get_all_todos(
//     State(store): State<Arc<Mutex<TodoStore>>>
// ) -> Result<Json<Vec<Todo>>> {
//     let store = store.lock().map_err(|_| TodoAppError::Internal)?;
//     Ok(Json(store.get_all_todos().clone()))
// }

// #[derive(Deserialize)]
// struct NewTodo {
//     title: String,
// }

// async fn add_todo(
//     State(store): State<Arc<Mutex<TodoStore>>>,
//     Json(payload): Json<NewTodo>,
// ) -> Result<Json<Todo>> {
//     if payload.title.trim().is_empty() {
//         return Err(TodoAppError::MissingField("title".to_string()));
//     }

//     let mut store = store.lock().map_err(|_| TodoAppError::Internal)?;
//     let todo = store.add_todo(payload.title);
//     Ok(Json(todo.clone()))
// }

// async fn complete_todo(
//     State(store): State<Arc<Mutex<TodoStore>>>,
//     Path(id): Path<u64>,
// ) -> Result<impl IntoResponse> {
//     let mut store = store.lock().map_err(|_| TodoAppError::Internal)?;

//     match store.complete_todo(id) {
//         Some(todo) => Ok(Json(todo.clone()).into_response()),
//         None => Err(TodoAppError::NotFound(id)),
//     }
// }

// async fn delete_todo(
//     State(store): State<Arc<Mutex<TodoStore>>>,
//     Path(id): Path<u64>,
// ) -> Result<impl IntoResponse> {
//     let mut store = store.lock().map_err(|_| TodoAppError::Internal)?;
//     match store.delete_todo(id) {
//         Some(_) => Ok((
//             StatusCode::OK,
//             Json(json!({
//                 "success": true,
//                 "message": format!("Todo item {} deleted", id),
//             })),
//         )
//             .into_response()),
//         None => Err(TodoAppError::NotFound(id)),
//     }
// }