use crate::{AppState, models::transfer::Transfer};
use actix_files::NamedFile;
use actix_web::{HttpResponse, Responder, get, http::header::ContentDisposition, web};

#[get("/download/{id}")]
pub async fn download(
    id: web::Path<String>,
    data: web::Data<AppState>,
) -> actix_web::Result<NamedFile> {
    let transfer = Transfer::find_by_id(&data.db, &id)
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("File not found"))?;

    let canonical_path = transfer
        .path
        .canonicalize()
        .map_err(|_| actix_web::error::ErrorNotFound("Invalid path"))?;

    let transfers_dir = data
        .config
        .transfers_dir
        .canonicalize()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Transfers dir missing"))?;

    if !canonical_path.starts_with(&transfers_dir) {
        return Err(actix_web::error::ErrorForbidden("Invalid file path"));
    }

    let filename = transfer.as_info(&data.config.base_url).filename;

    let file = NamedFile::open(&canonical_path)
        .map_err(|_| actix_web::error::ErrorNotFound("File not found"))?
        .set_content_type(transfer.mime_type.clone())
        .set_content_disposition(ContentDisposition::attachment(filename));

    transfer.update_download_count(&data.db).await.unwrap();

    Ok(file)
}

#[get("/download/{id}/info")]
pub async fn download_info(
    id: web::Path<String>,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let transfer = Transfer::find_by_id(&data.db, &id)
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("File not found"))?;

    Ok(HttpResponse::Ok().json(transfer.as_info(&data.config.base_url)))
}
