use crate::api::cargo::models::CrateIndex;
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
    #[sqlx(rename = "id")]
    pub _id: i32,
    #[sqlx(rename = "repo_id")]
    pub _repo_id: i32,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Repo {
    pub id: Option<i32>,
    pub name: String,
    pub repo_type: RepositoryType,
    pub public: bool,
}

#[derive(sqlx::FromRow)]
pub struct Crate {
    #[sqlx(rename = "id")]
    pub _id: Option<i32>,
    pub name: String,
    pub path: String,
    pub version: String,
    pub repo_id: i32,
    pub index: Json<CrateIndex>,
}
