use std::sync::Arc;

use async_trait::async_trait;

use crate::{models::UserWithPassword, ports};

pub struct UserAdapter {
    service: Arc<user::Service>,
}

impl UserAdapter {
    pub fn new(service: Arc<user::Service>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl ports::User for UserAdapter {
    async fn find_with_password_by_email(&self, email: &str) -> Option<UserWithPassword> {
        let user = self.service.find_with_password_by_email(email).await;
        user.map(|u| UserWithPassword {
            id: u.id,
            password: u.password,
        })
    }
}
