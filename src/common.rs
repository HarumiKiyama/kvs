use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub const DEFAULT_IP_ADDR: &str = "127.0.0.1:4000";

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Get { value: String },
    Rm { value: String },
    Set { value: String },
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Response::Rm { value } => write!(f, "{}", value),
            Response::Get { value } => write!(f, "{}", value),
            Response::Set { value } => write!(f, "{}", value),
        }
    }
}
