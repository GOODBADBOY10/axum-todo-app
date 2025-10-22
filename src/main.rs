use std::sync::{Arc, Mutex};

use axum::{Router};


mod error;
mod model;
mod web;

#[tokio::main]
async fn main() {

    // Shared todo store
    let store = Arc::new(Mutex::new(model::TodoStore::new()));
    
    // build our application
    let app = Router::new()
    .merge(web::routes_todo::routes())
    .with_state(store.clone());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}