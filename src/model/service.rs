// ============================================
// src/model/service.rs
// ============================================
use super::{Todo, TodoStore};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct TodoService {
    store: Arc<RwLock<TodoStore>>,
}

impl TodoService {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(TodoStore::new())),
        }
    }

    pub async fn get_all_todos(&self) -> Vec<Todo> {
        let store = self.store.read().await;
        store.get_all_todos()
    }

    pub async fn add_todo(&self, title: impl Into<String>) -> Todo {
        let mut store = self.store.write().await;
        store.add_todo(title)
    }

    pub async fn get_todo(&self, id: u64) -> Option<Todo> {
        let store = self.store.read().await;
        store.get_todo(id)
    }

    pub async fn complete_todo(&self, id: u64) -> Option<Todo> {
        let mut store = self.store.write().await;
        store.complete_todo(id)
    }

    pub async fn delete_todo(&self, id: u64) -> Option<Todo> {
        let mut store = self.store.write().await;
        store.delete_todo(id)
    }
}