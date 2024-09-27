use crate::repository::models::{Config, Repo};
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct CargoRepository {
    pool: Pool<Postgres>,
}

impl CargoRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_repos(&self) -> Result<Vec<Repo>, sqlx::Error> {
        sqlx::query_as::<_, Repo>("select id, name, repo_type, public from repos")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_repo_by_name(&self, name: &str) -> Result<Repo, sqlx::Error> {
        sqlx::query_as::<_, Repo>("select id, name, repo_type, public from repos where name = $1")
            .bind(name)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_config_by_repo(&self, repo_id: &i32) -> Result<Config, sqlx::Error> {
        sqlx::query_as::<_, Config>("SELECT * FROM configs WHERE repo_id = $1")
            .bind(repo_id)
            .fetch_one(&self.pool)
            .await
    }
}
