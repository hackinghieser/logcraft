<script setup lang="ts">
import { ref, onMounted, onUnmounted, markRaw } from "vue";
import Splitter from "primevue/splitter";
import SplitterPanel from "primevue/splitterpanel";

// Components
import AppToolbar from "./components/AppToolbar.vue";
import FiltersPanel from "./components/FiltersPanel.vue";
import LogTable from "./components/LogTable.vue";
import DetailsPanel from "./components/DetailsPanel.vue";
import StatusBar from "./components/StatusBar.vue";

import Filters from "./types/Filters";
import LogEntry from "./types/LogEntry";
import LogFileInfo from "./types/LogFileInfo";

// Reactive data
const logFile = ref<LogFileInfo | null>(null);
const logEntries = ref<LogEntry[]>([]);
const filteredEntries = ref<LogEntry[]>([]);
const selectedEntry = ref<LogEntry | null>(null);
const loading = ref(false);

// Filter state
const currentFilters = ref<Filters>({
  selectedLevels: [],
  searchText: "",
  dateRange: [],
});

// Loading state (pagination removed - now loads entire file)
const hasMorePages = ref(false);
const loadingMore = ref(false);

// Sample data for development

// Drag and drop state
const isDragOver = ref(false);

// Event handlers
async function handleOpenFile() {
  try {
    // Import Tauri APIs
    const { open } = await import("@tauri-apps/plugin-dialog");

    // Open file dialog
    const selected = await open({
      filters: [
        {
          name: "CLEF Log Files",
          extensions: ["clef", "log", "txt"],
        },
        {
          name: "All Files",
          extensions: ["*"],
        },
      ],
    });

    if (selected) {
      await handleFileOpen(selected);
    }
  } catch (error) {
    console.error("Failed to open file dialog:", error);
    alert(`Failed to open file dialog: ${error}`);
  }
}

// Tauri file drop handlers using 2.0 API
async function setupFileDropListener() {
  try {
    const { getCurrentWebview } = await import("@tauri-apps/api/webview");

    // Set up drag and drop event listener
    const unlisten = await getCurrentWebview().onDragDropEvent((event: any) => {
      switch (event.payload.type) {
        case "over":
          isDragOver.value = true;
          break;
        case "drop":
          isDragOver.value = false;
          if (event.payload.paths && event.payload.paths.length > 0) {
            handleFileDropped(event.payload.paths[0]);
          }
          break;
        case "cancel":
          isDragOver.value = false;
          break;
      }
    });

    // Return cleanup function
    return unlisten;
  } catch (error) {
    console.error("Failed to setup file drop listener:", error);
    return () => {};
  }
}

async function handleFileDropped(filePath: string) {
  isDragOver.value = false;

  // Check if file is a log file
  const validExtensions = ["clef", "log", "txt"];
  const fileExtension = filePath.split(".").pop()?.toLowerCase();

  if (fileExtension && validExtensions.includes(fileExtension)) {
    await handleFileOpen(filePath);
  } else {
    alert("Please drop a valid log file (.clef, .log, or .txt)");
  }
}

// Load more function (now deprecated - kept for compatibility)
async function loadMoreEntries() {
  // No longer needed since we load the entire file at once
  console.log("loadMoreEntries called but pagination is disabled - entire file is already loaded");
}

// Extract file opening logic to reuse for both dialog and drag & drop
async function handleFileOpen(filePath: string) {
  console.log("Start loading");
  loading.value = true;
  try {
    // Import Tauri APIs
    const { invoke } = await import("@tauri-apps/api/core");

    // Reset loading state
    hasMorePages.value = false;
    loadingMore.value = false;

    // Parse the selected file (now returns all entries)
    const result = (await invoke("parse_clef_file", { filePath })) as [
      any,
      any[],
    ];
    const [fileInfo, events] = result;
    console.log(result)
    // Update the application state
    logFile.value = {
      path: fileInfo.path,
      totalCount: fileInfo.total_count,
      logLevels: fileInfo.log_levels,
      dateRange: fileInfo.date_range,
    };

    logEntries.value = events;
    console.log(logEntries)
    //applyFilters(); // Apply filters after initial load
    selectedEntry.value = events.length > 0 ? events[0] : null;

    // No more pagination needed - entire file is loaded
    hasMorePages.value = false;

    console.log(
      `Loaded entire file: ${events.length} log entries from ${fileInfo.path}`,
    );
  } catch (error) {
    console.error("Failed to parse log file:", error);
    alert(`Failed to parse log file: ${error}`);
  } finally {
    console.log("Finished loading, false");
    loading.value = false;
  }
}

function handleToggleTheme() {
  const newTheme = (window as any).toggleTheme?.();
  console.log(`Theme switched to: ${newTheme}`);
}

function handleOpenSettings() {
  // TODO: Implement settings dialog
  console.log("Open settings...");
}

