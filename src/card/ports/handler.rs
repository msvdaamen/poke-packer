use async_trait::async_trait;

use crate::card::models::Card;

#[async_trait]
pub trait Handler: Send + Sync {
    async fn get_cards(&self) -> Vec<Card>;
}
