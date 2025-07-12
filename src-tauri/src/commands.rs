use crate::models::LogFileInfo;
use cleverlib::clever_parser_options::CleverParserOptions;
use cleverlib::event::Event;
use cleverlib::event_collection::EventCollection;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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

/// Load all log entries from the file
#[tauri::command]
pub async fn load_all_log_entries(file_path: String) -> Result<Vec<Event>, String> {
    // Validate file exists and is accessible
    if !Path::new(&file_path).exists() {
        return Err(format!("File does not exist: {file_path}"));
    }

    println!("Reading entire file: {}", file_path);

    let file = File::open(&file_path).map_err(|e| format!("Failed to open file: {e}"))?;
    let reader = BufReader::new(file);

    // Read all lines from the file
    let all_lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap_or_default())
        .filter(|line| !line.trim().is_empty())
        .collect();

    if all_lines.is_empty() {
        return Ok(Vec::new());
    }

    println!("Read {} lines, starting parse", all_lines.len());

    // Parse all lines
    let options = CleverParserOptions {
        ignore_errors: Some(true),
        debug: Some(false),
    };

    let collection = EventCollection::create(&all_lines, Some(&options))
        .map_err(|e| format!("Failed to parse CLEF file: {e}"))?;

    println!("Parsed {} events", collection.events.len());
    Ok(collection.events)
}

/// Parse CLEF file and return all log data
#[tauri::command]
pub async fn parse_clef_file(file_path: String) -> Result<(LogFileInfo, Vec<Event>), String> {
    // First scan the file to get metadata
    let log_file_info = scan_clef_file(file_path.clone()).await?;

    // Then load all entries from the file
    let all_entries = load_all_log_entries(file_path).await?;
    println!("Returning {} events", all_entries.len());
    Ok((log_file_info, all_entries))
}
