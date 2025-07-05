use serde::{Deserialize, Serialize};
use cleverlib::event_collection::EventCollection;
use cleverlib::clever_parser_options::CleverParserOptions;
use cleverlib::event::Event;
use std::fs;
use std::path::Path;

// Wrapper struct for Tauri IPC (cleverlib's Event doesn't implement Serialize)
#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableEvent {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub template: Option<String>,
    pub exception: Option<String>,
    #[serde(rename = "eventId")]
    pub event_id: Option<String>,
    pub properties: Option<serde_json::Value>,
}

// Convert from cleverlib's Event to our serializable version
impl From<(&Event, &str)> for SerializableEvent {
    fn from((event, raw_event_str): (&Event, &str)) -> Self {
        let message = if let Some(msg) = &event.message {
            msg.clone()
        } else {
            event.template.clone()
        };

               let properties: Option<serde_json::Value> = {
            if raw_event_str.is_empty() {
                None
            } else {
                match serde_json::from_str::<serde_json::Value>(raw_event_str) {
                    Ok(serde_json::Value::Object(mut map)) => {
                        // Remove standard CLEF properties to keep only custom ones
                        map.remove("@t");
                        map.remove("@mt");
                        map.remove("@m");
                        map.remove("@l");
                        map.remove("@x");
                        map.remove("@i");
                        map.remove("@r");

                        if map.is_empty() {
                            None
                        } else {
                            Some(serde_json::Value::Object(map))
                        }
                    },
                    _ => None, // Not a JSON object or parsing failed
                }
            }
        };

        SerializableEvent {
            timestamp: event.time.clone().unwrap_or_else(|| "Unknown".to_string()),
            level: event.level.clone().unwrap_or_else(|| "Information".to_string()),
            template: if event.template != message && !event.template.is_empty() {
                Some(event.template.clone())
            } else {
                None
            },
            message,
            exception: event.exception.clone(),
            event_id: event.eventid.clone(),
            properties,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogFileInfo {
    pub path: String,
    pub total_count: usize,
    pub log_levels: Vec<String>,
    pub date_range: Option<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterOptions {
    pub level_filter: Option<String>,
    pub text_filter: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}





// Parse CLEF file and return log data
#[tauri::command]
async fn parse_clef_file(file_path: String) -> Result<(LogFileInfo, Vec<SerializableEvent>), String> {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            parse_clef_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
