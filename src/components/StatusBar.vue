<script setup lang="ts">
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

interface Props {
  logFile?: LogFileInfo | null;
  logEntries: LogEntry[];
  selectedEntry?: LogEntry | null;
  loadingMore?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  logFile: null,
  selectedEntry: null,
  loadingMore: false,
});

function getSelectedEntryIndex(): number {
  if (!props.selectedEntry) return -1;
  return props.logEntries.indexOf(props.selectedEntry);
}
</script>

<template>
  <div class="status-bar">
    <div class="status-left">
      <span v-if="logFile">
        <i class="pi pi-chart-bar" />
        {{ logFile.totalCount.toLocaleString() }} total entries
      </span>
      <span v-if="logEntries.length !== logFile?.totalCount">
        <i class="pi pi-search" />
        {{ logEntries.length }} filtered
      </span>
    </div>

    <div class="status-right">
      <span
        v-if="loadingMore"
        class="loading-indicator"
      >
        <i class="pi pi-spin pi-spinner" />
        Loading more entries...
      </span>
      <span v-else-if="selectedEntry">
        <i class="pi pi-map-marker" />
        Entry {{ getSelectedEntryIndex() + 1 }} of {{ logEntries.length }}
      </span>
    </div>
  </div>
</template>

<style scoped>
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

.loading-indicator {
  color: var(--p-primary-500) !important;
  font-weight: 500;
}
</style>
