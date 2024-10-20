use crate::repository::models::Dashboard;
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, Pool, Postgres};

#[derive(Clone)]
pub struct DashboardsRepository {
    pool: Pool<Postgres>,
}

impl DashboardsRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn set_dashboard(&self, dashboard: Dashboard) -> Result<PgQueryResult, Error> {
        sqlx::query(r#"insert into dashboards (user_id, tiles) values ($1, $2) on conflict (user_id) do update set tiles = excluded.tiles"#)
            .bind(dashboard.user_id)
            .bind(dashboard.tiles)
            .execute(&self.pool)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::dashboards_repository::DashboardsRepository;
    use crate::repository::models::{Dashboard, DashboardTile, TileType};
    use sqlx::types::Json;
    use test_helpers::{build_pool, build_postgres_database, migrate};

    #[tokio::test]
    async fn create_dashboard_test() {
        let postgres = build_postgres_database().await;

        let pool = build_pool(&postgres).await;

        let repo = DashboardsRepository::new(pool.clone());

        migrate(&pool).await;

        repo.set_dashboard(Dashboard {
            id: None,
            user_id: "001".to_owned(),
            tiles: Json(vec![DashboardTile {
                header: "Test".to_owned(),
                tile_type: TileType::Count,
                data: "1".to_owned(),
            }]),
        })
        .await
        .expect("Create dashboard failed");
    }
}
