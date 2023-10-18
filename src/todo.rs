use serde::{Deserialize, Serialize};
use crate::{status::Status, action::Action, InputParams};
use serde_json::json;
use std::fs;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
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

pub fn get_db_file(todo: Todo) -> Result<fs::File, String> {
    let db_file = env::var("DB_FILE").unwrap_or("db_file".to_string());

    match fs::metadata(&db_file) {
        Ok(_) => {
            fs::File::open(&db_file).map_err(|e| format!("Failed to open db file: {}", e))
        }
        Err(_) => {
            fs::File::create(&db_file).map_err(|e| format!("Failed to create db file: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_db_file() {
        env::set_var("DB_FILE", "existing_file.txt");
        let file = fs::File::create("existing_file.txt").unwrap();
        dbg!(&file);
        file.sync_all().unwrap();
        let result = get_db_file(Todo { name: "test".to_string(), status: Status::Open, description: "bla bla".to_string()});
        assert!(result.is_ok());
        let file_metadata = result.unwrap().metadata().unwrap();
        assert!(file_metadata.is_file(), "{}", true);

        fs::remove_file("existing_file.txt").unwrap();
    }
}