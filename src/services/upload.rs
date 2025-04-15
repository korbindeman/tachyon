use crate::utils::{extension_to_filetype, get_base_url};
use mime::Mime;
use serde::Serialize;
use sqlx::{SqlitePool, prelude::FromRow};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Upload {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub download_count: u32,
    pub filesize: u32,
    pub mime_type: Mime,
}

#[derive(Serialize)]
pub struct UploadInfo {
    pub id: String,
    pub download_url: String,
    pub name: String,
    pub filename: String,
    pub filetype: String,
    pub filesize: u32,
}

#[derive(FromRow)]
struct UploadRow {
    pub id: String,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub download_count: u32,
    pub filesize: u32,
}

impl UploadRow {
    fn into_upload(self) -> Upload {
        Upload {
            id: self.id,
            name: self.name,
            path: PathBuf::from(self.path),
            mime_type: self.mime_type.parse().unwrap(),
            download_count: self.download_count,
            filesize: self.filesize,
        }
    }
}

impl Upload {
    pub fn new(id: String, name: String, path: PathBuf, filesize: usize, mime_type: Mime) -> Self {
        Self {
            id,
            name,
            path,
            download_count: 0,
            filesize: filesize as u32,
            mime_type,
        }
    }

    pub fn as_info(&self) -> UploadInfo {
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

    pub async fn find_by_id(db: &SqlitePool, id: &str) -> Result<Self, sqlx::Error> {
        let upload_row = sqlx::query_as::<_, UploadRow>("SELECT * FROM uploads WHERE id = $1")
            .bind(id)
            .fetch_one(db)
            .await?;

        Ok(upload_row.into_upload())
    }

    pub async fn insert(&self, db: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO uploads (id, name, path, mime_type, download_count, filesize) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.path.to_string_lossy())
            .bind(&self.mime_type.to_string())
            .bind(self.download_count)
            .bind(self.filesize)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn update_download_count(&self, db: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE uploads SET download_count = download_count + 1 WHERE id = $1")
            .bind(&self.id)
            .execute(db)
            .await?;
        Ok(())
    }
}
