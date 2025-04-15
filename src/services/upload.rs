use crate::utils::{extension_to_filetype, get_base_url};
use serde::Serialize;
use sqlx::prelude::FromRow;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct UploadInfo {
    pub id: String,
    pub download_url: String,
    pub name: String,
    pub filename: String,
    pub filetype: String,
    pub filesize: u32,
}

#[derive(Debug, FromRow)]
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

    pub fn info(&self) -> UploadInfo {
        let filename = format!(
            "{}.{}",
            &self.name,
            PathBuf::from(&self.path)
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
        );

        let filetype = extension_to_filetype(PathBuf::from(&self.path).extension().unwrap());

        UploadInfo {
            id: self.id.clone(),
            name: self.name.clone(),
            download_url: format!("{}/download/{}", get_base_url(), self.id),
            filename,
            filetype,
            filesize: self.filesize,
        }
    }
}
