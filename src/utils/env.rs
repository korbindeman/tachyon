use std::path::PathBuf;

pub fn get_host() -> String {
    std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
}

pub fn get_port() -> u16 {
    std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080)
}

pub fn get_base_url() -> String {
    std::env::var("BASE_URL").unwrap()
}

pub fn get_rate_limit_rps() -> u64 {
    std::env::var("RATE_LIMIT_RPS")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .unwrap_or(8080)
}

pub fn get_transfers_dir() -> PathBuf {
    std::env::var("TRANSFERS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("transfers"))
}

pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL").unwrap()
}

pub fn get_id_length() -> usize {
    std::env::var("ID_LENGTH")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .unwrap_or(5)
}

pub fn get_payload_limit() -> usize {
    std::env::var("PAYLOAD_LIMIT_MB")
        .ok()
        .and_then(|val| val.parse::<usize>().ok())
        .unwrap_or(5120)
        * 1024
        * 1024
}
