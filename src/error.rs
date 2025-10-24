use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

pub type ApiResult<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Error)]
pub enum ApiError {
    // --- Client-side errors ---
    #[error("Todo item with ID {0} was not found")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    BadRequest(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    // --- Authentication errors ---
    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Unauthorized access")]
    Unauthorized,

    // --- Server-side / internal errors ---
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("An unexpected internal error occurred")]
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        eprintln!("❌ ERROR: {:?}", self);

        let (status, message) = match &self {
            ApiError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),

            ApiError::BadRequest(_) | ApiError::MissingField(_) => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }

            ApiError::Auth(_) | ApiError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, self.to_string())
            }

            ApiError::Database(e) => {
                // Log the actual database error internally
                eprintln!("Database error details: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error occurred".to_string(),
                )
            }

            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        // Build JSON response body
        let body = Json(json!({
            "success": false,
            "error": message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

// use axum::{
//     Json,
//     http::StatusCode,
//     response::{IntoResponse, Response},
// };
// use serde_json::json;
// use std::io;
// use thiserror::Error;

// pub type Result<T> = std::result::Result<T, TodoAppError>;

// #[derive(Debug, Error)]
// pub enum TodoAppError {
//     // --- Client-side errors ---
//     #[error("Todo item with ID {0} was not found")]
//     NotFound(u64),

//     #[error("Invalid input: {0}")]
//     InvalidInput(String),

//     #[error("Missing required field: {0}")]
//     MissingField(String),

//     // --- Server-side / internal errors ---
//     #[error("Database connection failed: {0}")]
//     Database(#[from] io::Error),

//     // #[error("An unexpected internal error occurred")]
//     // InternalError,
// }

// impl IntoResponse for TodoAppError {
//     fn into_response(self) -> Response {
//         let (status, message) = match &self {
//             TodoAppError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),

//             TodoAppError::InvalidInput(_) | TodoAppError::MissingField(_) => {
//                 (StatusCode::BAD_REQUEST, self.to_string())
//             }

//             TodoAppError::Database(_) => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 "Database connection failed".to_string(),
//             ),

//             // TodoAppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
//         };

//         // Build JSON response body
//         let body = Json(json!({
//             "success": false,
//             "error": message,
//             "status": status.as_u16(),
//         }));

//         (status, body).into_response()

//         // (status, Json(json!({ "error": message }))).into_response()
//     }
// }

// // use axum::{
// //     http::StatusCode,
// //     response::{IntoResponse, Response},
// //     Json,
// // };
// // use serde_json::json;
// // use thiserror::Error;
// // use std::io;

// // pub type Result<T> = core::result::Result<T, TodoAppError>;

// // #[derive(Debug, Error)]
// // pub enum TodoAppError {
// //     // --- Client-side errors ---
// //     #[error("Todo item with ID {0} was not found")]
// //     NotFound(u64),

// //     #[error("Invalid input: {0}")]
// //     InvalidInput(String),

// //     #[error("Missing required field: {0}")]
// //     MissingField(String),

// //     // --- Server-side / internal errors ---
// //     #[error("Database connection failed: {0}")]
// //     Database(#[from] io::Error),

// //     #[error("Failed to parse data for key `{0}`")]
// //     DataParseError(String),

// //     #[error("An unexpected internal error occurred")]
// //     Internal,
// // }

// // impl IntoResponse for TodoAppError {
// //     fn into_response(self) -> Response {
// //         eprintln!("❌ ERROR: {:?}", self);

// //         // Match each error type to an HTTP status and user-facing message
// //         let (status, message) = match &self {

// //             TodoAppError::NotFound(_) => (
// //                 StatusCode::NOT_FOUND,
// //                 self.to_string()
// //             ),

// //             TodoAppError::InvalidInput(_) | TodoAppError::MissingField(_) => {
// //                 (
// //                     StatusCode::BAD_REQUEST,
// //                     self.to_string()
// //                 )
// //             },

// //             TodoAppError::Database(_) => (
// //                 StatusCode::INTERNAL_SERVER_ERROR,
// //                 "Database connection failed".to_string(),
// //             ),

// //             TodoAppError::DataParseError(_) => (
// //                 StatusCode::UNPROCESSABLE_ENTITY,
// //                 "Failed to process stored data".to_string(),
// //             ),

// //             TodoAppError::Internal => (
// //                 StatusCode::INTERNAL_SERVER_ERROR,
// //                 "Internal server error".to_string(),
// //             ),

// //         };

// //         // Build JSON response body
// //         let body = Json(json!({
// //             "success": false,
// //             "error": message,
// //             "status": status.as_u16(),
// //         }));

// //         (status, body).into_response()
// //     }
// // }
