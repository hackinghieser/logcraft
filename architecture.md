# Tauri CLEF Log Viewer - Implementation Guide

## Project Overview

A desktop application built with Tauri for viewing and analyzing CLEF (Compact Log Event Format) log files. This document serves as our implementation roadmap and collaboration guide.

### Technology Stack
- **Frontend**: Vue 3 + TypeScript + PrimeVue
- **Backend**: Rust + Tauri + cleverlib crate
- **Build System**: Vite
- **Package Manager**: Bun
- **Log Format**: CLEF (Compact Log Event Format)

## CLEF Format Reference

CLEF is a JSON-based log format where each line contains a single JSON object:

```json
{"@t":"2016-06-07T03:44:57.8532799Z","@mt":"Hello, {User}","User":"nblumhardt","@l":"Information"}
```

### Key CLEF Properties
- `@t` - Timestamp (ISO 8601)
- `@mt` - Message template
- `@m` - Rendered message (alternative to @mt)
- `@l` - Log level
- `@x` - Exception details
- `@i` - Event ID
- `@r` - Array of rendered values for message template tokens

## Implementation Plan

### Phase 1: Project Setup ✅ / ❌
- [ ] Initialize Tauri project with Vue 3
- [ ] Add PrimeVue and required dependencies
- [ ] Configure TypeScript
- [ ] Add cleverlib dependency to Rust backend
- [ ] Create basic project structure
- [ ] Test basic Tauri IPC communication

### Phase 2: Core Backend (Rust)
- [ ] Add cleverlib dependency and examine Event struct
- [ ] Determine if Event struct needs Serde traits for serialization
- [ ] Create wrapper types for Tauri IPC if needed
- [ ] Implement file reading with EventCollection
- [ ] Create Tauri commands using cleverlib models
- [ ] Add error handling and logging
- [ ] Test with sample CLEF files

### Phase 3: Basic Frontend (Vue 3 + PrimeVue)
- [ ] Create main application layout
- [ ] Implement file selection component
- [ ] Create log display table
- [ ] Add basic styling and theme
- [ ] Connect to backend via Tauri APIs
- [ ] Handle loading states

### Phase 4: Filtering & Search
- [ ] Add log level filtering
- [ ] Implement text search functionality
- [ ] Create filter UI components
- [ ] Add date/time range filtering
- [ ] Implement clear filters functionality

### Phase 5: Polish & Testing
- [ ] Error handling and user feedback
- [ ] Performance optimization
- [ ] UI/UX improvements
- [ ] Testing with various CLEF files
- [ ] Documentation

## Current Implementation Status

### ✅ Completed
- Initial project planning
- Architecture design

### 🚧 In Progress
- Project setup and configuration

### ❌ Not Started
- All implementation phases

## Data Models

### Using cleverlib's Built-in Models

Instead of creating custom models, we'll leverage cleverlib's existing structures:

```rust
// From cleverlib - we'll use these directly
use cleverlib::event_collection::EventCollection;
use cleverlib::Event; // The individual log event structure

// Wrapper for Tauri IPC (if needed for additional metadata)
#[derive(Debug, Serialize, Deserialize)]
pub struct LogFileInfo {
    pub path: String,
    pub total_count: usize,
    pub log_levels: Vec<String>,        // from EventCollection.log_levels
    pub date_range: Option<(String, String)>, // derived from events
}

// For filtering operations - simple wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct FilterOptions {
    pub level_filter: Option<String>,
    pub text_filter: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}
```

### Frontend TypeScript Types

Based on CLEF format specification and cleverlib's usage, we expect the Event structure to include:

```typescript
// This interface should match cleverlib's Event struct
// Based on CLEF format: https://clef-json.org/
interface Event {
  // Standard CLEF properties (all optional as per CLEF spec)
  timestamp?: string;        // @t - ISO 8601 timestamp
  messageTemplate?: string;  // @mt - message template
  message?: string;          // @m - rendered message (alternative to @mt)
  level?: string;           // @l - log level (Error, Warning, Info, etc.)
  exception?: string;       // @x - exception details
  eventId?: string;         // @i - event ID
  renderings?: string[];    // @r - rendered values for template tokens
  
  // Custom properties - any additional fields from the log entry
  properties?: Record<string, any>;
  
  // Potentially added by cleverlib for processing
  rawLine?: string;         // original JSON line
  lineNumber?: number;      // line number in file
  
  // Note: We'll verify these fields when we examine cleverlib's actual Event struct
}

interface EventCollection {
  events: Event[];
  log_levels: string[];     // automatically detected levels
}

interface LogFileInfo {
  path: string;
  totalCount: number;
  logLevels: string[];
  dateRange?: [string, string];
}

interface FilterOptions {
  levelFilter?: string;
  textFilter?: string;
  dateFrom?: string;
  dateTo?: string;
}
```

**Important**: These interfaces are based on the CLEF specification and cleverlib's documented functionality. We'll need to verify and adjust them once we examine the actual cleverlib Event struct during implementation.

## Tauri Commands (Backend API)

### File Operations
```rust
#[tauri::command]
async fn open_log_file(path: String) -> Result<LogFileInfo, String>

#[tauri::command]
async fn get_events_page(
    path: String, 
    page: usize, 
    page_size: usize
) -> Result<Vec<Event>, String>
```

