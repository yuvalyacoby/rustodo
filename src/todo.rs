use serde::{Deserialize, Serialize};
use crate::{status::Status, action::Action, InputParams};
use serde_json::json;
use std::fs;
use std::env;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub name: String,
    pub status: Status,
    pub description: String
}

impl Todo {
    // pub fn new(params: InputParams) -> Result<Todo, String> {
    //     let status = params.new_status.unwrap_or_else(|| Status::Open);
    //     let description = params.new_description.unwrap_or_else(|| String::from(""));
    //     let json_todo = json!(Todo {
    //         name: params.todo_name,
    //         status,
    //         description: description
    //     });
    //     return Ok(Todo {
    //         name: params.todo_name,
    //         status,
    //         description: description
    //     });
    // }
}

pub fn get_db_file() -> fs::File {
    let db_file = env::var("DB_FILE").unwrap_or("dbfile".to_string());

    match fs::metadata(&db_file) {
        Ok(_) => {
            fs::File::open(&db_file).unwrap()
        }
        Err(_) => {
            fs::File::create(db_file).unwrap()
        }
    }
}

pub fn get_todos() -> Result<Vec<Todo>, String> {
    let mut file = get_db_file();
    let mut file_content = String::new();
    let _ = file.read_to_string(&mut file_content).map_err(|e| format!("Failed to read from file {}", e));

    serde_json::from_str(&file_content).map_err(|e| format!("Failed to parse JSON: {}", e).to_string())
}

pub fn get_todo(name: &str) -> Result<Todo, String> {
    let todos = get_todos()?;
    if let Some(todo) = todos.iter().find(|todo| todo.name == name) {
        Ok(todo.clone())
    } else {
        Err("Todo not found".to_string())
    }
}

pub fn update_todo(name: String, new_status: Option<Status>, new_description: Option<String>) -> Result<Todo, String> {
    let db_file = env::var("DB_FILE").unwrap_or("dbfile".to_string());
    let mut existing_todos = get_todos()?;
    if let Some(todo) = existing_todos.iter_mut().find(|todo| todo.name == name) {
        if let Some(status) = new_status {
            todo.status = status;
        }
        if let Some(description) = new_description {
            todo.description = description;
        }
    } else {
        existing_todos.push(Todo {
            name: name.clone(),
            status: new_status.unwrap_or(Status::Open),
            description: new_description.unwrap_or(String::from(""))
        });
    }
    let t = serde_json::to_string::<Vec<Todo>>(&existing_todos).map_err(|e| format!("Failed to stringify new todos: {}", e))?;
    let _ = fs::write(db_file, t);
    get_todo(&name)
}

// Todo { name: "test".to_string(), status: Status::Open, description: "bla bla".to_string()}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_db_file() {
        env::set_var("DB_FILE", "existing_file.txt");
        let file = fs::File::create("existing_file.txt").unwrap();
        dbg!(&file);
        file.sync_all().unwrap();
        let f = get_db_file();
        let file_metadata = f.metadata().unwrap();
        assert!(file_metadata.is_file(), "{}", true);

        fs::remove_file("existing_file.txt").unwrap();
    }
}