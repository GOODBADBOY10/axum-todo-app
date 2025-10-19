use axum::{
    routing::post,
    Router,
    response::IntoResponse,
    http::StatusCode,
};

pub fn routes() -> Router {
    Router::new()
        .route("/api/todo", post(api_todo))
}

async fn api_todo() -> impl IntoResponse {
    println!("✅ TODO endpoint hit");
    (StatusCode::OK, "Todo endpoint reached")
}