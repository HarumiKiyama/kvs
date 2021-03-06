use serde::{Deserialize, Serialize};

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
