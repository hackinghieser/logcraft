<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import Splitter from "primevue/splitter";
import SplitterPanel from "primevue/splitterpanel";

// Components
import AppToolbar from "./components/AppToolbar.vue";
import FiltersPanel from "./components/FiltersPanel.vue";
import LogTable from "./components/LogTable.vue";
import DetailsPanel from "./components/DetailsPanel.vue";
import StatusBar from "./components/StatusBar.vue";

// Types
interface LogEntry {
  timestamp: string;
  level: string;
  message: string;
  template?: string;
  exception?: string;
  eventId?: string;
  properties?: Record<string, any>;
}

interface LogFileInfo {
  path: string;
  totalCount: number;
  logLevels: string[];
  dateRange?: [string, string];
}

// Reactive data
const logFile = ref<LogFileInfo | null>(null);
const logEntries = ref<LogEntry[]>([]);
const filteredEntries = ref<LogEntry[]>([]);
const selectedEntry = ref<LogEntry | null>(null);
const loading = ref(false);

// Sample data for development




// Drag and drop state
const isDragOver = ref(false);

// Event handlers
async function handleOpenFile() {
  try {
    // Import Tauri APIs
    const { open } = await import('@tauri-apps/plugin-dialog');
    
    // Open file dialog
    const selected = await open({
      filters: [
        {
          name: 'CLEF Log Files',
          extensions: ['clef', 'log', 'txt']
        },
        {
          name: 'All Files',
          extensions: ['*']
        }
      ]
    });

    if (selected) {
      await handleFileOpen(selected);
    }
  } catch (error) {
    console.error('Failed to open file dialog:', error);
    alert(`Failed to open file dialog: ${error}`);
  }
}

// Tauri file drop handlers using 2.0 API
async function setupFileDropListener() {
  try {
    const { getCurrentWebview } = await import('@tauri-apps/api/webview');
    
    // Set up drag and drop event listener
    const unlisten = await getCurrentWebview().onDragDropEvent((event: any) => {
      switch (event.payload.type) {
        case 'over':
          isDragOver.value = true;
          break;
        case 'drop':
          isDragOver.value = false;
          if (event.payload.paths && event.payload.paths.length > 0) {
            handleFileDropped(event.payload.paths[0]);
          }
          break;
        case 'cancel':
          isDragOver.value = false;
          break;
      }
    });
    
    // Return cleanup function
    return unlisten;
  } catch (error) {
    console.error('Failed to setup file drop listener:', error);
    return () => {};
  }
}

async function handleFileDropped(filePath: string) {
  isDragOver.value = false;
  
  // Check if file is a log file
  const validExtensions = ['clef', 'log', 'txt'];
  const fileExtension = filePath.split('.').pop()?.toLowerCase();
  
  if (fileExtension && validExtensions.includes(fileExtension)) {
    await handleFileOpen(filePath);
  } else {
    alert('Please drop a valid log file (.clef, .log, or .txt)');
  }
}

// Extract file opening logic to reuse for both dialog and drag & drop
async function handleFileOpen(filePath: string) {
  loading.value = true;
  try {
    // Import Tauri APIs
    const { invoke } = await import('@tauri-apps/api/core');
    
    // Parse the selected file
    const result = await invoke('parse_clef_file', { filePath }) as [any, LogEntry[]];
    const [fileInfo, events] = result;
    
    // Update the application state
    logFile.value = {
      path: fileInfo.path,
      totalCount: fileInfo.total_count,
      logLevels: fileInfo.log_levels,
      dateRange: fileInfo.date_range
    };
    
    logEntries.value = events;
    filteredEntries.value = [...events];
    selectedEntry.value = events.length > 0 ? events[0] : null;
    
    console.log(`Loaded ${events.length} log entries from ${fileInfo.path}`);
  } catch (error) {
    console.error('Failed to parse log file:', error);
    alert(`Failed to parse log file: ${error}`);
  } finally {
    loading.value = false;
  }
}

