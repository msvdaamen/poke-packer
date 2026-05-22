use std::sync::Arc;

use crate::user::{models::UserWithPassword, ports::Handler};

pub struct Service {
    core: Arc<dyn Handler>,
}

impl Service {
    pub fn new(core: Arc<dyn Handler>) -> Self {
        Self { core }
    }

    pub async fn find_with_password_by_email(&self, email: &str) -> Option<UserWithPassword> {
        self.core.find_with_password_by_email(email).await
    }
}
