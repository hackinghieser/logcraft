use serde::{Deserialize, Serialize};
use cleverlib::event_collection::EventCollection;
use cleverlib::clever_parser_options::CleverParserOptions;
use cleverlib::event::Event;

// Wrapper struct for Tauri IPC (cleverlib's Event doesn't implement Serialize)
#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableEvent {
    pub time: Option<String>,
    pub message: Option<String>,
    pub template: String,
    pub level: Option<String>,
    pub exception: Option<String>,
    pub eventid: Option<String>,
    pub renderings: Vec<String>,
}

// Convert from cleverlib's Event to our serializable version
impl From<&Event> for SerializableEvent {
    fn from(event: &Event) -> Self {
        SerializableEvent {
            time: event.time.clone(),
            message: event.message.clone(),
            template: event.template.clone(),
            level: event.level.clone(),
            exception: event.exception.clone(),
            eventid: event.eventid.clone(),
            renderings: event.renderings.clone(),
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

// Test cleverlib with serializable wrapper for Tauri IPC
#[tauri::command]
fn test_cleverlib_parsing() -> Result<Vec<SerializableEvent>, String> {
    let test_entries = vec![
        r#"{"@t":"2024-01-15T10:30:45.123Z","@mt":"User {UserId} logged in","UserId":"john_doe","@l":"Information"}"#.to_string(),
        r#"{"@t":"2024-01-15T10:31:02.456Z","@mt":"Database connection failed","@l":"Error","@x":"System.Exception: Connection timeout"}"#.to_string(),
    ];
    
    let options = CleverParserOptions {
        ignore_errors: Some(true),
        debug: Some(false),
    };
    
    match EventCollection::create(&test_entries, Some(&options)) {
        Ok(collection) => {
            let serializable_events: Vec<SerializableEvent> = collection.events
                .iter()
                .map(|event| SerializableEvent::from(event))
                .collect();
            Ok(serializable_events)
        }
        Err(e) => Err(format!("Failed to parse CLEF entries: {}", e))
    }
}

// Test cleverlib integration and demonstrate Event struct usage
#[tauri::command]
fn examine_cleverlib_event() -> String {
    use cleverlib::event_collection::EventCollection;
    use cleverlib::clever_parser_options::CleverParserOptions;
    
    // Create test CLEF log entries showcasing different CLEF properties
    let test_json_entries = vec![
        r#"{"@t":"2024-01-15T10:30:45.123Z","@mt":"User {UserId} logged in","UserId":"john_doe","@l":"Information"}"#.to_string(),
        r#"{"@t":"2024-01-15T10:31:02.456Z","@mt":"Database connection failed","@l":"Error","@x":"System.Data.SqlClient.SqlException: A network-related error occurred"}"#.to_string(),
        r#"{"@t":"2024-01-15T10:31:15.789Z","@mt":"Processing order {OrderId} for customer {CustomerId}","OrderId":"ORD-12345","CustomerId":"CUST-789","Amount":99.99,"Currency":"USD","@l":"Information","@i":"event123"}"#.to_string(),
        r#"{"@mt":"No timestamp event with template {Name}","Name":"TestEvent","@l":"Warning"}"#.to_string(),
        r#"{"@t":"2024-01-15T10:32:00.000Z","@m":"Direct message without template","@l":"Debug"}"#.to_string(),
    ];
    
    // Set up parser options
    let options = CleverParserOptions {
        ignore_errors: Some(true),
        debug: Some(true),
    };
    
    // Create EventCollection
    match EventCollection::create(&test_json_entries, Some(&options)) {
        Ok(collection) => {
            let mut result = String::new();
            result.push_str(&format!("=== CLEVERLIB EVENT EXAMINATION ===\n"));
            result.push_str(&format!("EventCollection created successfully!\n"));
            result.push_str(&format!("Total events: {}\n", collection.events.len()));
            result.push_str(&format!("Log levels detected: {:?}\n\n", collection.log_levels));
            
            // Examine each event to show different field combinations
            for (i, event) in collection.events.iter().enumerate() {
                result.push_str(&format!("Event {}:\n", i + 1));
                result.push_str(&format!("  Timestamp (@t): {:?}\n", event.time));
                result.push_str(&format!("  Message (@m): {:?}\n", event.message));
                result.push_str(&format!("  Template (@mt): {:?}\n", event.template));
                result.push_str(&format!("  Level (@l): {:?}\n", event.level));
                result.push_str(&format!("  Exception (@x): {:?}\n", event.exception));
                result.push_str(&format!("  Event ID (@i): {:?}\n", event.eventid));
                result.push_str(&format!("  Renderings (@r): {:?}\n", event.renderings));
                result.push_str("\n");
            }
            
            // Test filtering functionality
            let error_events = collection.filter_log_level("Error");
            result.push_str(&format!("Events with 'Error' level: {}\n", error_events.len()));
            
            let info_events = collection.filter_log_level("Information");
            result.push_str(&format!("Events with 'Information' level: {}\n", info_events.len()));
            
            result
        }
        Err(e) => {
            format!("Error creating EventCollection: {}", e)
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, examine_cleverlib_event, test_cleverlib_parsing])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
