use std::ffi::OsStr;

pub fn extension_to_filetype(extension: &OsStr) -> String {
    let ext = extension.to_str().unwrap_or("").to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" => "JPEG Image".to_string(),
        "png" => "PNG Image".to_string(),
        "gif" => "GIF Image".to_string(),
        "pdf" => "PDF Document".to_string(),
        "txt" => "Text File".to_string(),
        "doc" | "docx" => "Word Document".to_string(),
        "xls" | "xlsx" => "Excel Spreadsheet".to_string(),
        "ppt" | "pptx" => "PowerPoint Presentation".to_string(),
        "zip" => "Zip Archive".to_string(),
        "rar" => "RAR Archive".to_string(),
        "mp3" => "MP3 Audio".to_string(),
        "mp4" => "MP4 Video".to_string(),
        "html" | "htm" => "HTML Document".to_string(),
        "css" => "CSS Stylesheet".to_string(),
        "js" => "JavaScript File".to_string(),
        "json" => "JSON File".to_string(),
        "xml" => "XML Document".to_string(),
        "csv" => "CSV File".to_string(),
        _ => format!("{} File", ext.to_uppercase()),
    }
}
