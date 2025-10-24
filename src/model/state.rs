use super::TodoService;
use axum::extract::FromRef;

#[derive(Clone)]
pub struct AppState {
    pub todo_service: TodoService,
    // Add more services here as your app grows
    // pub user_service: UserService,
    // pub auth_service: AuthService,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            todo_service: TodoService::new(),
        }
    }
}

// FromRef implementation allows handlers to extract TodoService directly
impl FromRef<AppState> for TodoService {
    fn from_ref(state: &AppState) -> Self {
        state.todo_service.clone()
    }
}