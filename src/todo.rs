use serde::{Deserialize, Serialize};
use crate::{status::Status};
use crate::storage::{file_storage, storage_trait::Storage};

const storage: file_storage::FileStorage = file_storage::FileStorage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub name: String,
    pub status: Status,
    pub description: String
}

impl Default for Todo {
    fn default() -> Self {
        Todo { name: "sample".to_string(), status: Status::Open, description: "".to_string() }
    }
}

impl Todo {
    pub fn new(name: String) -> Todo {
        Todo {
            name,
            ..Todo::default()
        }
    }
}

pub fn get_todos() -> Result<Vec<Todo>, String> {
    storage.get_todos()
}

pub fn get_todo(name: &str) -> Result<Todo, String> {
    storage.get_todo(name)
}

pub fn update_todo(name: String, new_status: Option<Status>, new_description: Option<String>) -> Result<Todo, String> {
    storage.update_todo(name, new_status, new_description)
}

pub fn delete_all() -> Result<(), String> {
    storage.delete_all()
}

pub fn delete_one(name: &str) -> Result<Vec<Todo>, String> {
    storage.delete_one(name)
}