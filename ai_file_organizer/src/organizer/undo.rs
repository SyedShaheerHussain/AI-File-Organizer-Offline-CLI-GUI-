use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use anyhow::Result;
use std::fs;
use crate::constants::UNDO_HISTORY_FILE;

#[derive(Debug, Serialize, Deserialize)]
pub struct UndoEntry {
    pub original_path: PathBuf,
    pub current_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UndoHistory {
    pub entries: Vec<UndoEntry>,
}

#[allow(dead_code)]
pub fn save_history(history: &UndoHistory, root: &Path) -> Result<()> {
    let path = root.join(UNDO_HISTORY_FILE);
    let content = serde_json::to_string_pretty(history)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn load_history(root: &Path) -> Result<UndoHistory> {
    let path = root.join(UNDO_HISTORY_FILE);
    if !path.exists() {
        return Ok(UndoHistory::default());
    }
    let content = fs::read_to_string(path)?;
    let history = serde_json::from_str(&content)?;
    Ok(history)
}

pub fn undo(root: &Path) -> Result<()> {
    let history = load_history(root)?;
    for entry in history.entries.iter().rev() {
        if entry.current_path.exists() {
            if let Some(parent) = entry.original_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::rename(&entry.current_path, &entry.original_path)?;
            println!("Restored: {:?} -> {:?}", entry.current_path, entry.original_path);
        }
    }
    // Clear history after undo
    let path = root.join(UNDO_HISTORY_FILE);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
