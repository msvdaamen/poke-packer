use async_trait::async_trait;

use crate::models::UserWithPassword;
use shared::types::Email;

#[async_trait]
pub trait User: Sync + Send {
    async fn find_with_password_by_email(
        &self,
        email: &Email,
    ) -> Result<Option<UserWithPassword>, Box<dyn std::error::Error>>;
}
