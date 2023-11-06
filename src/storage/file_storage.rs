use crate::storage::storage_trait::Storage;
use crate::todo::Todo;
use crate::{status::Status};
use std::fs;
use std::env;
use std::io::Read;

pub struct FileStorage;

impl Storage for FileStorage {
    fn get_todos(&self) -> Result<Vec<Todo>, String> {
        let mut file = get_db_file();
        let mut file_content = String::new();
        let _ = file.read_to_string(&mut file_content).map_err(|e| format!("Failed to read from file {}", e));

        serde_json::from_str(&file_content).map_err(|e| format!("Failed to parse JSON: {}", e).to_string())
    }

    fn get_todo(&self, name: &str) -> Result<Todo, String> {
        let todos = self.get_todos()?;
        if let Some(todo) = todos.iter().find(|todo| todo.name == name) {
            Ok(todo.clone())
        } else {
            Err("Todo not found".to_string())
        }
    }

    fn update_todo(&self, name: String, new_status: Option<Status>, new_description: Option<String>) -> Result<Todo, String> {
        let db_file = env::var("DB_FILE").unwrap_or("dbfile".to_string());
        let mut existing_todos = self.get_todos()?;
        if let Some(todo) = existing_todos.iter_mut().find(|todo| todo.name == name) {
            if let Some(status) = new_status {
                todo.status = status;
            }
            if let Some(description) = new_description {
                todo.description = description;
            }
        } else {
            existing_todos.push(Todo::new(name.clone()));
        }
        let t = serde_json::to_string::<Vec<Todo>>(&existing_todos).map_err(|e| format!("Failed to stringify new todos: {}", e))?;
        let _ = fs::write(db_file, t);
        self.get_todo(&name)
    }

    fn delete_all(&self) -> Result<(), String> {
        let db_file = env::var("DB_FILE").unwrap_or("dbfile".to_string());
        let _ = fs::write(db_file, String::from("[]")).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn delete_one(&self, name: &str) -> Result<Vec<Todo>, String> {
        let _ = self.get_todo(&name)?;
        let existing_todos = self.get_todos()?;
        let new_todos: Vec<Todo> = existing_todos.into_iter().filter(|todo| todo.name != name).collect();
        let db_file = env::var("DB_FILE").unwrap_or("dbfile".to_string());
        let t = serde_json::to_string::<Vec<Todo>>(&new_todos).map_err(|e| format!("Failed to stringify new todos: {}", e))?;
        let _ = fs::write(db_file, t).map_err(|e| e.to_string())?;
        Ok(new_todos)
    }
}

fn get_db_file() -> fs::File {
    let db_file = env::var("DB_FILE").unwrap_or("dbfile".to_string());

    match fs::metadata(&db_file) {
        Ok(_) => {
            fs::File::open(&db_file).unwrap()
        }
        Err(_) => {
            fs::File::create(&db_file).unwrap();
            let _ = fs::write(&db_file, "[]");
            fs::File::open(&db_file).unwrap()
        }
    }
}