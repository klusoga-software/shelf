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
