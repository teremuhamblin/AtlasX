use crate::model::{AtlasXReport, ExtensionStats};
use chrono::Utc;
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

pub fn scan_directory(root: &str) -> AtlasXReport {
    let mut by_extension: HashMap<String, ExtensionStats> = HashMap::new();
    let mut total_files = 0;
    let mut total_size = 0;

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }

        total_files += 1;
        let path = entry.path();
        let size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        total_size += size;

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("[no-ext]")
            .to_lowercase();

        let stats = by_extension.entry(ext).or_insert(ExtensionStats {
            count: 0,
            total_size: 0,
            sample_paths: Vec::new(),
        });

        stats.count += 1;
        stats.total_size += size;
        if stats.sample_paths.len() < 5 {
            stats.sample_paths.push(path.to_string_lossy().to_string());
        }
    }

    AtlasXReport {
        scanned_root: root.to_string(),
        generated_at: Utc::now().to_rfc3339(),
        total_files,
        total_size,
        by_extension,
    }
}
