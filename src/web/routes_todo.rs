use axum::{routing::post, Router};

pub fn routes() -> Router {
    Router::new()
    .route("/api/add_todo",
    post(api_login))
}


async fn api_login() {}