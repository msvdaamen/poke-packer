use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    id: uuid::Uuid,
    name: String,
}

impl Card {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            name: name,
        }
    }
}
