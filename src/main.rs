mod auth;
mod db;
mod error;
mod model;
mod web;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use model::AppState;
use std::env;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env
    dotenv().ok();
    tracing_subscriber::fmt::init();

    tracing::info!("Loading environment variables...");

    let database_url = env::var("DATABASE_URL")?;
    let jwt_secret = env::var("JWT_SECRET")?;

    // Initialize SQLite database
    // let db_pool = db::init_db(&database_url).await?;
    tracing::info!("Connecting to database...");
    let db_pool = match db::init_db(&database_url).await {
        Ok(pool) => {
            tracing::info!("Database connected successfully!");
            pool
        }
        Err(e) => {
            tracing::error!("Database error: {}", e);
            return Err(e.into());
        }
    };
    // Create shared app state
    let app_state = AppState::new(db_pool, jwt_secret);

    // Build routes
    let app = Router::new()
        .route("/", get(|| async { "Todo API Server" }))
        .nest("/api", web::routes(app_state.clone()))
        .layer(CorsLayer::permissive());

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await?;
    tracing::info!("Server running on http://0.0.0.0:9000");
    // println!("Server running on http://0.0.0.0:9000");
    axum::serve(listener, app).await?;

    Ok(())
}

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