function applyFilters() {
  let filtered = [...logEntries.value];
  const filters = currentFilters.value;

  // Filter by log levels
  if (filters.selectedLevels.length > 0) {
    filtered = filtered.filter((entry) =>
      filters.selectedLevels.includes(entry['@l'] || ''),
    );
  }

  // Filter by search text
  if (filters.searchText.trim()) {
    const searchTerm = filters.searchText.toLowerCase();
    filtered = filtered.filter(
      (entry) =>
        entry['@m']?.toLowerCase().includes(searchTerm) ||
        entry['@mt']?.toLowerCase().includes(searchTerm) ||
        entry['@l']?.toLowerCase().includes(searchTerm),
    );
  }

  // Filter by date range
  if (filters.dateRange.length === 2) {
    const [startDate, endDate] = filters.dateRange;
    filtered = filtered.filter((entry) => {
      const entryDate = new Date(entry['@t'] || '');
      return entryDate >= startDate && entryDate <= endDate;
    });
  }

  filteredEntries.value = filtered;

  // Reset selection if current selection is not in filtered results
  if (selectedEntry.value && !filtered.includes(selectedEntry.value)) {
    selectedEntry.value = filtered.length > 0 ? filtered[0] : null;
  }
}

function handleUpdateFilters(filters: {
  selectedLevels: string[];
  searchText: string;
  dateRange: Date[];
}) {
  currentFilters.value = filters;
  applyFilters();
}

function handleEntrySelect(entry: LogEntry) {
  selectedEntry.value = entry;
}

// Handle splitter drag to prevent text selection
let isDraggingSplitter = false;

function handleSplitterStart() {
  isDraggingSplitter = true;
  document.body.classList.add("splitter-dragging");
}

function handleSplitterEnd() {
  isDraggingSplitter = false;
  document.body.classList.remove("splitter-dragging");
}

onMounted(async () => {
  // Setup file drop listener
  const cleanupFileDropListener = await setupFileDropListener();

  // Add global event listeners for splitter events
  document.addEventListener("mousedown", (e) => {
    if ((e.target as HTMLElement)?.closest(".p-splitter-gutter")) {
      handleSplitterStart();
    }
  });

  document.addEventListener("mouseup", () => {
    if (isDraggingSplitter) {
      handleSplitterEnd();
    }
  });

  document.addEventListener("mouseleave", () => {
    if (isDraggingSplitter) {
      handleSplitterEnd();
    }
  });

  // Additional safety: clear on any click outside splitter
  document.addEventListener("click", (e) => {
    if (!(e.target as HTMLElement)?.closest(".p-splitter-gutter")) {
      handleSplitterEnd();
    }
  });

  // Store cleanup function for unmount
  (window as any)._cleanupFileDropListener = cleanupFileDropListener;
});

onUnmounted(() => {
  document.body.classList.remove("splitter-dragging");

  // Cleanup file drop listener
  if ((window as any)._cleanupFileDropListener) {
    (window as any)._cleanupFileDropListener();
  }
});
</script>

<template>
  <div class="app-container" :class="{ 'drag-over': isDragOver }">
    <!-- Toolbar -->
    <AppToolbar
      :log-file="logFile"
      @open-file="handleOpenFile"
      @toggle-theme="handleToggleTheme"
      @open-settings="handleOpenSettings" />

    <!-- Filters Panel -->
    <FiltersPanel
      :log-levels="logFile?.logLevels || []"
      :initial-filters="currentFilters"
      @update-filters="handleUpdateFilters" />

    <!-- Main Content -->
    <div class="main-content">
      <Splitter>
        <!-- Log Table -->
        <SplitterPanel :size="80" :min-size="60">
          <LogTable
            :log-entries="logEntries"
            :selected-entry="selectedEntry"
            :loading="loading"
            :has-more-pages="hasMorePages"
            :loading-more="loadingMore"
            @entry-select="handleEntrySelect"
            @load-more="loadMoreEntries" />
        </SplitterPanel>

        <!-- Details Panel -->
        <SplitterPanel :size="20" :min-size="25">
          <DetailsPanel :selected-entry="selectedEntry" />
        </SplitterPanel>
      </Splitter>
    </div>

    <!-- Status Bar -->
    <StatusBar
      :log-file="logFile"
      :log-entries="filteredEntries"
      :selected-entry="selectedEntry"
      :loading-more="loadingMore" />
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
  box-shadow: 0 4px 12px var(--p-surface-border);
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

html,
body {
  height: 100%;
  overflow: hidden;
  font-family:
    system-ui,
    -apple-system,
    sans-serif;
}

#app {
  height: 100vh;
  overflow: hidden;
  font-family:
    system-ui,
    -apple-system,
    sans-serif;
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
  box-shadow: 0 2px 8px var(--p-surface-border);
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
  height: 32px;
  transition: all 0.2s ease;
  border-bottom: 1px solid var(--p-surface-border);
}

.p-datatable .p-datatable-tbody > tr td {
  padding: 6px 0;
  font-size: 13px;
  line-height: 1.3;
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
  background: var(--p-surface-hover);
}

.p-datatable .p-datatable-tbody > tr.p-highlight {
  background: var(--p-primary-50);
  border-left: 3px solid var(--p-primary-500);
}

.p-datatable .p-datatable-thead > tr > th {
  padding: 10px 0;
  font-size: 12px;
  font-weight: 600;
  background: var(--p-surface-50);
  border-bottom: 2px solid var(--p-surface-border);
  color: var(--p-text-muted-color);
  text-transform: uppercase;
  letter-spacing: 0.5px;
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
