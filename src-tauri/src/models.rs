use serde::{Deserialize, Serialize};

/// Information about a log file
#[derive(Serialize, Deserialize, Debug)]
pub struct LogFileInfo {
    pub path: String,
    pub total_count: usize,
    pub log_levels: Vec<String>,
    pub date_range: Option<(String, String)>,
}

