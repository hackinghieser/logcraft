use crate::models::{LogFileInfo, SerializableEvent};
use cleverlib::clever_parser_options::CleverParserOptions;
use cleverlib::event_collection::EventCollection;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const PAGE_SIZE: usize = 500; // Load 500 entries per page

/// Scan CLEF file to get metadata without loading all entries
#[tauri::command]
pub async fn scan_clef_file(file_path: String) -> Result<LogFileInfo, String> {
    // Validate file exists and is accessible
    if !Path::new(&file_path).exists() {
        return Err(format!("File does not exist: {file_path}"));
    }

    let first_timestamp: Option<String> = None;
    let last_timestamp: Option<String> = None;
    // Count remaining lines in file efficiently
    let file = File::open(&file_path).map_err(|e| format!("Failed to reopen file: {e}"))?;

    let reader = BufReader::new(file);
    let total_count = reader
        .lines()
        .filter(|line| !line.as_ref().unwrap_or(&String::new()).trim().is_empty())
        .count();

    let log_levels_vec: Vec<String> = vec![];

    let date_range = match (first_timestamp, last_timestamp) {
        (Some(first), Some(last)) => Some((first, last)),
        _ => None,
    };

    Ok(LogFileInfo {
        path: file_path,
        total_count,
        log_levels: log_levels_vec,
        date_range,
    })
}

/// Load a specific page of log entries
#[tauri::command]
pub async fn load_log_entries_page(
    file_path: String,
    page: usize,
) -> Result<Vec<SerializableEvent>, String> {
    // Validate file exists and is accessible
    if !Path::new(&file_path).exists() {
        return Err(format!("File does not exist: {file_path}"));
    }

    let file = File::open(&file_path).map_err(|e| format!("Failed to open file: {e}"))?;

    let reader = BufReader::new(file);

    let skip_count = page * PAGE_SIZE;
    // Read only the lines we need for this page
    let page_lines: Vec<String> = reader
        .lines()
        .skip(skip_count)
        .take(PAGE_SIZE)
        .map(|line| line.unwrap_or_default())
        .filter(|line| !line.trim().is_empty())
        .collect();

    if page_lines.is_empty() {
        return Ok(Vec::new());
    }

    // Parse only this page of lines
    let options = CleverParserOptions {
        ignore_errors: Some(true),
        debug: Some(false),
    };

    let collection = EventCollection::create(&page_lines, Some(&options))
        .map_err(|e| format!("Failed to parse CLEF file: {e}"))?;

    // Convert events to serializable format
    let serializable_events: Vec<SerializableEvent> = collection
        .events
        .iter()
        .zip(page_lines.iter())
        .map(|(event, raw_event_str)| SerializableEvent::from((event, raw_event_str.as_str())))
        .collect();

    Ok(serializable_events)
}

/// Parse CLEF file and return initial page of log data (backwards compatibility)
#[tauri::command]
pub async fn parse_clef_file(
    file_path: String,
) -> Result<(LogFileInfo, Vec<SerializableEvent>), String> {
    // First scan the file to get metadata
    let log_file_info = scan_clef_file(file_path.clone()).await?;

    // Then load the first page of entries
    let first_page_entries = load_log_entries_page(file_path, 0).await?;

    Ok((log_file_info, first_page_entries))
}

