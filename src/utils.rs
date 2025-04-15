use actix_web::{HttpRequest, HttpResponse, Result};
use nanoid::nanoid;
use std::{ffi::OsStr, path::PathBuf};

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

pub fn get_port() -> u16 {
    std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080)
}

pub fn get_base_url() -> String {
    std::env::var("BASE_URL").unwrap()
}

pub fn check_api_key(req: &HttpRequest) -> Result<(), HttpResponse> {
    let expected_key = std::env::var("API_KEY").expect("API_KEY not set");
    let header = req.headers().get("x-api-key").and_then(|h| h.to_str().ok());

    if header != Some(expected_key.as_str()) {
        return Err(HttpResponse::Unauthorized().body("Missing or invalid API key"));
    }

    Ok(())
}

pub fn extension_to_filetype(extension: &OsStr) -> String {
    let ext = extension.to_str().unwrap_or("").to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" => "JPEG Image".to_string(),
        "png" => "PNG Image".to_string(),
        "gif" => "GIF Image".to_string(),
        "pdf" => "PDF Document".to_string(),
        "txt" => "Text File".to_string(),
        "doc" | "docx" => "Word Document".to_string(),
        "xls" | "xlsx" => "Excel Spreadsheet".to_string(),
        "ppt" | "pptx" => "PowerPoint Presentation".to_string(),
        "zip" => "Zip Archive".to_string(),
        "rar" => "RAR Archive".to_string(),
        "mp3" => "MP3 Audio".to_string(),
        "mp4" => "MP4 Video".to_string(),
        "html" | "htm" => "HTML Document".to_string(),
        "css" => "CSS Stylesheet".to_string(),
        "js" => "JavaScript File".to_string(),
        "json" => "JSON File".to_string(),
        "xml" => "XML Document".to_string(),
        "csv" => "CSV File".to_string(),
        _ => format!("{} File", ext.to_uppercase()),
    }
}
