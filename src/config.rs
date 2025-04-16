use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub database_url: String,
    pub id_length: usize,
    pub payload_limit: usize,
    pub rate_limit_rps: u64,
    pub port: u16,
    pub host: String,
    pub transfers_dir: PathBuf,
}

impl Config {
    pub fn from_env() -> Self {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080);

        Self {
            base_url: env::var("BASE_URL").unwrap_or_else(|_| format!("http://{}:{}", host, port)),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://sqlite.db".to_string()),
            id_length: env::var("ID_LENGTH")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            payload_limit: env::var("PAYLOAD_LIMIT_MB")
                .ok()
                .and_then(|s| s.parse::<usize>().ok())
                .map(|mb| mb * 1024 * 1024)
                .unwrap_or(5 * 1024 * 1024 * 1024),
            rate_limit_rps: env::var("RATE_LIMIT_RPS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            port,
            host,
            transfers_dir: env::var("TRANSFERS_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("transfers")),
        }
    }
}
