use crate::repository::models::RepositoryType;
use chrono::{DateTime, Utc};
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

#[derive(Serialize, Deserialize)]
pub struct CreateServiceAccount {
    pub name: String,
    pub expired_at: Option<DateTime<Utc>>,
    pub repo_list: Vec<(i32, i32)>,
}
