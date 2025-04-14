use actix_web::{HttpRequest, HttpResponse, Result};
use nanoid::nanoid;
use std::path::PathBuf;

pub const KILOBYTE: usize = 1024;
pub const MEGABYTE: usize = usize::pow(KILOBYTE, 2);
pub const GIGABYTE: usize = usize::pow(MEGABYTE, 2);

const ID_LENGTH: usize = 5;

pub fn create_id() -> String {
    nanoid!(ID_LENGTH)
}

pub fn get_uploads_dir() -> PathBuf {
    std::env::var("UPLOADS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("uploads"))
}

pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL").unwrap()
}

pub fn check_api_key(req: &HttpRequest) -> Result<(), HttpResponse> {
    let expected_key = std::env::var("API_KEY").expect("API_KEY not set");
    let header = req.headers().get("x-api-key").and_then(|h| h.to_str().ok());

    if header != Some(expected_key.as_str()) {
        return Err(HttpResponse::Unauthorized().body("Missing or invalid API key"));
    }

    Ok(())
}
