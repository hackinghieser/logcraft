<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// Components
import Toolbar from "primevue/toolbar";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import MultiSelect from "primevue/multiselect";
import Calendar from "primevue/calendar";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Splitter from "primevue/splitter";
import SplitterPanel from "primevue/splitterpanel";
import Card from "primevue/card";
import Tag from "primevue/tag";

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
const selectedEntry = ref<LogEntry | null>(null);
const selectedLevels = ref<string[]>([]);
const searchText = ref("");
const dateRange = ref<Date[]>([]);
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
selectedEntry.value = sampleEntries[0];
logFile.value = {
  path: "events.clef",
  totalCount: 15432,
  logLevels: ["Error", "Warning", "Information", "Debug"],
  dateRange: ["2024-01-15T10:30:00.000Z", "2024-01-15T10:35:00.000Z"]
};

// Methods
async function openFile() {
  // TODO: Implement file opening with Tauri
  console.log("Opening file...");
}

function onRowSelect(event: any) {
  selectedEntry.value = event.data;
}

function clearFilters() {
  selectedLevels.value = [];
  searchText.value = "";
  dateRange.value = [];
}

function formatTimestamp(timestamp: string): string {
  return new Date(timestamp).toLocaleString();
}

function getLevelSeverity(level: string): "success" | "info" | "warning" | "danger" {
  switch (level.toLowerCase()) {
    case "error": return "danger";
    case "warning": return "warning";
    case "information": return "info";
    case "debug": return "success";
    default: return "info";
  }
}

async function testCleverlib() {
  try {
    const result = await invoke("test_cleverlib_parsing");
    console.log("Cleverlib test result:", result);
  } catch (error) {
    console.error("Cleverlib test error:", error);
  }
}
</script>

<template>
  <div class="app-container">
    <!-- Toolbar -->
    <Toolbar class="app-toolbar">
      <template #start>
        <div class="toolbar-left">
          <h1 class="app-title">CLEF Log Viewer</h1>
          <div class="file-info" v-if="logFile">
            <i class="pi pi-file"></i>
            {{ logFile.path }} • {{ logFile.totalCount.toLocaleString() }} entries
          </div>
        </div>
      </template>
      
      <template #end>
        <div class="toolbar-right">
          <Button icon="pi pi-folder-open" label="Open File" @click="openFile" />
          <Button icon="pi pi-moon" label="Theme" text />
          <Button icon="pi pi-cog" label="Settings" text />
          <Button icon="pi pi-code" label="Test Cleverlib" @click="testCleverlib" outlined />
        </div>
      </template>
    </Toolbar>

    <!-- Filters Panel -->
    <Card class="filters-panel">
      <template #content>
        <div class="filters-grid">
          <div class="filter-group">
            <label>Log Level</label>
            <MultiSelect
              v-model="selectedLevels"
              :options="logFile?.logLevels || []"
              placeholder="All Levels"
              class="filter-control"
            />
          </div>
          
          <div class="filter-group">
            <label>Search Text</label>
            <InputText
              v-model="searchText"
              placeholder="Search messages..."
              class="filter-control"
            />
          </div>
          
          <div class="filter-group">
            <label>Date Range</label>
            <Calendar
              v-model="dateRange"
              selectionMode="range"
              :showIcon="true"
              dateFormat="yy-mm-dd"
              class="filter-control"
            />
          </div>
          
          <div class="filter-group">
            <label>Actions</label>
            <Button
              icon="pi pi-times"
              label="Clear Filters"
              @click="clearFilters"
              outlined
              class="filter-control"
            />
          </div>
        </div>
      </template>
    </Card>

    <!-- Main Content -->
    <div class="main-content">
      <Splitter>
        <!-- Log Table -->
        <SplitterPanel :size="70" :minSize="50">
          <Card class="logs-table-card">
            <template #title>
              <div class="table-header">
                <i class="pi pi-list"></i>
                Log Entries
              </div>
            </template>
            
            <template #content>
              <DataTable
                v-model:selection="selectedEntry"
                :value="logEntries"
                selectionMode="single"
                @row-select="onRowSelect"
                :loading="loading"
                scrollable
                scrollHeight="flex"
                class="logs-table"
              >
                <Column field="timestamp" header="Timestamp" :sortable="true" style="width: 200px">
                  <template #body="slotProps">
                    <span class="timestamp">
                      {{ formatTimestamp(slotProps.data.timestamp) }}
                    </span>
                  </template>
                </Column>
                
                <Column field="level" header="Level" :sortable="true" style="width: 100px">
                  <template #body="slotProps">
                    <Tag
                      :value="slotProps.data.level"
                      :severity="getLevelSeverity(slotProps.data.level)"
                      class="level-tag"
                    />
                  </template>
                </Column>
                
                <Column field="message" header="Message" :sortable="true">
                  <template #body="slotProps">
                    <span class="message-text">{{ slotProps.data.message }}</span>
                  </template>
                </Column>
                
                <Column header="Actions" style="width: 80px">
                  <template #body>
                    <Button
                      icon="pi pi-eye"
                      size="small"
                      text
                      rounded
                      aria-label="View Details"
                    />
                  </template>
                </Column>
              </DataTable>
            </template>
          </Card>
        </SplitterPanel>

        <!-- Details Panel -->
        <SplitterPanel :size="30" :minSize="25">
          <Card class="details-panel-card">
            <template #title>
              <div class="panel-header">
                <i class="pi pi-info-circle"></i>
                Entry Details
              </div>
            </template>
            
            <template #content>
              <div v-if="selectedEntry" class="details-content">
                <div class="property-group">
                  <div class="property-title">
                    <i class="pi pi-clock"></i>
                    Timestamp
                  </div>
                  <div class="property-value">{{ selectedEntry.timestamp }}</div>
                </div>
                
                <div class="property-group">
                  <div class="property-title">
                    <i class="pi pi-tag"></i>
                    Level
                  </div>
                  <div class="property-value">
                    <Tag
                      :value="selectedEntry.level"
                      :severity="getLevelSeverity(selectedEntry.level)"
                    />
                  </div>
                </div>
                
                <div class="property-group" v-if="selectedEntry.template">
                  <div class="property-title">
                    <i class="pi pi-file-edit"></i>
                    Message Template
                  </div>
                  <div class="property-value template-text">{{ selectedEntry.template }}</div>
                </div>
                
                <div class="property-group" v-if="selectedEntry.properties">
                  <div class="property-title">
                    <i class="pi pi-cog"></i>
                    Properties
                  </div>
                  <div class="property-value json-text">
                    {{ JSON.stringify(selectedEntry.properties, null, 2) }}
                  </div>
                </div>
                
                <div class="property-group" v-if="selectedEntry.exception">
                  <div class="property-title">
                    <i class="pi pi-exclamation-triangle"></i>
                    Exception
                  </div>
                  <div class="property-value exception-text">{{ selectedEntry.exception }}</div>
                </div>
              </div>
              
              <div v-else class="no-selection">
                <i class="pi pi-info-circle"></i>
                <p>Select a log entry to view details</p>
              </div>
            </template>
          </Card>
        </SplitterPanel>
      </Splitter>
    </div>

    <!-- Status Bar -->
    <div class="status-bar">
      <div class="status-left">
        <span v-if="logFile">
          <i class="pi pi-chart-bar"></i>
          {{ logFile.totalCount.toLocaleString() }} total entries
        </span>
        <span v-if="logEntries.length !== logFile?.totalCount">
          <i class="pi pi-search"></i>
          {{ logEntries.length }} filtered
        </span>
        <span>
          <i class="pi pi-bolt"></i>
          Ready
        </span>
      </div>
      
      <div class="status-right">
        <span v-if="selectedEntry">
          <i class="pi pi-map-marker"></i>
          Entry {{ logEntries.indexOf(selectedEntry) + 1 }} of {{ logEntries.length }}
        </span>
        <span v-if="logFile">
          <i class="pi pi-refresh"></i>
          Loaded
        </span>
      </div>
    </div>
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

