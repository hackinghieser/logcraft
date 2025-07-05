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
const sampleEntries: LogEntry[] = [
  {
    timestamp: "2024-01-15T10:30:45.123Z",
    level: "Error",
    message: "Database connection failed: timeout after 30s",
    template: "Database connection failed: {ErrorDetails}",
    exception: "System.TimeoutException: The operation has timed out.\n   at Database.Connect()\n   at Service.Initialize()",
    properties: { ErrorDetails: "timeout after 30s", ConnectionString: "postgresql://...", RetryCount: 3 }
  },
  {
    timestamp: "2024-01-15T10:30:44.891Z",
    level: "Warning",
    message: "Retrying connection attempt 3/5",
    template: "Retrying connection attempt {Attempt}/{MaxAttempts}",
    properties: { Attempt: 3, MaxAttempts: 5 }
  },
  {
    timestamp: "2024-01-15T10:30:43.567Z",
    level: "Information",
    message: "User alice logged in from IP 192.168.1.100",
    template: "User {Username} logged in from IP {IPAddress}",
    properties: { Username: "alice", IPAddress: "192.168.1.100" }
  },
  {
    timestamp: "2024-01-15T10:30:42.234Z",
    level: "Debug",
    message: "Processing order #12345 for customer CUST-789",
    template: "Processing order #{OrderId} for customer {CustomerId}",
    properties: { OrderId: "12345", CustomerId: "CUST-789", Amount: 99.99, Currency: "USD" }
  },
  {
    timestamp: "2024-01-15T10:30:41.098Z",
    level: "Information",
    message: "Application startup completed in 1.23s",
    template: "Application startup completed in {Duration}",
    properties: { Duration: "1.23s" }
  }
];

// Initialize with sample data
logEntries.value = sampleEntries;
filteredEntries.value = [...sampleEntries];
selectedEntry.value = sampleEntries[0];
logFile.value = {
  path: "events.clef",
  totalCount: 15432,
  logLevels: ["Error", "Warning", "Information", "Debug"],
  dateRange: ["2024-01-15T10:30:00.000Z", "2024-01-15T10:35:00.000Z"]
};

// Event handlers
function handleOpenFile() {
  // TODO: Implement file opening with Tauri
  console.log("Opening file...");
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

function handleClearFilters() {
  filteredEntries.value = [...logEntries.value];
  selectedEntry.value = logEntries.value.length > 0 ? logEntries.value[0] : null;
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

onMounted(() => {
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
});

onUnmounted(() => {
  document.body.classList.remove('splitter-dragging');
});
</script>

<template>
  <div class="app-container">
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
      @clearFilters="handleClearFilters"
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
}

#app {
  height: 100vh;
  overflow: hidden;
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
  overflow: auto;
  height: 100%;
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
}
</style>