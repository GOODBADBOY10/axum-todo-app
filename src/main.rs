mod auth;
mod db;
mod error;
mod model;
mod web;

use axum::{routing::get, Router};
use model::AppState;
use tower_http::cors::CorsLayer;
use dotenvy::dotenv;
use std::env;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from `.env`
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:todo.db".into());
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".into());

    // Initialize SQLite database
    let db_pool = db::init_db(&database_url)
        .await
        .expect("❌ Failed to initialize database");

    // Create shared app state
    let app_state = AppState::new(db_pool, jwt_secret);

    // Build routes
    let app = Router::new()
        .route("/", get(|| async { "📝 Todo API Server" }))
        .nest("/api", web::routes(app_state.clone()))
        .layer(CorsLayer::permissive());

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await?;
    println!("🚀 Server running on http://0.0.0.0:9000");
    axum::serve(listener, app).await?;

    Ok(())
}

// mod auth;
// mod db;
// mod error;
// mod model;
// mod web;

// use axum::{routing::get, Router};
// use model::AppState;
// use tower_http::cors::CorsLayer;

// #[tokio::main]
// async fn main() {
//     // Load environment variables
//     dotenvy::dotenv().ok();
//     let database_url = std::env::var("DATABASE_URL")
//         .unwrap_or_else(|_| "sqlite:todo.db".into());
//     let jwt_secret = std::env::var("JWT_SECRET")
//         .unwrap_or_else(|_| "your-secret-key".into());

//     // Initialize database
//     let db_pool = db::init_db(&database_url)
//         .await
//         .expect("Failed to initialize database");

//     // Create shared app state
//     let app_state = AppState::new(db_pool, jwt_secret);
    
//     // Build application with routes
//     let app = Router::new()
//         .route("/", get(|| async { "Todo API Server" }))
//         .nest("/api", web::routes(app_state.clone()))
//         .layer(CorsLayer::permissive());

//     // Run server
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:9000")
//         .await
//         .unwrap();
    
//     println!("🚀 Server running on http://0.0.0.0:9000");
//     axum::serve(listener, app).await.unwrap();
// }
// // use axum::Router;

// // mod error;
// // mod model;
// // mod web;

// // #[tokio::main]
// // async fn main() {
// //     // Create shared app state
// //     let app_state = model::AppState::new();
    
// //     // Build application with routes
// //     let app = Router::new()
// //         .merge(web::routes_todo::routes())
// //         .with_state(app_state);

// //     // Run server
// //     let listener = tokio::net::TcpListener::bind("0.0.0.0:9000")
// //         .await
// //         .unwrap();
    
// //     println!("🚀 Server running on http://0.0.0.0:9000");
// //     axum::serve(listener, app).await.unwrap();
// // }