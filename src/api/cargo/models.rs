use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub dl: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
}
