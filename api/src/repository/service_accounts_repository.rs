use crate::api::models::CreateServiceAccount;
use crate::repository::models::ServiceAccount;
use sqlx::{Error, Pool, Postgres, Row};

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

    pub async fn create_service_account(
        &self,
        account: CreateServiceAccount,
        secret: &String,
    ) -> Result<(), Error> {
        let tx = self.pool.begin().await?;

        let result = sqlx::query(
            r#"insert into service_accounts (name, expires_at, key, secret)
values ($1, $2, $3, $4) returning id"#,
        )
        .bind(account.name)
        .bind(account.expired_at)
        .bind(account.key)
        .bind(secret)
        .fetch_one(&self.pool)
        .await?;

        for repo in account.repo_list {
            let account_id: i32 = result.get("id");

            sqlx::query(r#"insert into service_accounts_repos (repo_id, service_account_id, role_id) VALUES ($1, $2, $3)"#)
                .bind(repo.0)
                .bind(account_id)
                .bind(repo.1)
                .execute(&self.pool).await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
