use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::{HttpRequest, HttpResponse, Responder, post, web};

use crate::{
    AppState,
    services::upload::Upload,
    utils::{check_api_key, create_id},
};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    name: Text<String>,
    file: TempFile,
}

#[post("/upload")]
pub async fn upload(
    req: HttpRequest,
    MultipartForm(form): MultipartForm<UploadForm>,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(res) = check_api_key(&req) {
        return res;
    }

    let id = create_id();

    let path_string = format!("uploads/{}.zip", id);

    let filesize = form.file.size;

    dbg!(filesize);

    if let Err(e) = form.file.file.persist(&path_string) {
        println!("Error persisting file: {:?}", e);
        return HttpResponse::InternalServerError().json("Upload failed");
    }

    let upload = Upload::new(id.clone(), form.name.0, path_string, filesize);

    sqlx::query(
        "INSERT INTO uploads (id, name, path, download_count, filesize) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(&upload.id)
    .bind(&upload.name)
    .bind(&upload.path)
    .bind(upload.download_count)
    .bind(upload.filesize)
    .execute(&data.db)
    .await
    .unwrap();

    HttpResponse::Ok().json(format!(
        "File uploaded successfully: http://localhost:8080/download/{}",
        id
    ))
}
