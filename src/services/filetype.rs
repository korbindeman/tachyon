use actix_multipart::form::tempfile::TempFile;
use infer;
use mime::Mime;
use std::{fs::File, io::Read, path::PathBuf};
use tracing::warn;

#[derive(Debug)]
pub struct FileTypeInfo {
    pub mime_type: Mime,
    pub extension: String,
}

pub fn detect_file_type(file: &TempFile) -> FileTypeInfo {
    let file_name = file.file_name.clone().unwrap();
    let path_buf = PathBuf::from(file_name);
    let path = file.file.path();

    let browser_mime = file.content_type.clone().unwrap();
    let original_file_extension = path_buf
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("bin")
        .to_lowercase();

    let mut file = File::open(path).unwrap();

    let mut buffer = [0u8; 8192];
    let bytes_read = file.read(&mut buffer).unwrap();

    // try to infer by MIME type
    if let Some(kind) = infer::get(&buffer[..bytes_read]) {
        let file_type_info = FileTypeInfo {
            mime_type: kind.mime_type().parse().unwrap(),
            extension: kind.extension().to_string(),
        };

        compare_mime_type(&file_type_info.mime_type, &browser_mime);

        return file_type_info;
    }

    // check if it's plain text
    let is_text = std::str::from_utf8(&buffer[..bytes_read]).is_ok();
    if is_text {
        let file_type_info = FileTypeInfo {
            mime_type: plaintext_extension_to_mime(&original_file_extension),
            extension: original_file_extension,
        };

        compare_mime_type(&file_type_info.mime_type, &browser_mime);

        return file_type_info;
    }

    // TODO: should probably have a whitelist of MIME types that are allowed and just deny others

    // if it's still unknown log it as suspicious and add the original MIME type and extension
    warn!("Unable to infer MIME type. Given: {}", browser_mime);

    FileTypeInfo {
        mime_type: browser_mime,
        extension: original_file_extension,
    }
}

fn compare_mime_type(inferred: &Mime, given: &Mime) -> bool {
    if inferred == given {
        true
    } else {
        warn!(
            "Given MIME type doesn't match inferred MIME type. Inferred: {}, given: {}",
            inferred, given
        );

        false
    }
}

fn plaintext_extension_to_mime(ext: &str) -> Mime {
    match ext {
        "js" | "mjs" | "jsx" => "application/javascript".parse().unwrap(),
        "ts" | "tsx" => "application/typescript".parse().unwrap(),
        "html" => "text/html".parse().unwrap(),
        "css" => "text/css".parse().unwrap(),
        "json" => "application/json".parse().unwrap(),
        "md" => "text/markdown".parse().unwrap(),
        "csv" => "text/csv".parse().unwrap(),
        "xml" => "application/xml".parse().unwrap(),
        "svg" => "image/svg+xml".parse().unwrap(),
        _ => "text/plain".parse().unwrap(),
    }
}
