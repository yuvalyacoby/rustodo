use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    GetAll,
    GetOne(String),
    Update(String),
    DeleteOne(String),
    DeleteAll
}

impl FromStr for Action {

    fn from_str(s: &str) -> Result<Action, String> {
        let parts: Vec<String> = s.split("::").map(|x| x.to_lowercase()).collect();
        match (&parts[0][..], &parts[1][..]) {
            ("getall", _) => Ok(Action::GetAll),
            ("getone", s) if s.len() > 0 => Ok(Action::GetOne(s.to_string())),
            ("getone", _) => Err(format!("{} Todo name is required", s)),
            ("update", s) if s.len() > 0 => Ok(Action::Update(s.to_string())),
            ("update", _) => Err(format!("{} Todo name is required", s)),
            ("deleteone", s) if s.len() > 0 => Ok(Action::DeleteOne(s.to_string())),
            ("deleteone", _) =>Err(format!("{} Todo name is required", s)),
            ("deleteall", _) => Ok(Action::DeleteAll),
            _ => Err(format!("{} not supported action", s)),
        }
    }

    type Err = String;
}