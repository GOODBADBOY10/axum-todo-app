// use super::Todo;

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

//     pub fn get_all_todos(&self) -> Vec<Todo> {
//         self.todos.clone()
//     }

//     pub fn add_todo(&mut self, title: impl Into<String>) -> Todo {
//         let todo = Todo::new(self.next_id, title);
//         self.todos.push(todo.clone());
//         self.next_id += 1;
//         todo
//     }

//     pub fn get_todo(&self, id: u64) -> Option<Todo> {
//         self.todos.iter().find(|todo| todo.id == id).cloned()
//     }

//     pub fn complete_todo(&mut self, id: u64) -> Option<Todo> {
//         if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == id) {
//             todo.completed = true;
//             Some(todo.clone())
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