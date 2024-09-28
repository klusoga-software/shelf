use crate::api::cargo::models::CrateIndex;
use sqlx::types::Json;

#[derive(sqlx::FromRow)]
pub struct Config {
    pub api: String,
    pub dl: String,
    #[sqlx(rename = "id")]
    pub _id: i32,
    #[sqlx(rename = "repo_id")]
    pub _repo_id: i32,
}

#[derive(sqlx::FromRow)]
pub struct Repo {
    pub id: i32,
    #[sqlx(rename = "name")]
    pub _name: String,
    #[sqlx(rename = "repo_type")]
    pub _repo_type: i32,
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
