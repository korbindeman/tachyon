use crate::{AppState, services::upload::Upload, utils::get_uploads_dir};
use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse, Responder, get, http::header::ContentDisposition, web};

#[get("/download/{id}")]
pub async fn download(
    id: web::Path<String>,
    data: web::Data<AppState>,
) -> actix_web::Result<NamedFile> {
    let upload = Upload::find_by_id(&data.db, &id)
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("File not found"))?;

    let canonical_path = upload
        .path
        .canonicalize()
        .map_err(|_| actix_web::error::ErrorNotFound("Invalid path"))?;

    let uploads_dir = get_uploads_dir()
        .canonicalize()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Uploads dir missing"))?;

    if !canonical_path.starts_with(&uploads_dir) {
        return Err(actix_web::error::ErrorForbidden("Invalid file path"));
    }

    let filename = upload.as_info().filename;

    let file = NamedFile::open(&canonical_path)
        .map_err(|_| actix_web::error::ErrorNotFound("File not found"))?
        .set_content_type(upload.mime_type.clone())
        .set_content_disposition(ContentDisposition::attachment(filename));

    upload.update_download_count(&data.db).await.unwrap();

    Ok(file)
}

#[get("/info/{id:.*}")]
pub async fn download_info(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let id: &str = req.match_info().query("id");

    let upload = Upload::find_by_id(&data.db, id)
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("File not found"))?;

    Ok(HttpResponse::Ok().json(upload.as_info()))
}
