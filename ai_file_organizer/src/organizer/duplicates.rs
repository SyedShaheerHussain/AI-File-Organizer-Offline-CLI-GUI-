use crate::models::FileInfo;
use std::collections::HashMap;
use anyhow::Result;
use crate::organizer::metadata::calculate_hash;

pub fn find_duplicates(files: &mut [FileInfo]) -> Result<HashMap<String, Vec<FileInfo>>> {
    let mut hash_map: HashMap<String, Vec<FileInfo>> = HashMap::new();

    for file in files {
        if file.metadata.hash.is_empty() {
            let _ = calculate_hash(file);
        }
        
        hash_map.entry(file.metadata.hash.clone())
            .or_insert_with(Vec::new)
            .push(file.clone());
    }

    // Keep only those with more than 1 occurrence
    hash_map.retain(|_, v| v.len() > 1);

    Ok(hash_map)
}

pub fn remove_duplicates(duplicates: &HashMap<String, Vec<FileInfo>>, dry_run: bool) -> Result<()> {
    for (hash, files) in duplicates {
        println!("Duplicate set for hash {}:", hash);
        // Skip the first one (keep it)
        for duplicate in files.iter().skip(1) {
            if dry_run {
                println!("  [DRY-RUN] Would remove duplicate: {:?}", duplicate.path);
            } else {
                println!("  Removing duplicate: {:?}", duplicate.path);
                std::fs::remove_file(&duplicate.path)?;
            }
        }
    }
    Ok(())
}
