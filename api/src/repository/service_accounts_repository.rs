use crate::repository::models::ServiceAccount;
use sqlx::{Error, Pool, Postgres};

#[derive(Clone)]
pub struct ServiceAccountsRepository {
    pool: Pool<Postgres>,
}

impl ServiceAccountsRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn list_service_accounts(&self) -> Result<Vec<ServiceAccount>, Error> {
        sqlx::query_as::<_, ServiceAccount>(
            r#"select sa.id,
       sa.name,
       sa.created_at,
       sa.updated_at,
       sa.expires_at,
       count(r.id) as repo_count
from service_accounts sa
left join
    service_accounts_repos sar on sa.id = sar.service_account_id
left join repos r on sar.repo_id = r.id
group by sa.id"#,
        )
        .fetch_all(&self.pool)
        .await
    }
}
