use std::env;

use shared::config::FromEnv;

#[derive(Debug, Clone)]
pub struct Config {
    pub secret: String,
    pub grpc_url: String,
}

impl FromEnv for Config {
    fn from_env() -> Result<Self, String> {
        Ok(Self {
            secret: env::var("AUTH_SECRET_KEY").map_err(|_| "AUTH_SECRET_KEY is not set")?,
            grpc_url: env::var("GRPC_URL").map_err(|_| "GRPC_URL is not set")?,
        })
    }
}
