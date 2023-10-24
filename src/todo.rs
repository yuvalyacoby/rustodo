use serde::{Deserialize, Serialize};
use crate::{status::Status};

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