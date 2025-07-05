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
      <DataTable
        :value="logEntries"
        :selection="selectedEntry"
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
</template>

<style scoped>
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
</style>