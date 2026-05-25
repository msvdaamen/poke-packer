use shared::config::FromEnv;

#[derive(Debug, Clone)]
pub struct Config {}

impl FromEnv for Config {
    fn from_env() -> Result<Self, String> {
        Ok(Config {})
    }
}
