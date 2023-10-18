use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Open,
    InProgress,
    Done
}

impl FromStr for Status {

    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "open" => Ok(Status::Open),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(format!("{s} not supported status")),
        }
    }

    type Err = String;
}