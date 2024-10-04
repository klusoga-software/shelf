#[cfg(test)]
mod database_tests {
    use std::path::Path;
    use sqlx::migrate::Migrator;
    use sqlx::Pool;
    use sqlx::postgres::PgPoolOptions;
    use testcontainers::ContainerAsync;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;

    async fn build_postgres_database() -> ContainerAsync<Postgres> {
        Postgres::default().start().await.unwrap()
    }

    async fn migrate(db_pool: &Pool<sqlx::Postgres>) {
        let migrator: Migrator = Migrator::new(Path::new("../migrations"))
            .await
            .expect("Failed to migrate");

        migrator.run(db_pool).await.expect("Failed to run migrations");
    }

    #[tokio::test]
    async fn test_migrations(){
        let _postgres = build_postgres_database().await;
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect("postgres://postgres:password@localhost/postgres")
            .await
            .expect("Failed to connect to database");

        migrate(&pool).await;
    }
}