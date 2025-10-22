use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};

use crate::error::{Result, TodoAppError};
use crate::model::{Todo, TodoStore};
use serde::Deserialize;
use serde_json::json;
use std::sync::{Arc, Mutex};

pub fn routes() -> Router<Arc<Mutex<TodoStore>>> {
    Router::new()
        .route("/api/test", post(api_todo))
        .route("/api/todos", get(get_all_todos).post(add_todo))
        .route("/api/todos/:id/complete", post(complete_todo))
        .route("/api/todos/:id/delete", delete(delete_todo))
}

async fn api_todo() -> impl IntoResponse {
    println!("✅ TODO testing endpoint hit");
    (StatusCode::OK, "Todo testing endpoint reached")
}

async fn get_all_todos(
    State(store): State<Arc<Mutex<TodoStore>>>
) -> Result<Json<Vec<Todo>>> {
    let store = store.lock().map_err(|_| TodoAppError::Internal)?;
    Ok(Json(store.get_all_todos().clone()))
}

#[derive(Deserialize)]
struct NewTodo {
    title: String,
}

async fn add_todo(
    State(store): State<Arc<Mutex<TodoStore>>>,
    Json(payload): Json<NewTodo>,
) -> Result<Json<Todo>> {
    if payload.title.trim().is_empty() {
        return Err(TodoAppError::MissingField("title".to_string()));
    }

    let mut store = store.lock().map_err(|_| TodoAppError::Internal)?;
    let todo = store.add_todo(payload.title);
    Ok(Json(todo.clone()))
}

async fn complete_todo(
    State(store): State<Arc<Mutex<TodoStore>>>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse> {
    let mut store = store.lock().map_err(|_| TodoAppError::Internal)?;

    match store.complete_todo(id) {
        Some(todo) => Ok(Json(todo.clone()).into_response()),
        None => Err(TodoAppError::NotFound(id)),
    }
}

async fn delete_todo(
    State(store): State<Arc<Mutex<TodoStore>>>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse> {
    let mut store = store.lock().map_err(|_| TodoAppError::Internal)?;
    match store.delete_todo(id) {
        Some(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": format!("Todo item {} deleted", id),
            })),
        )
            .into_response()),
        None => Err(TodoAppError::NotFound(id)),
    }
}