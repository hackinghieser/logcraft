<script setup lang="ts">
import { ref, watch, computed } from "vue";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import MultiSelect from "primevue/multiselect";
import Calendar from "primevue/calendar";
import { Filters } from "../types/Filters";

interface Props {
  logLevels?: string[];
  initialFilters?: Filters;
}

// Common log levels used when no file is loaded
const DEFAULT_LOG_LEVELS = ["Error", "Warning", "Information", "Debug"];

const props = withDefaults(defineProps<Props>(), {
  logLevels: () => [],
  initialFilters: () => ({
    selectedLevels: [],
    searchText: "",
    dateRange: [],
  }),
});

// Use file-specific log levels if available, otherwise use defaults
const availableLogLevels = computed(() => {
  return props.logLevels && props.logLevels.length > 0
    ? props.logLevels
    : DEFAULT_LOG_LEVELS;
});

const emit = defineEmits<{
  updateFilters: [
    filters: {
      selectedLevels: string[];
      searchText: string;
      dateRange: Date[];
    },
  ];
}>();

const selectedLevels = ref<string[]>(props.initialFilters.selectedLevels);
const searchText = ref(props.initialFilters.searchText);
const dateRange = ref<Date[]>(props.initialFilters.dateRange);

function clearFilters() {
  selectedLevels.value = [];
  searchText.value = "";
  dateRange.value = [];
}

function emitFilters() {
  emit("updateFilters", {
    selectedLevels: selectedLevels.value,
    searchText: searchText.value,
    dateRange: dateRange.value,
  });
}

// Watch for changes and emit
watch(
  [selectedLevels, searchText, dateRange],
  () => {
    emitFilters();
  },
  { deep: true },
);
</script>

<template>
  <div class="filters-panel">
    <div class="filters-grid">
      <div class="filter-group">
        <label>Log Level</label>
        <MultiSelect
          v-model="selectedLevels"
          :options="availableLogLevels"
          :placeholder="
            props.logLevels && props.logLevels.length > 0
              ? 'All Levels'
              : 'All Levels (defaults)'
          "
          class="filter-control" />
      </div>

      <div class="filter-group">
        <label>Search Text</label>
        <InputText
          v-model="searchText"
          placeholder="Search messages..."
          class="filter-control" />
      </div>

      <div class="filter-group">
        <label>Date Range</label>
        <Calendar
          v-model="dateRange"
          selection-mode="range"
          :show-icon="true"
          date-format="yy-mm-dd"
          class="filter-control" />
      </div>

      <div class="filter-group">
        <label>Actions</label>
        <Button
          v-tooltip.top="'Clear all filters'"
          icon="pi pi-times"
          outlined
          class="filter-control"
          aria-label="Clear all filters"
          @click="clearFilters" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.filters-panel {
  background: var(--p-surface-card);
  border: 1px solid var(--p-surface-border);
  border-radius: 6px;
  padding: 16px;
}

.filters-grid {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr auto;
  gap: 20px;
  align-items: end;
}

@media (max-width: 768px) {
  .filters-grid {
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }
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
  height: 32px;
  display: flex;
  align-items: center;
}

/* Ensure consistent height for all PrimeVue components */
.filter-control :deep(.p-inputtext) {
  height: 32px;
  font-size: 14px;
}

.filter-control :deep(.p-multiselect) {
  height: 32px;
}

.filter-control :deep(.p-multiselect .p-multiselect-label) {
  font-size: 14px;
  padding: 6px 12px;
}

.filter-control :deep(.p-calendar),
.filter-control :deep(.p-datepicker-dropdown) {
  height: 32px;
}

.filter-control :deep(.p-calendar .p-inputgroup),
.filter-control :deep(.p-datepicker-dropdown .p-inputgroup) {
  height: 32px;
}

.filter-control :deep(.p-calendar .p-calendar-input-wrapper),
.filter-control :deep(.p-datepicker-dropdown .p-calendar-input-wrapper) {
  height: 32px;
  display: flex;
  align-items: stretch;
}

.filter-control :deep(.p-calendar .p-inputtext),
.filter-control :deep(.p-datepicker-dropdown .p-inputtext) {
  height: 32px;
  font-size: 14px;
  border-right: none;
  border-radius: 4px 0 0 4px;
}

.filter-control :deep(.p-calendar .p-calendar-button),
.filter-control :deep(.p-datepicker-dropdown .p-calendar-button) {
  height: 32px;
  width: 32px;
  border-radius: 0 4px 4px 0;
  border-left: none;
}

.filter-control :deep(.p-button) {
  height: 32px;
  justify-content: center;
  font-size: 14px;
}

/* Make clear filter button smaller and square */
.filter-group:last-child .filter-control {
  width: 32px;
}

.filter-group:last-child .filter-control :deep(.p-button) {
  width: 32px;
  height: 32px;
  min-width: 32px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* Align dropdown triggers */
.filter-control :deep(.p-multiselect-trigger) {
  height: 30px;
}

.filter-control :deep(.p-calendar-button .p-icon) {
  font-size: 16px;
  width: 16px;
  height: 16px;
}
</style>
