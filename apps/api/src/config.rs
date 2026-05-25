use dotenv::dotenv;
use shared::config::FromEnv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: String,
    pub database_url: String,
    pub auth: auth::Config,
    pub card: card::Config,
}

impl FromEnv for Config {
    fn from_env() -> Result<Self, String> {
        dotenv().ok();

        let auth = auth::Config::from_env()?;
        let card = card::Config::from_env()?;

        Ok(Self {
            port: env::var("PORT").unwrap_or_else(|_| "3000".to_string()),
            database_url: env::var("DATABASE_URL").map_err(|e| e.to_string())?,
            auth,
            card,
        })
    }
}
