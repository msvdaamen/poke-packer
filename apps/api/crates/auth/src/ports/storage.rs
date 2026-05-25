use uuid::Uuid;

use async_trait::async_trait;

use crate::models::RefreshToken;

#[async_trait]
pub trait Storage: Sync + Send {
    async fn create_refresh_token(
        &self,
        user_id: &Uuid,
    ) -> Result<RefreshToken, sqlx::error::Error>;

    async fn purge_expired_refresh_tokens(&self) -> Result<u64, sqlx::error::Error>;

    async fn get_refresh_token(
        &self,
        id: &Uuid,
    ) -> Result<Option<RefreshToken>, sqlx::error::Error>;

    async fn delete_refresh_token(&self, id: &Uuid) -> Result<(), sqlx::error::Error>;
}
