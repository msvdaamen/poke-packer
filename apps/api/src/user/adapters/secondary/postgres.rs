use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::user::{
    models::{User, UserWithPassword},
    ports::Storage,
};

pub struct PostgresAdapter {
    pool: Pool<Postgres>,
}

impl PostgresAdapter {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Storage for PostgresAdapter {
    async fn get_by_id(&self, id: Uuid) -> Option<User> {
        let result = sqlx::query_as!(
            User,
            "SELECT id, username, email, created_at, updated_at FROM users WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await;
        match result {
            Ok(user) => Some(user),
            _ => None,
        }
    }

    async fn find_with_password_by_email(&self, email: &str) -> Option<UserWithPassword> {
        let result = sqlx::query_as!(
            UserWithPassword,
            "SELECT id, username, email, password, created_at, updated_at FROM users WHERE email = $1",
            email
        )
        .fetch_one(&self.pool)
        .await;
        match result {
            Ok(user) => Some(user),
            _ => None,
        }
    }
}
