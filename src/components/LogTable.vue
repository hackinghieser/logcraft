<script setup lang="ts">
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Button from "primevue/button";
import Card from "primevue/card";
import Tag from "primevue/tag";

interface LogEntry {
  timestamp: string;
  level: string;
  message: string;
  template?: string;
  exception?: string;
  eventId?: string;
  properties?: Record<string, any>;
}

interface Props {
  logEntries: LogEntry[];
  selectedEntry?: LogEntry | null;
  loading?: boolean;
}

withDefaults(defineProps<Props>(), {
  selectedEntry: null,
  loading: false
});

const emit = defineEmits<{
  entrySelect: [entry: LogEntry];
}>();

function onRowSelect(event: any) {
  emit('entrySelect', event.data);
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
</script>

<template>
  <Card class="logs-table-card">
    <template #title>
      <div class="table-header">
        <i class="pi pi-list"></i>
        Log Entries
      </div>
    </template>
    
    <template #content>
      <div v-if="loading" class="loading-overlay">
        <div class="loading-container">
          <div class="loading-spinner">
            <div class="spinner-ring"></div>
            <div class="spinner-ring"></div>
            <div class="spinner-ring"></div>
          </div>
          <p class="loading-text">Loading log entries...</p>
        </div>
      </div>
      
      <DataTable
        v-else
        :value="logEntries"
        :selection="selectedEntry"
        selectionMode="single"
        @row-select="onRowSelect"
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
</template>

<style scoped>
.logs-table-card {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  position: relative;
}

.logs-table-card :deep(.p-card-content),
.logs-table-card :deep(.p-card-body) {
  padding: 0;
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
}

.table-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
  color: var(--p-text-color);
  padding: 12px 16px;
  border-bottom: 1px solid var(--p-surface-border);
}

.logs-table {
  flex: 1;
  min-height: 0;
}

.timestamp {
  font-size: 11px;
  color: var(--p-text-muted-color);
}

.level-tag {
  font-size: 11px;
  font-weight: 600;
  display: inline-flex;
  align-items: center;
  vertical-align: middle;
}

.message-text {
  font-size: 13px;
  line-height: 1.3;
  color: var(--p-text-color);
}

/* Modern Loading Animation */
.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24px;
}

.loading-spinner {
  position: relative;
  width: 60px;
  height: 60px;
}

.spinner-ring {
  position: absolute;
  width: 60px;
  height: 60px;
  border-radius: 50%;
  border: 3px solid transparent;
  border-top: 3px solid var(--p-primary-500);
  animation: spin 1.2s linear infinite;
}

.spinner-ring:nth-child(1) {
  animation-delay: 0s;
  opacity: 1;
}

.spinner-ring:nth-child(2) {
  animation-delay: -0.4s;
  opacity: 0.6;
  transform: scale(0.8);
  border-top-color: var(--p-primary-400);
}

.spinner-ring:nth-child(3) {
  animation-delay: -0.8s;
  opacity: 0.3;
  transform: scale(0.6);
  border-top-color: var(--p-primary-300);
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.loading-text {
  color: var(--p-text-muted-color);
  font-size: 16px;
  font-weight: 500;
  margin: 0;
  text-align: center;
  letter-spacing: 0.5px;
}
</style>