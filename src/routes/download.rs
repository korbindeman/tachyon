use actix_files::NamedFile;
use actix_web::{HttpRequest, get, http::header::ContentDisposition, web};

use crate::{AppState, services::upload::Upload};

#[get("/download/{id:.*}")]
pub async fn download(req: HttpRequest, data: web::Data<AppState>) -> actix_web::Result<NamedFile> {
    let id: &str = req.match_info().query("id");

    let res = sqlx::query_as::<_, Upload>("SELECT * FROM uploads WHERE id = $1")
        .bind(id)
        .fetch_one(&data.db)
        .await
        .unwrap();

    let file = NamedFile::open(res.path)?;

    let filename = format!("{}.zip", res.name);

    let download_count = res.download_count + 1;
    sqlx::query("UPDATE uploads SET download_count = $1 WHERE id = $2")
        .bind(download_count)
        .bind(id)
        .execute(&data.db)
        .await
        .unwrap();

    Ok(file.set_content_disposition(ContentDisposition::attachment(filename)))
}
