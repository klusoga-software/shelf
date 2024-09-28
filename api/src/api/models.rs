use crate::repository::models::RepositoryType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<Error>,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub detail: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRepoRequest {
    pub name: String,
    pub repo_type: RepositoryType,
    pub public: bool,
}
