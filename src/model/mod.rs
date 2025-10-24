// mod todo;
// mod todo_service;
// mod todo_store;
// mod user;
// mod app_state;

mod todo;
// mod store;
mod service;
mod user;
mod state;

pub use todo::{Todo, CreateTodo, UpdateTodo};
pub use service::TodoService;
// pub use store::TodoStore;  // You can remove this later since we're using DB now
pub use user::{User, CreateUser};
pub use state::AppState;

// pub use todo::{Todo, CreateTodo, UpdateTodo};
// pub use todo_service::TodoService;
// pub use todo_store::TodoStore;
// pub use user::{User, CreateUser};
// pub use app_state::AppState;

// mod todo;
// mod store;
// mod service;
// mod state;

// pub use todo::Todo;
// pub use store::TodoStore;
// pub use service::TodoService;
// pub use state::AppState;