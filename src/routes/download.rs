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

    Ok(file.set_content_disposition(ContentDisposition::attachment(filename)))
}
