use std::path::Path;

#[allow(dead_code)]
pub fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .map(|s| s.to_string_lossy().starts_with('.'))
        .unwrap_or(false)
}

#[allow(dead_code)]
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[allow(dead_code)]
pub fn is_screenshot(name: &str) -> bool {
    let re = regex::Regex::new(r"(?i)screenshot|scrnshot|capture").unwrap();
    re.is_match(name)
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Stats {
    pub total_files: usize,
    pub total_size: u64,
    pub category_counts: std::collections::HashMap<String, usize>,
}

#[allow(dead_code)]
pub fn calculate_stats(files: &[crate::models::FileInfo]) -> Stats {
    let mut stats = Stats::default();
    stats.total_files = files.len();
    for file in files {
        stats.total_size += file.metadata.size;
        if let Some(cat) = &file.category {
            *stats.category_counts.entry(cat.clone()).or_insert(0) += 1;
        }
    }
    stats
}
