pub mod routes_auth;
pub mod routes_todo;

use axum::{middleware, Router};
use crate::model::AppState;

pub fn routes(state: AppState) -> Router {
    // Public routes - no authentication required
    let public_routes = Router::new()
        .nest("/auth", routes_auth::routes());
    
    // Protected routes - authentication required
    let protected_routes = Router::new()
        .nest("/todos", routes_todo::routes())
        // .nest("/", routes_todo::routes())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            jwt_middleware,
        ));
    
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}

// Middleware to inject JWT secret into request extensions
async fn jwt_middleware(
    axum::extract::State(state): axum::extract::State<AppState>,
    mut req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    req.extensions_mut().insert(state.jwt_secret.clone());
    next.run(req).await
}