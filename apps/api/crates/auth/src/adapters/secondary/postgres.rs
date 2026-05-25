use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{models::RefreshToken, ports};

pub struct PostgresAdapter {
    pool: Pool<Postgres>,
}

impl PostgresAdapter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ports::Storage for PostgresAdapter {
    async fn create_refresh_token(
        &self,
        user_id: &Uuid,
    ) -> Result<RefreshToken, sqlx::error::Error> {
        let token = sqlx::query_as!(
            RefreshToken,
            "insert into refresh_tokens (id, user_id, created_at, expires_at) values ($1, $2, $3, $4) returning id, user_id, created_at, expires_at",
            Uuid::new_v4(),
            user_id.clone(),
            chrono::Utc::now(),
            chrono::Utc::now() + chrono::Duration::weeks(5),
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(token)
    }

    async fn purge_expired_refresh_tokens(&self) -> Result<u64, sqlx::error::Error> {
        let result = sqlx::query("DELETE FROM refresh_tokens WHERE expires_at <= NOW()")
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    async fn get_refresh_token(
        &self,
        id: &Uuid,
    ) -> Result<Option<RefreshToken>, sqlx::error::Error> {
        let result = sqlx::query_as!(
            RefreshToken,
            "select id, user_id, created_at, expires_at from refresh_tokens where id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn delete_refresh_token(&self, id: &Uuid) -> Result<(), sqlx::error::Error> {
        sqlx::query("DELETE FROM refresh_tokens WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
