use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{create_jwt, hash_password, verify_password},
    error::{ApiError, ApiResult},
    model::{AppState, User},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
}

// POST /api/register
async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> ApiResult<Json<AuthResponse>> {
    // Validate input
    if payload.username.trim().is_empty() {
        return Err(ApiError::MissingField("username".into()));
    }

    if payload.password.is_empty() {
        return Err(ApiError::MissingField("password".into()));
    }

    if payload.password.len() < 6 {
        return Err(ApiError::BadRequest(
            "Password must be at least 6 characters".into(),
        ));
    }

    if payload.username.len() < 3 {
        return Err(ApiError::BadRequest(
            "Username must be at least 3 characters".into(),
        ));
    }

    // Hash password
    let password_hash = hash_password(&payload.password)?;

    // Create user in database
    let user = User::create(&state.db, &payload.username, &password_hash)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.message().contains("UNIQUE") => {
                ApiError::BadRequest("Username already exists".into())
            }
            _ => ApiError::Database(e),
        })?;

    // Generate JWT token
    let token = create_jwt(user.id, &state.jwt_secret)?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
        },
    }))
}

// POST /api/login
async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> ApiResult<Json<AuthResponse>> {
    // Validate input
    if payload.username.trim().is_empty() || payload.password.is_empty() {
        return Err(ApiError::BadRequest(
            "Username and password are required".into(),
        ));
    }

    // Find user by username
    let user = User::find_by_username(&state.db, &payload.username)
        .await
        .map_err(|_| ApiError::Auth("Invalid username or password".into()))?;

    // Verify password
    verify_password(&payload.password, &user.password_hash)?;

    // Generate JWT token
    let token = create_jwt(user.id, &state.jwt_secret)?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
        },
    }))
}