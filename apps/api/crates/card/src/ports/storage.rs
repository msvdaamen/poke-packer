use async_trait::async_trait;

use crate::models::Card;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn get_cards(&self) -> Vec<Card>;
}
