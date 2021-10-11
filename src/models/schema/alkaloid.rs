use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Alkaloid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subclass: Option<String>,
}
