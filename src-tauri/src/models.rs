use cleverlib::event::Event;
use serde::{Deserialize, Serialize};

/// Wrapper struct for Tauri IPC (cleverlib's Event doesn't implement Serialize)
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

/// Convert from cleverlib's Event to our serializable version
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
                    }
                    _ => None, // Not a JSON object or parsing failed
                }
            }
        };

        SerializableEvent {
            timestamp: event.time.clone().unwrap_or_else(|| "Unknown".to_string()),
            level: event
                .level
                .clone()
                .unwrap_or_else(|| "Information".to_string()),
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

/// Information about a log file
#[derive(Serialize, Deserialize, Debug)]
pub struct LogFileInfo {
    pub path: String,
    pub total_count: usize,
    pub log_levels: Vec<String>,
    pub date_range: Option<(String, String)>,
}

