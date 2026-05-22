use std::path::PathBuf;

use crate::card::{models::Card, ports::Storage};
use async_trait::async_trait;
use serde::Deserialize;
use tokio::fs;
use tokio::task::JoinSet;

pub struct StorageAdapter;

impl StorageAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Storage for StorageAdapter {
    async fn get_cards(&self) -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();
        let mut join_set: JoinSet<Vec<Card>> = JoinSet::new();
        if let Ok(mut result) = fs::read_dir("../../data/cards/en").await {
            while let Ok(Some(entry)) = result.next_entry().await {
                if let Ok(file_type) = entry.file_type().await {
                    if file_type.is_file() {
                        let path = entry.path();
                        // let result = read_cards(path).await;
                        // cards.extend(result);
                        join_set.spawn(read_cards(path));
                    }
                }
            }
        }
        for result in join_set.join_all().await {
            cards.extend(result);
        }
        cards
    }
}

#[derive(Deserialize, Debug)]
struct CardJson {
    id: String,
    name: String,
}

async fn read_cards(path: PathBuf) -> Vec<Card> {
    let mut cards = Vec::new();
    let content = fs::read_to_string(path).await;
    if let Ok(content) = content {
        let result: Result<Vec<CardJson>, serde_json::Error> = serde_json::from_str(&content);
        if let Ok(card_json) = result {
            for card in card_json {
                cards.push(Card::new(card.name));
            }
        }
    }
    cards
}
