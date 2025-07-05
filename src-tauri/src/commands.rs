use crate::models::{LogFileInfo, SerializableEvent};
use cleverlib::event_collection::EventCollection;
use cleverlib::clever_parser_options::CleverParserOptions;
use std::fs;
use std::path::Path;

/// Parse CLEF file and return log data
#[tauri::command]
pub async fn parse_clef_file(file_path: String) -> Result<(LogFileInfo, Vec<SerializableEvent>), String> {
    // Validate file exists and is accessible
    if !Path::new(&file_path).exists() {
        return Err(format!("File does not exist: {}", file_path));
    }

    // Read file content
    let content = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to read file: {}", e)),
    };

    // Split into lines for CLEF parsing
    let lines: Vec<String> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.to_string())
        .collect();

    if lines.is_empty() {
        return Err("File is empty or contains no valid log entries".to_string());
    }

    // Parse with cleverlib
    let options = CleverParserOptions {
        ignore_errors: Some(true),
        debug: Some(false),
    };

    let collection = match EventCollection::create(&lines, Some(&options)) {
        Ok(collection) => collection,
        Err(e) => return Err(format!("Failed to parse CLEF file: {}", e)),
    };

    // Convert events to serializable format
    let serializable_events: Vec<SerializableEvent> = collection.events
        .iter()
        .zip(lines.iter()) // Pair events with their original raw strings
        .map(|(event, raw_event_str)| SerializableEvent::from((event, raw_event_str.as_str())))
        .collect();

    // Extract unique log levels
    let mut log_levels: Vec<String> = collection.log_levels.clone();
    log_levels.sort();

    // Find date range from events
    let mut timestamps: Vec<&str> = collection.events
        .iter()
        .filter_map(|event| event.time.as_deref())
        .collect();
    timestamps.sort();

    let date_range = if timestamps.len() >= 2 {
        Some((timestamps[0].to_string(), timestamps[timestamps.len() - 1].to_string()))
    } else if timestamps.len() == 1 {
        Some((timestamps[0].to_string(), timestamps[0].to_string()))
    } else {
        None
    };

    let log_file_info = LogFileInfo {
        path: file_path,
        total_count: serializable_events.len(),
        log_levels,
        date_range,
    };

    Ok((log_file_info, serializable_events))
}