### Filtering Operations
```rust
#[tauri::command]
async fn filter_events(
    path: String,
    filter_options: FilterOptions
) -> Result<Vec<Event>, String>

#[tauri::command]
async fn search_events(
    path: String,
    search_term: String
) -> Result<Vec<Event>, String>
```

Note: We'll need to ensure cleverlib's `Event` struct implements `Serialize` and `Deserialize` for Tauri IPC. If not, we may need to create a wrapper or contribute to cleverlib.

## UI Component Structure

### Main Application Layout
```
App.vue
├── Toolbar (file operations, theme toggle)
├── MainContent
│   ├── FileSelector (when no file loaded)
│   └── LogViewer (when file loaded)
│       ├── FilterPanel
│       │   ├── LevelFilter
│       │   ├── TextSearch
│       │   ├── DateRangeFilter
│       │   └── ClearFilters
│       ├── LogTable (PrimeVue DataTable)
│       └── LogDetails (expandable details panel)
└── StatusBar (file info, entry count, etc.)
```

### Key PrimeVue Components
- `DataTable` - Main log display
- `FileUpload` - File selection (customized for native picker)
- `MultiSelect` - Log level filtering
- `Calendar` - Date range selection
- `InputText` - Search functionality
- `Toolbar` - Top navigation
- `Splitter` - Resizable panels
- `Dialog` - Log entry details
- `ProgressBar` - File loading progress

## Implementation Notes

### Performance Considerations
- **Serial Processing**: Use `EventCollection::create()` for simplicity
- **Virtual Scrolling**: Implement with PrimeVue's DataTable virtualScrolling
- **Pagination**: Server-side pagination for large files
- **Debounced Search**: 300ms delay for text search inputs
- **Lazy Loading**: Load entry details on demand

### Error Handling Strategy
- **File Errors**: Invalid format, permissions, file not found
- **Parsing Errors**: Malformed JSON, missing required fields
- **UI Errors**: Display user-friendly messages with retry options
- **Performance**: Graceful degradation for very large files

### Development Workflow
1. **Backend First**: Implement and test Rust functions independently
2. **API Integration**: Create Tauri commands and test via browser console
3. **UI Implementation**: Build Vue components with mock data first
4. **Integration**: Connect frontend to backend
5. **Testing**: Test with real CLEF files of various sizes

## Sample CLEF Files for Testing

### Basic Log Entry
```json
{"@t":"2024-01-15T10:30:45.123Z","@mt":"User {UserId} logged in","UserId":"john_doe","@l":"Information"}
```

### Error with Exception
```json
{"@t":"2024-01-15T10:31:02.456Z","@mt":"Database connection failed","@l":"Error","@x":"System.Data.SqlClient.SqlException: A network-related or instance-specific error..."}
```

### Complex Entry with Properties
```json
{"@t":"2024-01-15T10:31:15.789Z","@mt":"Processing order {OrderId} for customer {CustomerId}","OrderId":"ORD-12345","CustomerId":"CUST-789","Amount":99.99,"Currency":"USD","@l":"Information","RequestId":"req-abc-123"}
```

## Development Commands

### Setup
```bash
# Initialize Tauri project
bun create tauri-app@latest clef-viewer
cd clef-viewer

# Add dependencies
bun add primevue primeicons
bun add -D @types/node

# Add Rust dependencies in src-tauri/Cargo.toml
# cleverlib = "0.1"  # Check actual version
# serde = { version = "1.0", features = ["derive"] }
# serde_json = "1.0"
```

### Development
```bash
# Start development server
bun run tauri dev

# Build for production
bun run tauri build

# Test Rust backend only
cd src-tauri && cargo test

# Install dependencies (if needed)
bun install
```

## Questions & Decisions

### Current Questions
- [ ] Which version of cleverlib should we use?
- [ ] Should we implement custom error types or use String for simplicity?
- [ ] Virtual scrolling vs pagination for large files?
- [ ] Theme preference: should we match system theme?
- [ ] Do we need to add Serde traits to cleverlib's Event struct for JSON serialization?

### Decisions Made
- ✅ Serial processing for v1 (parallel processing deferred)
- ✅ PrimeVue for UI components
- ✅ TypeScript for frontend
- ✅ Simple string-based error handling initially
- ✅ Reuse cleverlib's Event and EventCollection models instead of creating custom ones

## Next Steps

1. **Project Initialization**: Set up basic Tauri + Vue 3 + PrimeVue project
2. **Backend Foundation**: Create basic file reading and cleverlib integration
3. **Frontend Skeleton**: Basic layout with PrimeVue components
4. **Integration Testing**: Ensure Tauri IPC works correctly

## Resources

- [CLEF Specification](http://clef-json.org/)
- [cleverlib Documentation](https://github.com/hackinghieser/cleverlib)
- [Tauri Documentation](https://tauri.app/)
- [PrimeVue Documentation](https://primevue.org/)
- [Vue 3 Documentation](https://vuejs.org/)

---

**Last Updated**: [Current Date]  
**Current Phase**: Project Setup  
**Next Milestone**: Basic file reading functionality
