use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub ui: UiConfiguration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfiguration {
    authority: String,
}
