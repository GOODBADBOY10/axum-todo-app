use super::TodoService;
use crate::db::DbPool;
use axum::extract::FromRef;

#[derive(Clone)]
pub struct AppState {
    // Database pool
    pub db: DbPool,
    
    // JWT secret for authentication
    pub jwt_secret: String,
    
    // Services
    pub todo_service: TodoService,
}

impl AppState {
    pub fn new(db: DbPool, jwt_secret: String) -> Self {
        Self {
            db: db.clone(),
            jwt_secret,
            todo_service: TodoService::new(db),
        }
    }
}

// FromRef implementations allow handlers to extract services/resources directly
impl FromRef<AppState> for TodoService {
    fn from_ref(state: &AppState) -> Self {
        state.todo_service.clone()
    }
}

impl FromRef<AppState> for DbPool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for String {
    fn from_ref(state: &AppState) -> Self {
        state.jwt_secret.clone()
    }
}

// use super::TodoService;
// use crate::db::DbPool;
// use axum::extract::FromRef;

// #[derive(Clone)]
// pub struct AppState {
//     // Database pool
//     pub db: DbPool,
    
//     // JWT secret for authentication
//     pub jwt_secret: String,
    
//     // Services
//     pub todo_service: TodoService,
// }

// impl AppState {
//     pub fn new(db: DbPool, jwt_secret: String) -> Self {
//         Self {
//             db: db.clone(),
//             jwt_secret,
//             todo_service: TodoService::new(db),
//         }
//     }
// }

// // FromRef implementations allow handlers to extract services directly
// impl FromRef<AppState> for TodoService {
//     fn from_ref(state: &AppState) -> Self {
//         state.todo_service.clone()
//     }
// }

// impl FromRef<AppState> for DbPool {
//     fn from_ref(state: &AppState) -> Self {
//         state.db.clone()
//     }
// }

// use super::TodoService;
// use axum::extract::FromRef;

// #[derive(Clone)]
// pub struct AppState {
//     // add more service here
//     pub todo_service: TodoService,
// }

// impl AppState {
//     pub fn new() -> Self {
//         Self {
//             todo_service: TodoService::new(),
//         }
//     }
// }

// // FromRef implementation allows handlers to extract TodoService directly
// impl FromRef<AppState> for TodoService {
//     fn from_ref(state: &AppState) -> Self {
//         state.todo_service.clone()
//     }
// }