use crate::repository::models::Role;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct RoleRepository {
    pool: Pool<Postgres>,
}

impl RoleRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_roles(&self) -> Result<Vec<Role>, sqlx::Error> {
        sqlx::query_as::<_, Role>(r#"select id, name, permissions from roles"#)
            .fetch_all(&self.pool)
            .await
    }
}
