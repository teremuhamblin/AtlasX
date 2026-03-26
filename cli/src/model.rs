use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct ExtensionStats {
    pub count: u64,
    pub total_size: u64,
    pub sample_paths: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct AtlasXReport {
    pub scanned_root: String,
    pub generated_at: String,
    pub total_files: u64,
    pub total_size: u64,
    pub by_extension: HashMap<String, ExtensionStats>,
}
