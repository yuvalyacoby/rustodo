use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    GetAll,
    GetOne,
    Update,
    DeleteOne,
    DeleteAll
}

impl FromStr for Action {

    fn from_str(s: &str) -> Result<Action, String> {
        match s.to_lowercase().as_str() {
            "getall" => Ok(Action::GetAll),
            "getone" => Ok(Action::GetOne),
            "update" => Ok(Action::Update),
            "deleteone" => Ok(Action::DeleteOne),
            "deleteall" => Ok(Action::DeleteAll),
            _ => Err(format!("{s} not supported action")),
        }
    }

    type Err = String;
}