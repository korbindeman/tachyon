use crate::{AppState, services::upload::Upload, utils::get_uploads_dir};
use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse, Responder, get, http::header::ContentDisposition, web};
use mime::Mime;
use std::path::PathBuf;

#[get("/download/{id:.*}")]
pub async fn download(req: HttpRequest, data: web::Data<AppState>) -> actix_web::Result<NamedFile> {
    let id: &str = req.match_info().query("id");

    let res = sqlx::query_as::<_, Upload>("SELECT * FROM uploads WHERE id = $1")
        .bind(id)
        .fetch_one(&data.db)
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("File not found"))?;

    let file_path = PathBuf::from(&res.path);
    let canonical_path = file_path
        .canonicalize()
        .map_err(|_| actix_web::error::ErrorNotFound("Invalid path"))?;

    let uploads_dir = get_uploads_dir()
        .canonicalize()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Uploads dir missing"))?;

    if !canonical_path.starts_with(&uploads_dir) {
        return Err(actix_web::error::ErrorForbidden("Invalid file path"));
    }

    let path_buf = PathBuf::from(&res.path);
    let extension = path_buf.extension().unwrap().to_str().unwrap();

    let sanitized_name = res.name.replace(
        |c: char| !c.is_ascii() || c == '"' || c == '\\' || c == '/',
        "_",
    ); // TODO: I don't know if this is necessary
    let filename = format!("{}.{}", sanitized_name, extension);

    let mime_type = res.mime_type.parse::<Mime>().unwrap();

    let file = NamedFile::open(&canonical_path)
        .map_err(|_| actix_web::error::ErrorNotFound("File not found"))?
        .set_content_type(mime_type)
        .set_content_disposition(ContentDisposition::attachment(filename));

    sqlx::query("UPDATE uploads SET download_count = download_count + 1 WHERE id = $1")
        .bind(id)
        .execute(&data.db)
        .await
        .map_err(|e| {
            println!("DB update error: {:?}", e);
            actix_web::error::ErrorInternalServerError("Failed to update count")
        })?;

    Ok(file)
}

#[get("/info/{id:.*}")]
pub async fn download_info(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let id: &str = req.match_info().query("id");

    let res = sqlx::query_as::<_, Upload>("SELECT * FROM uploads WHERE id = $1")
        .bind(id)
        .fetch_one(&data.db)
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("Upload not found"))?;

    Ok(HttpResponse::Ok().json(res.info()))
}
