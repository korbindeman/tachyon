use crate::{
    AppState,
    services::{filetype::detect_file_type, upload::Upload},
    utils::{check_api_key, create_id, get_uploads_dir},
};
use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use std::path::PathBuf;

#[derive(MultipartForm)]
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

    let filetype = detect_file_type(&form.file);

    let file_path: PathBuf = get_uploads_dir().join(format!("{}.{}", id, &filetype.extension));

    if let Err(e) = form.file.file.persist(&file_path) {
        println!("Error persisting file: {:?}", e);
        return HttpResponse::InternalServerError().json("Upload failed");
    }

    let upload = Upload::new(
        id.clone(),
        form.name.0,
        file_path,
        form.file.size,
        filetype.mime_type,
    );

    if let Err(e) = upload.insert(&data.db).await {
        println!("DB insert error: {:?}", e);
        return HttpResponse::InternalServerError().json("DB error");
    }

    HttpResponse::Created()
        .append_header(("Location", format!("/download/{}", id)))
        .json(upload.as_info())
}
