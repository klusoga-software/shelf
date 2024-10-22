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

    pub async fn get_dashboard_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Option<Dashboard>, Error> {
        match sqlx::query_as::<_, Dashboard>(
            r#"select id, user_id, tiles from dashboards where user_id = $1"#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        {
            Ok(tiles) => Ok(Some(tiles)),
            Err(err) => match err {
                Error::RowNotFound => Ok(None),
                _ => Err(err),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::dashboards_repository::DashboardsRepository;
    use crate::repository::models::{Dashboard, DashboardTile};
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
                id: "Test".to_string(),
                row_span: None,
                column_span: None,
                column_offset: None,
            }]),
        })
        .await
        .expect("Create dashboard failed");
    }
}
