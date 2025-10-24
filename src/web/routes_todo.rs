use axum::{
    extract::{Path, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::Deserialize;
use crate::{
    auth::AuthUser,
    error::{ApiResult, ApiError},
    model::{AppState, Todo, TodoService},
};

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    title: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/todos", get(get_all_todos))
        .route("/todos", post(create_todo))
        .route("/todos/{id}", get(get_todo))
        .route("/todos/{id}/complete", patch(complete_todo))
        .route("/todos/{id}", delete(delete_todo))
}

// GET /todos - Now requires authentication
async fn get_all_todos(
    State(todo_service): State<TodoService>,
    auth_user: AuthUser, // Extract user_id from JWT
) -> ApiResult<Json<Vec<Todo>>> {
    let todos = todo_service.get_all_todos(auth_user.user_id).await?;
    Ok(Json(todos))
}

// POST /todos - Requires authentication
async fn create_todo(
    State(todo_service): State<TodoService>,
    auth_user: AuthUser,
    Json(payload): Json<CreateTodoRequest>,
) -> ApiResult<Json<Todo>> {
    if payload.title.trim().is_empty() {
        return Err(ApiError::BadRequest("Title cannot be empty".into()));
    }

    if payload.title.len() > 100 {
        return Err(ApiError::BadRequest(
            "Title must not exceed 100 characters".into(),
        ));
    }

    let todo = todo_service
        .add_todo(auth_user.user_id, payload.title)
        .await?;

    Ok(Json(todo))
}

// GET /todos/:id - User can only get their own todos
async fn get_todo(
    State(todo_service): State<TodoService>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> ApiResult<Json<Todo>> {
    let todo = todo_service.get_todo(id, auth_user.user_id).await?;
    Ok(Json(todo))
}

// PATCH /todos/:id/complete - User can only complete their own todos
async fn complete_todo(
    State(todo_service): State<TodoService>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> ApiResult<Json<Todo>> {
    let todo = todo_service.complete_todo(id, auth_user.user_id).await?;
    Ok(Json(todo))
}

// DELETE /todos/:id - User can only delete their own todos
async fn delete_todo(
    State(todo_service): State<TodoService>,
    auth_user: AuthUser,
    Path(id): Path<i64>,
) -> ApiResult<Json<serde_json::Value>> {
    todo_service.delete_todo(id, auth_user.user_id).await?;
    
    Ok(Json(serde_json::json!({
        "message": "Todo deleted successfully"
    })))
}

// use axum::{
//     extract::{Path, State},
//     routing::{delete, get, patch, post},
//     Json, Router,
// };
// use serde::Deserialize;
// use crate::{
//     error::{Result, TodoAppError},
//     model::{AppState, Todo, TodoService},
// };

// #[derive(Deserialize)]
// pub struct CreateTodoRequest {
//     title: String,
// }

// pub fn routes() -> Router<AppState> {
//     Router::new()
//         .route("/todos", get(get_all_todos))
//         .route("/todos", post(create_todo))
//         .route("/todos/{id}", get(get_todo))
//         .route("/todos/{id}/complete", patch(complete_todo))
//         .route("/todos/{id}", delete(delete_todo))
// }

// // GET /todos
// async fn get_all_todos(State(todo_service): State<TodoService>) -> Result<Json<Vec<Todo>>> {
//     // no map_err here — returns Vec<Todo> directly
//     let todos = todo_service.get_all_todos().await;
//     Ok(Json(todos))
// }

// // POST /todos
// async fn create_todo(
//     State(todo_service): State<TodoService>,
//     Json(payload): Json<CreateTodoRequest>,
// ) -> Result<Json<Todo>> {
//     if payload.title.trim().is_empty() {
//         return Err(TodoAppError::MissingField("title".into()));
//     }

//     if payload.title.len() > 100 {
//         return Err(TodoAppError::InvalidInput(
//             "Title must not exceed 100 characters".into(),
//         ));
//     }

//     // no map_err — add_todo returns Todo
//     let todo = todo_service.add_todo(payload.title.clone()).await;

//     Ok(Json(todo))
// }

// // GET /todos/:id
// async fn get_todo(
//     State(todo_service): State<TodoService>,
//     Path(id): Path<u64>,
// ) -> Result<Json<Todo>> {
//     // get_todo likely returns Option<Todo>
//     match todo_service.get_todo(id).await {
//         Some(todo) => Ok(Json(todo)),
//         None => Err(TodoAppError::NotFound(id)),
//     }
// }

// // PATCH /todos/:id/complete
// async fn complete_todo(
//     State(todo_service): State<TodoService>,
//     Path(id): Path<u64>,
// ) -> Result<Json<Todo>> {
//     match todo_service.complete_todo(id).await {
//         Some(todo) => Ok(Json(todo)),
//         None => Err(TodoAppError::NotFound(id)),
//     }
// }

// // DELETE /todos/:id
// async fn delete_todo(
//     State(todo_service): State<TodoService>,
//     Path(id): Path<u64>,
// ) -> Result<Json<Todo>> {
//     match todo_service.delete_todo(id).await {
//         Some(todo) => Ok(Json(todo)),
//         None => Err(TodoAppError::NotFound(id)),
//     }
// }