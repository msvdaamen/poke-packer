use std::{error::Error, ops::Add};

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use chrono::Duration;
use jsonwebtoken::{EncodingKey, Header, jws::encode};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    models::{JwtClaims, SignInDto, SignInError, SignInResult},
    ports::{self},
};

pub struct Core {
    secret: String,
    storage: Box<dyn ports::Storage>,
    user: Box<dyn ports::User>,
}

impl Core {
    pub fn new(
        secret: String,
        storage: Box<dyn ports::Storage>,
        user: Box<dyn ports::User>,
    ) -> Self {
        Self {
            secret,
            storage,
            user,
        }
    }

    fn create_token<T: Serialize>(
        &self,
        claims: &T,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let token = encode(
            &Header::default(),
            Some(claims),
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;
        Ok(format!(
            "{}.{}.{}",
            token.protected, token.payload, token.signature
        ))
    }

    async fn create_refresh_token(&self, user_id: &Uuid) -> Result<String, Box<dyn Error>> {
        let refresh_token = self.storage.create_refresh_token(user_id).await?;
        let claims = JwtClaims {
            sub: refresh_token.id.to_string(),
            exp: refresh_token.expires_at.timestamp() as usize,
            iat: refresh_token.created_at.timestamp() as usize,
        };
        let refresh_token = self.create_token(&claims)?;
        Ok(refresh_token)
    }

    fn create_access_token(&self, user_id: &Uuid) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = JwtClaims {
            sub: user_id.to_string(),
            exp: chrono::Utc::now().add(Duration::minutes(5)).timestamp() as usize,
            iat: chrono::Utc::now().timestamp() as usize,
        };
        let access_token = self.create_token(&claims)?;
        Ok(access_token)
    }
}

#[async_trait]
impl ports::Handler for Core {
    async fn sign_in(&self, dto: SignInDto) -> Result<SignInResult, SignInError> {
        let user = self
            .user
            .find_with_password_by_email(&dto.email)
            .await
            .map_err(|_| SignInError::InternalServerError)?;
        let Some(user) = user else {
            return Err(SignInError::UserNotFound);
        };
        let parsed_hash =
            PasswordHash::new(&user.password).map_err(|_| SignInError::InternalServerError)?;
        Argon2::default()
            .verify_password(&dto.password.into_bytes(), &parsed_hash)
            .map_err(|_| SignInError::IncorrectPassword)?;

        let access_token = self
            .create_access_token(&user.id)
            .map_err(|_| SignInError::InternalServerError)?;

        let refresh_token = self
            .create_refresh_token(&user.id)
            .await
            .map_err(|_| SignInError::InternalServerError)?;

        Ok(SignInResult {
            access_token,
            refresh_token,
        })
    }

    async fn purge_refresh_tokens(&self) -> Result<(), Box<dyn Error>> {
        self.storage.purge_expired_refresh_tokens().await?;
        Ok(())
    }
}
