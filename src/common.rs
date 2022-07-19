use serde::{Deserialize, Serialize};

pub const DEFAULT_IP_ADDR: &str = "127.0.0.1:4000";

#[derive(Debug, Serialize, Deserialize)]
pub enum CliOperation {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}
