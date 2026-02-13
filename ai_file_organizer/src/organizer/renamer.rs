use crate::models::FileInfo;
use std::path::PathBuf;
use anyhow::{Result, Context};
use std::fs;

pub struct Renamer {
    pub dry_run: bool,
    pub base_path: PathBuf,
}

impl Renamer {
    pub fn new(base_path: PathBuf, dry_run: bool) -> Self {
        Self { base_path, dry_run }
    }

    pub fn get_dest_path(&self, file: &FileInfo, category: &str) -> PathBuf {
        let mut dest = self.base_path.join(category);
        
        // Add date structure if available
        if let Some(modified) = file.metadata.modified {
            let year = modified.format("%Y").to_string();
            let month = modified.format("%m").to_string();
            dest = dest.join(year).join(month);
        }

        dest.join(&file.name)
    }

    pub fn apply(&self, file: &FileInfo, category: &str) -> Result<PathBuf> {
        let dest_path = self.get_dest_path(file, category);
        let dest_dir = dest_path.parent().unwrap();

        if !self.dry_run {
            fs::create_dir_all(dest_dir)?;
            
            // Handle collisions
            let mut final_path = dest_path.clone();
            let mut count = 1;
            while final_path.exists() {
                let stem = dest_path.file_stem().unwrap().to_string_lossy();
                let ext = dest_path.extension().unwrap_or_default().to_string_lossy();
                let new_name = format!("{}_{}.{}", stem, count, ext);
                final_path = dest_dir.join(new_name);
                count += 1;
            }

            fs::rename(&file.path, &final_path)
                .with_context(|| format!("Failed to move {:?} to {:?}", file.path, final_path))?;
            
            Ok(final_path)
        } else {
            Ok(dest_path)
        }
    }
}
