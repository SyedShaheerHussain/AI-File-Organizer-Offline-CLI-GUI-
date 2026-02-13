use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub mime_type: String,
    pub extension: String,
    pub created: Option<DateTime<Local>>,
    pub modified: Option<DateTime<Local>>,
    pub size: u64,
    pub hash: String,
    pub extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub metadata: FileMetadata,
    pub category: Option<String>,
    pub suggested_name: Option<String>,
}

impl FileInfo {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        Self {
            path,
            name,
            metadata: FileMetadata {
                mime_type: "unknown".to_string(),
                extension: "".to_string(),
                created: None,
                modified: None,
                size: 0,
                hash: "".to_string(),
                extra: std::collections::HashMap::new(),
            },
            category: None,
            suggested_name: None,
        }
    }
}
