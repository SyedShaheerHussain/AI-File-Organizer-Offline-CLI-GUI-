use crate::models::FileInfo;
use std::path::Path;
use walkdir::WalkDir;
use rayon::prelude::*;
use anyhow::Result;

pub fn scan(path: &Path) -> Result<Vec<FileInfo>> {
    let entries: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    let files: Vec<FileInfo> = entries
        .into_par_iter()
        .map(|entry| {
            FileInfo::new(entry.path().to_path_buf())
        })
        .collect();

    Ok(files)
}

#[allow(dead_code)]
pub fn scan_with_ignore(path: &Path, ignore_list: &[String]) -> Result<Vec<FileInfo>> {
    let entries: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            !ignore_list.contains(&name) && !name.starts_with('.')
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    let files: Vec<FileInfo> = entries
        .into_par_iter()
        .map(|entry| {
            FileInfo::new(entry.path().to_path_buf())
        })
        .collect();

    Ok(files)
}