function handleToggleTheme() {
  // TODO: Implement theme toggle
  console.log("Toggle theme...");
}

function handleOpenSettings() {
  // TODO: Implement settings dialog
  console.log("Open settings...");
}

function handleUpdateFilters(filters: { selectedLevels: string[]; searchText: string; dateRange: Date[] }) {
  // Apply filters to logEntries
  let filtered = [...logEntries.value];
  
  // Filter by log levels
  if (filters.selectedLevels.length > 0) {
    filtered = filtered.filter(entry => filters.selectedLevels.includes(entry.level));
  }
  
  // Filter by search text
  if (filters.searchText.trim()) {
    const searchTerm = filters.searchText.toLowerCase();
    filtered = filtered.filter(entry => 
      entry.message.toLowerCase().includes(searchTerm) ||
      entry.template?.toLowerCase().includes(searchTerm) ||
      entry.level.toLowerCase().includes(searchTerm)
    );
  }
  
  // Filter by date range
  if (filters.dateRange.length === 2) {
    const [startDate, endDate] = filters.dateRange;
    filtered = filtered.filter(entry => {
      const entryDate = new Date(entry.timestamp);
      return entryDate >= startDate && entryDate <= endDate;
    });
  }
  
  filteredEntries.value = filtered;
  
  // Reset selection if current selection is not in filtered results
  if (selectedEntry.value && !filtered.includes(selectedEntry.value)) {
    selectedEntry.value = filtered.length > 0 ? filtered[0] : null;
  }
}


function handleEntrySelect(entry: LogEntry) {
  selectedEntry.value = entry;
}

// Handle splitter drag to prevent text selection
let isDraggingSplitter = false;

function handleSplitterStart() {
  isDraggingSplitter = true;
  document.body.classList.add('splitter-dragging');
}

function handleSplitterEnd() {
  isDraggingSplitter = false;
  document.body.classList.remove('splitter-dragging');
}

onMounted(async () => {
  // Setup file drop listener
  const cleanupFileDropListener = await setupFileDropListener();
  
  // Add global event listeners for splitter events
  document.addEventListener('mousedown', (e) => {
    if ((e.target as HTMLElement)?.closest('.p-splitter-gutter')) {
      handleSplitterStart();
    }
  });
  
  document.addEventListener('mouseup', () => {
    if (isDraggingSplitter) {
      handleSplitterEnd();
    }
  });
  
  document.addEventListener('mouseleave', () => {
    if (isDraggingSplitter) {
      handleSplitterEnd();
    }
  });
  
  // Additional safety: clear on any click outside splitter
  document.addEventListener('click', (e) => {
    if (!(e.target as HTMLElement)?.closest('.p-splitter-gutter')) {
      handleSplitterEnd();
    }
  });
  
  // Store cleanup function for unmount
  (window as any)._cleanupFileDropListener = cleanupFileDropListener;
});

onUnmounted(() => {
  document.body.classList.remove('splitter-dragging');
  
  // Cleanup file drop listener
  if ((window as any)._cleanupFileDropListener) {
    (window as any)._cleanupFileDropListener();
  }
});
</script>

<template>
  <div 
    class="app-container"
    :class="{ 'drag-over': isDragOver }"
  >
    <!-- Toolbar -->
    <AppToolbar 
      :logFile="logFile"
      @openFile="handleOpenFile"
      @toggleTheme="handleToggleTheme"
      @openSettings="handleOpenSettings"
    />

    <!-- Filters Panel -->
    <FiltersPanel 
      :logLevels="logFile?.logLevels || []"
      @updateFilters="handleUpdateFilters"
    />

    <!-- Main Content -->
    <div class="main-content">
      <Splitter>
        <!-- Log Table -->
        <SplitterPanel :size="70" :minSize="50">
          <LogTable 
            :logEntries="filteredEntries"
            :selectedEntry="selectedEntry"
            :loading="loading"
            @entrySelect="handleEntrySelect"
          />
        </SplitterPanel>

        <!-- Details Panel -->
        <SplitterPanel :size="30" :minSize="25">
          <DetailsPanel :selectedEntry="selectedEntry" />
        </SplitterPanel>
      </Splitter>
    </div>

    <!-- Status Bar -->
    <StatusBar 
      :logFile="logFile"
      :logEntries="filteredEntries"
      :selectedEntry="selectedEntry"
    />
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  gap: 8px;
  padding: 8px;
  background: var(--p-surface-ground);
  overflow: hidden;
  box-sizing: border-box;
  position: relative;
  transition: all 0.2s ease;
}

