pub mod cargo_repository;
pub mod models;
pub mod role_repository;
pub mod service_accounts_repository;

#[cfg(test)]
mod database_tests {
    use test_helpers::{build_pool, build_postgres_database, migrate};

    #[tokio::test]
    async fn test_migrations() {
        let postgres = build_postgres_database().await;

        let pool = build_pool(&postgres).await;

        migrate(&pool).await;
    }
}
