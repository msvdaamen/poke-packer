use async_trait::async_trait;
use uuid::Uuid;

use crate::models::{User, UserWithPassword};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Option<User>;
    async fn find_with_password_by_email(&self, email: &str) -> Option<UserWithPassword>;
}
