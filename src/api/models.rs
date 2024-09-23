use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<Error>,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub detail: String,
}