.app-toolbar {
  background: var(--p-surface-card);
  border: 1px solid var(--p-surface-border);
  border-radius: 6px;
  padding: 16px;
}

.toolbar-left {
  display: flex;
  align-items: center;
  gap: 20px;
}

.app-title {
  font-size: 24px;
  font-weight: 700;
  margin: 0;
  color: var(--p-text-color);
}

.file-info {
  font-size: 14px;
  color: var(--p-text-muted-color);
  display: flex;
  align-items: center;
  gap: 8px;
}

.toolbar-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.filters-panel {
  border-radius: 6px;
  background: var(--p-surface-card);
  border: 1px solid var(--p-surface-border);
}

.filters-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  align-items: end;
}

.filter-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.filter-group label {
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
  color: var(--p-text-muted-color);
  letter-spacing: 0.5px;
}

.filter-control {
  width: 100%;
}

.main-content {
  flex: 1;
  min-height: 0;
}

.logs-table-card {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.table-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
  color: var(--p-text-color);
}

.logs-table {
  flex: 1;
}

.timestamp {
  font-family: 'Courier New', monospace;
  font-size: 12px;
  color: var(--p-text-muted-color);
}

.level-tag {
  font-size: 11px;
  font-weight: 600;
}

.message-text {
  font-size: 14px;
  line-height: 1.4;
  color: var(--p-text-color);
}

.details-panel-card {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.panel-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
  color: var(--p-text-color);
}

.details-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
  overflow-y: auto;
  flex: 1;
}

.property-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.property-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
  color: var(--p-text-color);
  font-size: 14px;
}

.property-value {
  background: var(--p-surface-50);
  border: 1px solid var(--p-surface-border);
  padding: 12px;
  border-radius: 6px;
  font-size: 13px;
  word-break: break-word;
  color: var(--p-text-color);
}

.template-text {
  font-family: 'Courier New', monospace;
  background: var(--p-primary-50);
  border-left: 4px solid var(--p-primary-color);
}

.json-text {
  font-family: 'Courier New', monospace;
  background: var(--p-surface-100);
  border-left: 4px solid var(--p-text-muted-color);
  white-space: pre-wrap;
}

.exception-text {
  font-family: 'Courier New', monospace;
  background: var(--p-red-50);
  border-left: 4px solid var(--p-red-500);
  white-space: pre-wrap;
}

.no-selection {
  text-align: center;
  color: var(--p-text-muted-color);
  padding: 40px 20px;
}

.no-selection i {
  font-size: 48px;
  margin-bottom: 16px;
  opacity: 0.5;
}

.status-bar {
  height: 40px;
  background: var(--p-surface-card);
  border: 1px solid var(--p-surface-border);
  color: var(--p-text-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
  font-size: 13px;
  border-radius: 6px;
}

.status-left,
.status-right {
  display: flex;
  align-items: center;
  gap: 20px;
}

.status-left span,
.status-right span {
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--p-text-muted-color);
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

.p-datatable {
  border: none;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.p-datatable .p-datatable-wrapper {
  flex: 1;
  overflow: auto;
}

.p-datatable .p-datatable-header {
  background: transparent;
  border: none;
}

.p-datatable .p-datatable-tbody > tr {
  background: transparent;
}

.p-datatable .p-datatable-tbody > tr:hover {
  background: rgba(0, 0, 0, 0.02);
}

.p-datatable .p-datatable-tbody > tr.p-highlight {
  background: rgba(33, 150, 243, 0.1);
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