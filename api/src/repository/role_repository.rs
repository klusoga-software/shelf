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

#[cfg(test)]
mod role_repository_tests {
    use crate::repository::role_repository::RoleRepository;
    use test_helpers::{build_pool, build_postgres_database, migrate};

    #[tokio::test]
    async fn test_get_roles() {
        let postgres = build_postgres_database().await;

        let pool = build_pool(&postgres).await;

        migrate(&pool).await;

        let role_repository = RoleRepository::new(pool);

        let roles = role_repository.get_roles().await.unwrap();

        assert_eq!(roles.len(), 3);
    }
}
