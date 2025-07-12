<script setup lang="ts">
import { ref, computed } from "vue";
import DataTable, { DataTableRowClickEvent } from "primevue/datatable";
import Column from "primevue/column";
import Button from "primevue/button";
import Tag from "primevue/tag";
import LogEntry from "../types/LogEntry";

interface Props {
  logEntries: LogEntry[];
  selectedEntry?: LogEntry | null;
  loading?: boolean;
  hasMorePages?: boolean;
  loadingMore?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  selectedEntry: null,
  loading: false,
  hasMorePages: false,
  loadingMore: false,
});

const emit = defineEmits<{
  entrySelect: [entry: LogEntry];
  loadMore: [];
}>();

// Pagination state
const currentPage = ref(0);
const rowsPerPage = 1000;

function onRowSelect(event: DataTableRowClickEvent) {
  emit("entrySelect", event.data);
}

function onSelectionChange(event: any) {
  if (event.value) {
    emit("entrySelect", event.value);
  }
}

function onRowClick(event: any) {
  emit("entrySelect", event.data);
}

function formatTimestamp(timestamp?: string): string {
  if (!timestamp) return '';
  return new Date(timestamp).toLocaleString();
}

function getLevelSeverity(
  level?: string,
): "success" | "info" | "warning" | "danger" {
  if (!level) return "info";
  switch (level.toLowerCase()) {
    case "error":
      return "danger";
    case "warning":
      return "warning";
    case "information":
      return "info";
    case "debug":
      return "success";
    default:
      return "info";
  }
}

// DataTable ref
const dataTableRef = ref<any>(null);

// Compute total rows for pagination
const totalRecords = computed(() => props.logEntries.length);

// Handle page change (simplified - no more backend loading needed)
function onPageChange(event: any) {
  currentPage.value = event.page;
}
</script>

<template>
  <div class="logs-table-container">
    <div class="table-header">
      <i class="pi pi-list" />
      Log Entries
    </div>

    <div class="table-content">
      <div v-if="loading" class="loading-overlay">
        <div class="loading-container">
          <div class="loading-spinner">
            <div class="spinner-ring" />
            <div class="spinner-ring" />
            <div class="spinner-ring" />
          </div>
          <p class="loading-text">Loading log entries...</p>
        </div>
      </div>

      <DataTable
        v-else
        ref="dataTableRef"
        :value="logEntries"
        :selection="selectedEntry"
        selection-mode="single"
        data-key="@t"
        paginator
        :rows="rowsPerPage"
        :total-records="totalRecords"
        :first="currentPage * rowsPerPage"
        :paginator-template="'FirstPageLink PrevPageLink PageLinks NextPageLink LastPageLink CurrentPageReport RowsPerPageDropdown'"
        current-page-report-template="Showing {first} to {last} of {totalRecords} entries"
        scrollable
        scroll-height="flex"
        class="logs-table"
        @row-select="onRowSelect"
        @selection-change="onSelectionChange"
        @row-click="onRowClick"
        @page="onPageChange">
        <Column
          field="@t"
          header="Timestamp"
          :sortable="true"
          style="width: 200px">
          <template #body="slotProps">
            <span class="timestamp">
              {{ formatTimestamp(slotProps.data['@t']) }}
            </span>
          </template>
        </Column>

        <Column
          field="@l"
          header="Level"
          :sortable="true"
          style="width: 100px">
          <template #body="slotProps">
            <Tag
              :value="slotProps.data['@l']"
              :severity="getLevelSeverity(slotProps.data['@l'])"
              class="level-tag" />
          </template>
        </Column>

        <Column field="@m" header="Message" :sortable="true">
          <template #body="slotProps">
            <span class="message-text">{{ slotProps.data['@m'] }}</span>
          </template>
        </Column>

        <Column header="Actions" style="width: 80px">
          <template #body>
            <Button
              icon="pi pi-eye"
              size="small"
              text
              rounded
              aria-label="View Details" />
          </template>
        </Column>
      </DataTable>
    </div>
  </div>
</template>

<style scoped>
.logs-table-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  position: relative;
}

.table-content {
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
  padding: 1rem;
  border-bottom: 1px solid var(--p-surface-border);
  font-size: 1.125rem;
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
  background: var(--p-surface-ground);
  opacity: 0.95;
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


/* DataTable header styling */
.logs-table :deep(.p-datatable-thead > tr > th) {
  background: var(--p-surface-card) !important;
  color: var(--p-text-color) !important;
  border-bottom: 1px solid var(--p-surface-border) !important;
  font-weight: 600;
}

.logs-table :deep(.p-datatable-tbody > tr) {
  background: var(--p-surface-card) !important;
  border-bottom: 1px solid var(--p-surface-border) !important;
}

.logs-table :deep(.p-datatable-tbody > tr:hover) {
  background: var(--p-surface-hover) !important;
}

.logs-table :deep(.p-datatable-tbody > tr.p-highlight) {
  background: var(--p-surface-100) !important;
  color: var(--p-text-color) !important;
  border-left: 4px solid var(--p-surface-400) !important;
  box-shadow: inset 0 0 0 1px var(--p-surface-300) !important;
  font-weight: 600 !important;
}

.logs-table :deep(.p-datatable-tbody > tr.p-highlight:hover) {
  background: var(--p-surface-200) !important;
  color: var(--p-text-color) !important;
  border-left: 4px solid var(--p-surface-500) !important;
}

.logs-table :deep(.p-datatable-tbody > tr.p-highlight td) {
  border-color: var(--p-surface-300) !important;
}

/* Enhanced row hover effect */
.logs-table :deep(.p-datatable-tbody > tr:hover:not(.p-highlight)) {
  background: var(--p-surface-hover) !important;
  cursor: pointer;
}

/* Additional selection styling for better visibility */
.logs-table :deep(.p-datatable-tbody > tr.p-highlight) {
  position: relative;
}

.logs-table :deep(.p-datatable-tbody > tr.p-highlight)::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  background: var(--p-surface-500);
  z-index: 1;
}

/* Dark mode improvements */
@media (prefers-color-scheme: dark) {
  .logs-table :deep(.p-datatable-tbody > tr.p-highlight) {
    background: rgba(255, 255, 255, 0.1) !important;
    border-left: 4px solid #6b7280 !important;
    box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.1) !important;
  }
  
  .logs-table :deep(.p-datatable-tbody > tr.p-highlight:hover) {
    background: rgba(255, 255, 255, 0.15) !important;
    border-left: 4px solid #9ca3af !important;
  }
}

/* Paginator styling */
.logs-table :deep(.p-paginator) {
  background: var(--p-surface-card) !important;
  border-top: 1px solid var(--p-surface-border) !important;
  padding: 0.75rem 1rem !important;
  border-radius: 0 0 6px 6px !important;
}

.logs-table :deep(.p-paginator .p-paginator-current) {
  color: var(--p-text-muted-color) !important;
  font-size: 0.875rem !important;
}

.logs-table :deep(.p-paginator .p-paginator-pages .p-paginator-page) {
  min-width: 2.5rem !important;
  height: 2.5rem !important;
  border-radius: 4px !important;
}

.logs-table :deep(.p-paginator .p-paginator-pages .p-paginator-page.p-highlight) {
  background: var(--p-primary-500) !important;
  color: white !important;
}
</style>
