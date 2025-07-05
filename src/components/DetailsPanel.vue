<script setup lang="ts">
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
  selectedEntry?: LogEntry | null;
}

withDefaults(defineProps<Props>(), {
  selectedEntry: null
});

function getLevelSeverity(level: string): "success" | "info" | "warning" | "danger" {
  switch (level.toLowerCase()) {
    case "error": return "danger";
    case "warning": return "warning";
    case "information": return "info";
    case "debug": return "success";
    default: return "info";
  }
}

function formatTimestamp(timestamp: string): string {
  return new Date(timestamp).toLocaleString();
}
</script>

<template>
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
          <div class="property-value">{{ formatTimestamp(selectedEntry.timestamp) }}</div>
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
        
        <div class="property-group">
          <div class="property-title">
            <i class="pi pi-comment"></i>
            Message
          </div>
          <div class="property-value message-text"><pre>{{ selectedEntry.message }}</pre></div>
        </div>
        
        <div class="property-group" v-if="selectedEntry.eventId">
          <div class="property-title">
            <i class="pi pi-key"></i>
            Event ID
          </div>
          <div class="property-value">{{ selectedEntry.eventId }}</div>
        </div>
        
        <div class="property-group" v-if="selectedEntry.template">
          <div class="property-title">
            <i class="pi pi-file-edit"></i>
            Message Template
          </div>
          <div class="property-value template-text"><pre>{{ selectedEntry.template }}</pre></div>
        </div>
        
        <div class="property-group" v-if="selectedEntry.properties ">
          <div class="property-title">
            <i class="pi pi-cog"></i>
            Properties
          </div>
          <div class="property-value json-text">
            <pre>{{ JSON.stringify(selectedEntry.properties, null, 2) }}</pre>
          </div>
        </div>
        
        <div class="property-group" v-if="selectedEntry.exception">
          <div class="property-title">
            <i class="pi pi-exclamation-triangle"></i>
            Exception
          </div>
          <div class="property-value exception-text"><pre>{{ selectedEntry.exception }}</pre></div>
        </div>
      </div>
      
      <div v-else class="no-selection">
        <i class="pi pi-info-circle"></i>
        <p>Select a log entry to view details</p>
      </div>
    </template>
  </Card>
</template>

<style scoped>
.details-panel-card {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.details-panel-card :deep(.p-card-content),
.details-panel-card :deep(.p-card-body) {
  padding: 0;
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.panel-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
  color: var(--p-text-color);
  padding: 12px 16px;
  border-bottom: 1px solid var(--p-surface-border);
}

.details-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
  overflow-y: auto;
  flex: 1;
  padding: 16px;
  min-height: 0;
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
  background: var(--p-primary-50);
  border-left: 4px solid var(--p-primary-color);
}

.json-text {
  background: var(--p-surface-100);
  border-left: 4px solid var(--p-text-muted-color);
}

.exception-text {
  background: var(--p-red-50);
  border-left: 4px solid var(--p-red-500);
}

.property-value pre {
  margin: 0;
  font-family: inherit;
  font-size: inherit;
  white-space: pre-wrap;
  word-break: break-word;
  overflow-wrap: break-word;
}

.no-selection {
  text-align: center;
  color: var(--p-text-muted-color);
  padding: 40px 16px;
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.no-selection i {
  font-size: 48px;
  margin-bottom: 16px;
  opacity: 0.5;
}
</style>
