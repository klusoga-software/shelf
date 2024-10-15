use crate::repository::models::{Config, Crate, Repo};
use log::trace;
use sqlx::postgres::{PgQueryResult, PgRow};
use sqlx::{Error, Pool, Postgres, Row};

#[derive(Clone)]
pub struct CargoRepository {
    pool: Pool<Postgres>,
}

impl CargoRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn list_crates_for_repo(&self, repo_id: i32) -> Result<Vec<Crate>, Error> {
        sqlx::query_as::<_, Crate>(
            r#"SELECT id, name, path, version, repo_id, index, crate_size from crates where repo_id = $1"#,
        )
        .bind(repo_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn delete_crate(&self, crate_id: &i32) -> Result<PgQueryResult, Error> {
        sqlx::query(r#"delete from crates where id = $1"#)
            .bind(crate_id)
            .execute(&self.pool)
            .await
    }

    pub async fn get_repos(&self) -> Result<Vec<Repo>, Error> {
        sqlx::query_as::<_, Repo>("select id, name, repo_type, public from repos")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn delete_repo(&self, id: &i32) -> Result<PgQueryResult, Error> {
        sqlx::query("delete from repos where id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
    }

    pub async fn create_repo(&self, repo: Repo) -> Result<i32, Error> {
        let row = sqlx::query(
            r#"insert into repos (name, repo_type, public) VALUES ($1, $2, $3) returning id"#,
        )
        .bind(repo.name)
        .bind(repo.repo_type)
        .bind(repo.public)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.get("id"))
    }

    pub async fn create_config(&self, config: Config) -> Result<PgQueryResult, Error> {
        sqlx::query(r#"insert into configs (repo_id, dl, api) values ($1, $2, $3)"#)
            .bind(config.repo_id)
            .bind(config.dl)
            .bind(config.api)
            .execute(&self.pool)
            .await
    }

    pub async fn delete_config(&self, repo_id: &i32) -> Result<PgQueryResult, Error> {
        sqlx::query("delete from configs where repo_id = $1")
            .bind(repo_id)
            .execute(&self.pool)
            .await
    }

    pub async fn get_repo_by_name(&self, name: &str) -> Result<Repo, Error> {
        sqlx::query_as::<_, Repo>(
            r#"select id, name, repo_type, public from repos where name = $1"#,
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_config_by_repo(&self, repo_id: &i32) -> Result<Config, Error> {
        sqlx::query_as::<_, Config>(
            r#"SELECT id, repo_id, dl, api FROM configs WHERE repo_id = $1"#,
        )
        .bind(repo_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn add_index(&self, crate_row: Crate) -> Result<PgRow, Error> {
        sqlx::query(
            r#"
insert into crates (name, repo_id, version, path, index, crate_size) values ($1, $2, $3, $4, $5, $6)
RETURNING id
        "#,
        )
        .bind(crate_row.name)
        .bind(crate_row.repo_id)
        .bind(crate_row.version)
        .bind(crate_row.path)
        .bind(crate_row.index)
        .bind(crate_row.crate_size)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_index_by_name_and_repo_id(
        &self,
        name: &str,
        id: &i32,
    ) -> Result<Vec<Crate>, Error> {
        sqlx::query_as::<_, Crate>(
            r#"select id, name, path, repo_id, version, index, crate_size from crates where name = $1 and repo_id = $2"#,
        )
        .bind(name)
        .bind(id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_index_by_id(&self, id: &i32) -> Result<Crate, Error> {
        sqlx::query_as::<_, Crate>(
            r#"select id, name, path, repo_id, version, index, crate_size from crates where id = $1"#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_index_by_name_id_and_version(
        &self,
        name: &str,
        version: &str,
        id: &i32,
    ) -> Result<Option<Crate>, Error> {
        trace!(
            "Fetch crate: {} for version {} and repo id {}",
            name,
            version,
            id
        );

        sqlx::query_as::<_, Crate>(
            r#"select id, name, path, repo_id, version, index, crate_size from crates where name = $1 and repo_id = $2 and version = $3"#,
        )
            .bind(name)
            .bind(id)
            .bind(version)
            .fetch_optional(&self.pool)
            .await
    }
}
