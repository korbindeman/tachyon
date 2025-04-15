use crate::{
    AppState,
    services::{filetype::detect_file_type, upload::Upload},
    utils::{check_api_key, create_id},
};
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::{HttpRequest, HttpResponse, Responder, post, web};

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

    let f = detect_file_type(&form.file);

    let id = create_id();

    let path_string = format!("uploads/{}.{}", id, &f.extension);

    let filesize = form.file.size;

    if let Err(e) = form.file.file.persist(&path_string) {
        println!("Error persisting file: {:?}", e);
        return HttpResponse::InternalServerError().json("Upload failed");
    }

    let upload = Upload::new(
        id.clone(),
        form.name.0,
        path_string,
        filesize,
        f.mime_type.to_string(),
    );

    sqlx::query(
        "INSERT INTO uploads (id, name, path, download_count, filesize, mime_type) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(&upload.id)
    .bind(&upload.name)
    .bind(&upload.path)
    .bind(upload.download_count)
    .bind(upload.filesize)
    .bind(&upload.mime_type)
    .execute(&data.db)
    .await
    .unwrap();

    HttpResponse::Created()
        .append_header(("Location", format!("/download/{}", id)))
        .json(upload.info())
}