.app-container.drag-over {
  background: var(--p-primary-50);
  border: 2px dashed var(--p-primary-500);
}

.app-container.drag-over::after {
  content: "Drop log files here to open them";
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--p-primary-500);
  background: var(--p-surface-0);
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  z-index: 1000;
  pointer-events: none;
}

.main-content {
  flex: 1;
  min-height: 0;
}
</style>

<style>
/* Reset default styles and ensure full viewport usage */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  height: 100%;
  overflow: hidden;
  font-family: system-ui, -apple-system, sans-serif;
}

#app {
  height: 100vh;
  overflow: hidden;
  font-family: system-ui, -apple-system, sans-serif;
}

/* Ensure all PrimeVue components use system fonts */
.p-component {
  font-family: inherit !important;
}

/* Global styles for PrimeVue components */
.p-toolbar {
  border: none;
  background: transparent !important;
}

.p-card {
  border: none;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.p-card .p-card-content {
  padding: 1rem;
}

.p-splitter {
  border: none;
  height: 100%;
}

.p-splitter .p-splitter-panel {
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Prevent text selection when dragging splitter */
.p-splitter-gutter {
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  cursor: col-resize;
}

/* Only prevent selection during actual splitter dragging */
body.splitter-dragging * {
  user-select: none !important;
  -webkit-user-select: none !important;
  -moz-user-select: none !important;
  -ms-user-select: none !important;
}

/* Allow the splitter gutter itself to remain interactive */
body.splitter-dragging .p-splitter-gutter {
  cursor: col-resize !important;
}

/* Ensure normal text selection is preserved when not dragging */
.p-datatable tbody tr td,
.details-content,
.property-value {
  user-select: text;
  -webkit-user-select: text;
  -moz-user-select: text;
  -ms-user-select: text;
}

.p-datatable {
  border: none;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.p-datatable .p-datatable-wrapper {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  min-height: 0;
}

.p-datatable .p-datatable-table {
  width: 100%;
  table-layout: fixed;
}

.p-datatable .p-datatable-header {
  background: transparent;
  border: none;
}

.p-datatable .p-datatable-tbody > tr {
  background: transparent;
  height: 36px;
}

.p-datatable .p-datatable-tbody > tr td {
  padding: 6px 0;
  font-size: 14px;
  line-height: 1.2;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  vertical-align: middle;
}

.p-datatable .p-datatable-tbody > tr td:first-child {
  padding-left: 12px;
}

.p-datatable .p-datatable-tbody > tr td:last-child {
  padding-right: 12px;
}

.p-datatable .p-datatable-tbody > tr:hover {
  background: rgba(0, 0, 0, 0.02);
}

.p-datatable .p-datatable-tbody > tr.p-highlight {
  background: rgba(33, 150, 243, 0.1);
}

.p-datatable .p-datatable-thead > tr > th {
  padding: 8px 0;
  font-size: 14px;
  font-weight: 600;
}

.p-datatable .p-datatable-thead > tr > th:first-child {
  padding-left: 12px;
}

.p-datatable .p-datatable-thead > tr > th:last-child {
  padding-right: 12px;
}

.p-button {
  border-radius: 6px;
}

.p-inputtext {
  border-radius: 6px;
}

.p-multiselect {
  border-radius: 6px;
}

.p-calendar {
  border-radius: 6px;
}

.p-tag {
  border-radius: 12px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  vertical-align: middle;
}
</style>