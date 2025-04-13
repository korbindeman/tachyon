use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::{HttpResponse, Responder, post, web};

use crate::{AppState, services::upload::Upload, utils::create_id};

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

    let path_string = format!("uploads/{}.zip", id);

    if let Err(e) = form.file.file.persist(&path_string) {
        println!("Error persisting file: {:?}", e);
        return HttpResponse::InternalServerError().json("Upload failed");
    }

    let upload = Upload::new(id.clone(), form.name.0, path_string);

    sqlx::query("INSERT INTO uploads (id, name, path) VALUES ($1, $2, $3)")
        .bind(&upload.id)
        .bind(&upload.name)
        .bind(&upload.path)
        .execute(&data.db)
        .await
        .unwrap();

    HttpResponse::Ok().json(format!(
        "File uploaded successfully: http://localhost:8080/download/{}",
        id
    ))
}
