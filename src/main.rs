use axum::{Router};

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() {
    // build our application
    let app = Router::new()
    .merge(web::routes_todo::routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}