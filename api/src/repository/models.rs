use crate::api::cargo::models::CrateIndex;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[derive(sqlx::Type, Serialize, Deserialize, Clone)]
#[repr(i32)]
pub enum RepositoryType {
    Cargo = 1,
}

#[derive(sqlx::FromRow)]
pub struct Config {
    pub api: String,
    pub dl: String,
    pub id: Option<i32>,
    pub repo_id: i32,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Repo {
    pub id: Option<i32>,
    pub name: String,
    pub repo_type: RepositoryType,
    pub public: bool,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Crate {
    pub id: Option<i32>,
    pub name: String,
    pub path: String,
    pub version: String,
    pub repo_id: i32,
    pub index: Json<CrateIndex>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct ServiceAccount {
    pub id: Option<i32>,
    pub name: String,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
    pub expires_at: Option<DateTime<chrono::Utc>>,
    pub repo_count: i64,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Role {
    pub id: Option<i32>,
    pub name: String,
    pub permissions: String,
}
