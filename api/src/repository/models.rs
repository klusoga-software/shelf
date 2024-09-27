#[derive(sqlx::FromRow)]
pub struct Config {
    pub api: String,
    pub dl: String,
    pub id: i32,
    pub repo_id: i32,
}

#[derive(sqlx::FromRow)]
pub struct Repo {
    pub id: i32,
    pub name: String,
    pub repo_type: i32,
    pub public: bool,
}
