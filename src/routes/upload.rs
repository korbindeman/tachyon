use crate::{
    AppState,
    services::upload::Upload,
    utils::{create_id, get_uploads_dir},
};
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::{HttpResponse, Responder, post, web};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    name: Text<String>,
    file: TempFile,
}

#[post("/upload")]
pub async fn upload(
    MultipartForm(form): MultipartForm<UploadForm>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = create_id();

    let uploads_dir = get_uploads_dir();
    let file_path = uploads_dir.join(format!("{id}.zip"));

    let file_path_str = file_path.to_string_lossy().to_string();

    let filesize = form.file.size;

    if let Err(e) = form.file.file.persist(&file_path) {
        println!("Error persisting file: {:?}", e);
        return HttpResponse::InternalServerError().json("Upload failed");
    }

    let upload = Upload::new(id.clone(), form.name.0, file_path_str, filesize);

    if let Err(e) = sqlx::query(
        "INSERT INTO uploads (id, name, path, download_count, filesize) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(&upload.id)
    .bind(&upload.name)
    .bind(&upload.path)
    .bind(&upload.download_count)
    .bind(&upload.filesize)
    .execute(&data.db)
    .await
    {
        println!("DB insert error: {:?}", e);
        return HttpResponse::InternalServerError().json("DB error");
    }

    HttpResponse::Ok().json(format!(
        "File uploaded successfully: http://localhost:8080/download/{}",
        id
    ))
}
