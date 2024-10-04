#[cfg(test)]
mod database_tests {
    use sqlx::migrate::Migrator;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::Pool;
    use std::path::Path;
    use testcontainers::runners::AsyncRunner;
    use testcontainers::ContainerAsync;
    use testcontainers_modules::postgres::Postgres;

    async fn build_postgres_database() -> ContainerAsync<Postgres> {
        Postgres::default().start().await.unwrap()
    }

    async fn migrate(db_pool: &Pool<sqlx::Postgres>) {
        let migrator: Migrator = Migrator::new(Path::new("../migrations"))
            .await
            .expect("Failed to migrate");

        migrator
            .run(db_pool)
            .await
            .expect("Failed to run migrations");
    }

    #[tokio::test]
    async fn test_migrations() {
        let postgres = build_postgres_database().await;

        let port = postgres
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to get port");

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(format!("postgres://postgres:postgres@localhost:{}/postgres", port).as_str())
            .await
            .expect("Failed to connect to database");

        migrate(&pool).await;
    }
}
