// use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Todo {
//     id: u64,
//     title: String,
//     completed: bool,
// }


// impl Todo {
//     pub fn new(id: u64, title: impl Into<String>) -> Self {
//         Self { 
//             id,
//             title: title.into(), 
//             completed: false, 
//         }
//     }
// }

// pub struct TodoStore {
//     todos: Vec<Todo>,
//     next_id: u64,
// }

// impl TodoStore {
//     pub fn new() -> Self {
//         Self {
//             todos: Vec::new(),
//             next_id: 1,
//         }
//     }

//      pub fn get_all_todos(&self) -> &Vec<Todo> {
//         &self.todos
//     }

//     pub fn add_todo(&mut self, title: impl Into<String>) -> &Todo {
//         let todo = Todo::new(self.next_id, title);
//         self.todos.push(todo);
//         self.next_id += 1;
//         self.todos.last().unwrap()
//     }

//     // pub fn get_todo(&self, id: u64) -> Option<&Todo> {
//     //     self.todos.iter().find(|todo| todo.id == id)
//     // }

//     pub fn complete_todo(&mut self, id: u64) -> Option<&mut Todo> {
//         if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == id) {
//             todo.completed = true;
//             Some(todo)
//         } else {
//             None
//         }
//     }

//     pub fn delete_todo(&mut self, id: u64) -> Option<Todo> {
//         if let Some(pos) = self.todos.iter().position(|todo| todo.id == id) {
//             Some(self.todos.remove(pos))
//         } else {
//             None
//         }
//     }

// }