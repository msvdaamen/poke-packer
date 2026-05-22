use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::user::models::User;
use crate::user::models::UserWithPassword;
use crate::user::ports::Handler;
use crate::user::ports::Storage;

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
    async fn get_by_id(&self, id: Uuid) -> Option<User> {
        self.storage.get_by_id(id).await
    }

    async fn find_with_password_by_email(&self, email: &str) -> Option<UserWithPassword> {
        self.storage.find_with_password_by_email(email).await
    }
}
