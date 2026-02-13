use std::path::{Path, PathBuf};
use anyhow::Result;
use std::fs;

#[allow(dead_code)]
pub fn extract_archive(path: &Path) -> Result<PathBuf> {
    let dest = path.with_extension("");
    if !dest.exists() {
        fs::create_dir_all(&dest)?;
    }

    match path.extension().and_then(|s| s.to_str()) {
        Some("zip") => {
            let file = fs::File::open(path)?;
            let mut archive = zip::ZipArchive::new(file)?;
            archive.extract(&dest)?;
        }
        _ => {
            return Err(anyhow::anyhow!("Unsupported archive format"));
        }
    }

    Ok(dest)
}
