use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Upload {
    pub id: String,
    pub name: String,
    pub path: String,
    pub download_count: u32,
    pub filesize: u32,
}

impl Upload {
    pub fn new(id: String, name: String, path: String, filesize: usize) -> Self {
        Self {
            id,
            name,
            path,
            download_count: 0,
            filesize: filesize as u32,
        }
    }
}
