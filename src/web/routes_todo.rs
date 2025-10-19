use axum::{routing::post, Router};

pub fn routes() -> Router {
    Router::new()
    .route("/api/todo",
    post(api_todo))
}


async fn api_todo() {}