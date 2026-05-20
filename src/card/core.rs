use std::sync::Arc;

use async_trait::async_trait;

use crate::card::{
    models::Card,
    ports::{Handler, Storage},
};

pub struct Core {
    storage: Arc<dyn Storage>,
}

impl Core {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl Handler for Core {
    async fn get_cards(&self) -> Vec<Card> {
        self.storage.get_cards().await
    }
}
