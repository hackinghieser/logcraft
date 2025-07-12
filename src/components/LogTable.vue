<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, watch } from "vue";
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

function onRowSelect(event: DataTableRowClickEvent) {
  emit("entrySelect", event.data);
}

function formatTimestamp(timestamp: string): string {
  return new Date(timestamp).toLocaleString();
}

function getLevelSeverity(
  level: string,
): "success" | "info" | "warning" | "danger" {
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

// Scroll-based pagination
const dataTableRef = ref<any>(null);

function handleScroll(event: Event) {
  if (props.loadingMore || !props.hasMorePages || props.loading) {
    return;
  }

  const element = event.target as HTMLElement;
  const threshold = 200; // Trigger load when 200px from bottom (earlier)

  if (
    element.scrollTop + element.clientHeight >=
    element.scrollHeight - threshold
  ) {
    emit("loadMore");
  }
}

let scrollElement: HTMLElement | null = null;

const setupScrollListener = async () => {
  // Wait for the DOM to be fully updated
  await nextTick();

  // Remove existing listener if any
  if (scrollElement) {
    scrollElement.removeEventListener("scroll", handleScroll);
    scrollElement = null;
  }

  // Method 1: Via DataTable ref
  if (dataTableRef.value && dataTableRef.value.$el) {
    const wrapper = dataTableRef.value.$el.querySelector(
      ".p-datatable-wrapper",
    );
    if (wrapper) {
      scrollElement = wrapper as HTMLElement;
      wrapper.addEventListener("scroll", handleScroll, { passive: true });
      return true;
    }
  }

  // Method 2: Direct class search
  const wrapper = document.querySelector(".p-datatable-wrapper");
  if (wrapper) {
    scrollElement = wrapper as HTMLElement;
    wrapper.addEventListener("scroll", handleScroll, { passive: true });
    return true;
  }

  // Method 3: Look for any element with overflow
  const scrollables = document.querySelectorAll(
    '[style*="overflow"], .p-datatable-wrapper',
  );
  for (const element of scrollables) {
    const styles = window.getComputedStyle(element);
    if (styles.overflow === "auto" || styles.overflowY === "auto") {
      scrollElement = element as HTMLElement;
      element.addEventListener("scroll", handleScroll, { passive: true });
      return true;
    }
  }

  return false;
};

// Watch for when logEntries change (data is loaded) and setup scroll listener
watch(
  () => props.logEntries.length,
  async (newLength) => {
    if (newLength > 0) {
      await setupScrollListener();
    }
  },
);

onMounted(async () => {
  // Try multiple times with different delays
  for (let attempt = 0; attempt < 5; attempt++) {
    await new Promise((resolve) => setTimeout(resolve, attempt * 200));
    if (await setupScrollListener()) {
      break;
    }
  }
});

onUnmounted(() => {
  if (scrollElement) {
    scrollElement.removeEventListener("scroll", handleScroll);
    scrollElement = null;
  }
});
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
        scrollable
        scroll-height="flex"
        class="logs-table"
        @row-select="onRowSelect">
        <Column
          field="timestamp"
          header="Timestamp"
          :sortable="true"
          style="width: 200px">
          <template #body="slotProps">
            <span class="timestamp">
              {{ formatTimestamp(slotProps.data.timestamp) }}
            </span>
          </template>
        </Column>

        <Column
          field="level"
          header="Level"
          :sortable="true"
          style="width: 100px">
          <template #body="slotProps">
            <Tag
              :value="slotProps.data.level"
              :severity="getLevelSeverity(slotProps.data.level)"
              class="level-tag" />
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
              aria-label="View Details" />
          </template>
        </Column>
      </DataTable>

      <!-- Loading More Indicator -->
      <div v-if="loadingMore" class="loading-more-container">
        <div class="loading-more-spinner">
          <div class="spinner-ring" />
        </div>
        <p class="loading-more-text">Loading more entries...</p>
      </div>
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

/* Loading More Indicator */
.loading-more-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px;
  border-top: 1px solid var(--p-surface-border);
  background: var(--p-surface-card);
}

.loading-more-spinner {
  position: relative;
  width: 30px;
  height: 30px;
  margin-bottom: 12px;
}

.loading-more-spinner .spinner-ring {
  position: absolute;
  width: 30px;
  height: 30px;
  border-radius: 50%;
  border: 2px solid transparent;
  border-top: 2px solid var(--p-primary-500);
  animation: spin 1s linear infinite;
}

.loading-more-text {
  color: var(--p-text-muted-color);
  font-size: 14px;
  font-weight: 500;
  margin: 0;
  text-align: center;
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
  background: var(--p-primary-50) !important;
  color: var(--p-primary-700) !important;
}

.logs-table :deep(.p-datatable-tbody > tr.p-highlight:hover) {
  background: var(--p-primary-100) !important;
}
</style>
