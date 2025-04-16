pub mod env;
pub mod format;
pub mod id;

use actix_web::{HttpRequest, HttpResponse, Result};

pub fn check_api_key(req: &HttpRequest) -> Result<(), HttpResponse> {
    let expected_key = std::env::var("API_KEY").expect("API_KEY not set");
    let header = req.headers().get("x-api-key").and_then(|h| h.to_str().ok());

    if header != Some(expected_key.as_str()) {
        return Err(HttpResponse::Unauthorized().body("Missing or invalid API key"));
    }

    Ok(())
}
