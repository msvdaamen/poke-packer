use async_trait::async_trait;

use super::{
    models::Card,
    ports::{Handler, Storage},
};

pub struct Core {
    storage: Box<dyn Storage>,
}

impl Core {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl Handler for Core {
    async fn get_cards(&self) -> Vec<Card> {
        self.storage.get_cards().await
    }
}
