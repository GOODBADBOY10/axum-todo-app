use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

use crate::error::{ApiError, ApiResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: i64,
}

impl Claims {
    pub fn new(user_id: i64) -> Self {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        Self {
            sub: user_id,
            exp: expiration,
        }
    }
}

pub fn create_jwt(user_id: i64, secret: &str) -> ApiResult<String> {
    let claims = Claims::new(user_id);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| {
        eprintln!("JWT creation error: {:?}", e);
        ApiError::Internal
    })
}

pub fn verify_jwt(token: &str, secret: &str) -> ApiResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| {
        eprintln!("JWT verification error: {:?}", e);
        ApiError::Auth("Invalid or expired token".into())
    })
}

pub fn hash_password(password: &str) -> ApiResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| {
            eprintln!("Password hashing error: {:?}", e);
            ApiError::Internal
        })
}

pub fn verify_password(password: &str, hash: &str) -> ApiResult<()> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| {
        eprintln!("Password hash parsing error: {:?}", e);
        ApiError::Internal
    })?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::Auth("Invalid credentials".into()))
}

// Extractor for authenticated user
pub struct AuthUser {
    pub user_id: i64,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    fn from_request_parts<'a, 'b>(
        parts: &'a mut Parts,
        _state: &'b S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
    // BoxFuture<'a, Result<Self, Self::Rejection>> {
        Box::pin(async move {
            // Extract Authorization header
            let auth_header = parts
                .headers
                .get("authorization")
                .and_then(|value| value.to_str().ok())
                .ok_or_else(|| ApiError::Auth("Missing authorization header".into()))?;

            // Strip "Bearer " prefix
            let token = auth_header
                .strip_prefix("Bearer ")
                .ok_or_else(|| ApiError::Auth("Invalid authorization header format".into()))?;

            // Get JWT secret from extensions
            let jwt_secret = parts
                .extensions
                .get::<String>()
                .ok_or_else(|| {
                    eprintln!("JWT secret not found in request extensions");
                    ApiError::Internal
                })?;

            // Verify token
            let claims = verify_jwt(token, jwt_secret)?;

            Ok(AuthUser {
                user_id: claims.sub,
            })
        })
    }
}