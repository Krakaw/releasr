use crate::errors::ReleasrError;
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Config {
    pub listen: SocketAddr,
    pub db_path: String,
}

impl Config {
    pub fn init() -> Result<Config, ReleasrError> {
        dotenv().ok();
        let config = Config {
            listen: env::var("LISTEN")
                .unwrap_or_else(|| "0.0.0.0:8080".to_string())
                .parse()?,
            db_path: env::var("DB_PATH").unwrap_or_else(|| "./db.sqlite".to_string()),
        };
        Ok(config)
    }
}
