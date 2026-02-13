use crate::models::FileInfo;
use std::fs;
use anyhow::Result;
use chrono::DateTime;

pub fn extract_metadata(file: &mut FileInfo) -> Result<()> {
    let path = &file.path;
    let attr = fs::metadata(path)?;
    
    file.metadata.size = attr.len();
    file.metadata.modified = Some(DateTime::from(attr.modified()?));
    file.metadata.created = attr.created().ok().map(DateTime::from);

    // Extension
    if let Some(ext) = path.extension() {
        file.metadata.extension = ext.to_string_lossy().to_lowercase();
    }

    // MIME Type
    if let Ok(Some(kind)) = infer::get_from_path(path) {
        file.metadata.mime_type = kind.mime_type().to_string();
    }

    // Specialized extraction
    match file.metadata.extension.as_str() {
        "jpg" | "jpeg" | "png" => extract_image_metadata(file)?,
        "pdf" => extract_pdf_metadata(file)?,
        _ => {}
    }

    Ok(())
}

fn extract_image_metadata(file: &mut FileInfo) -> Result<()> {
    let path = &file.path;
    let file_stream = fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(file_stream);
    
    let exifreader = exif::Reader::new();
    if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
        if let Some(field) = exif.get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY) {
            file.metadata.extra.insert("date_original".to_string(), field.display_value().to_string());
        }
        if let Some(field) = exif.get_field(exif::Tag::Model, exif::In::PRIMARY) {
            file.metadata.extra.insert("camera_model".to_string(), field.display_value().to_string());
        }
    }
    Ok(())
}

fn extract_pdf_metadata(_file: &mut FileInfo) -> Result<()> {
    // Basic PDF metadata extraction (simplified for MVP)
    // In a real app, use pdf-extract crate
    Ok(())
}

pub fn calculate_hash(file: &mut FileInfo) -> Result<()> {
    let mut hasher = blake3::Hasher::new();
    let bytes = fs::read(&file.path)?;
    hasher.update(&bytes);
    file.metadata.hash = hasher.finalize().to_string();
    Ok(())
}
