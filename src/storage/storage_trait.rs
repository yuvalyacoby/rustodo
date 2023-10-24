use crate::{todo::Todo, status::Status};

pub trait Storage {
    fn get_todos(&self) -> Result<Vec<Todo>, String>;
    fn get_todo(&self, name: &str) -> Result<Todo, String>;
    fn update_todo(&self, name: String, new_status: Option<Status>, new_description: Option<String>) -> Result<Todo, String>;
    fn delete_all(&self) -> Result<(), String>;
    fn delete_one(&self, name: &str) -> Result<Vec<Todo>, String>;
}