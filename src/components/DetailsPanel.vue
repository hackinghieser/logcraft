<script setup lang="ts">
import { ref } from "vue";
import Card from "primevue/card";
import Tag from "primevue/tag";
import Button from "primevue/button";

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
  selectedEntry: null,
});

// State for copy feedback
const copiedField = ref<string | null>(null);

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

function formatTimestamp(timestamp: string): string {
  return new Date(timestamp).toLocaleString();
}

async function copyToClipboard(text: string, fieldName: string) {
  try {
    await navigator.clipboard.writeText(text);
    copiedField.value = fieldName;
    // Clear the feedback after 2 seconds
    setTimeout(() => {
      copiedField.value = null;
    }, 2000);
  } catch (error) {
    console.error("Failed to copy to clipboard:", error);
  }
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
          <div class="property-value-container">
            <div class="property-value">
              {{ formatTimestamp(selectedEntry.timestamp) }}
            </div>
            <Button
              :icon="copiedField === 'timestamp' ? 'pi pi-check' : 'pi pi-copy'"
              :class="{ 'copy-success': copiedField === 'timestamp' }"
              size="small"
              text
              rounded
              class="copy-btn"
              @click="
                copyToClipboard(
                  formatTimestamp(selectedEntry.timestamp),
                  'timestamp',
                )
              "
              v-tooltip.top="
                copiedField === 'timestamp' ? 'Copied!' : 'Copy timestamp'
              "
            />
          </div>
        </div>

        <div class="property-group">
          <div class="property-title">
            <i class="pi pi-tag"></i>
            Level
          </div>
          <div class="property-value-container">
            <div class="property-value">
              <Tag
                :value="selectedEntry.level"
                :severity="getLevelSeverity(selectedEntry.level)"
              />
            </div>
            <Button
              :icon="copiedField === 'level' ? 'pi pi-check' : 'pi pi-copy'"
              :class="{ 'copy-success': copiedField === 'level' }"
              size="small"
              text
              rounded
              class="copy-btn"
              @click="copyToClipboard(selectedEntry.level, 'level')"
              v-tooltip.top="copiedField === 'level' ? 'Copied!' : 'Copy level'"
            />
          </div>
        </div>

        <div class="property-group">
          <div class="property-title">
            <i class="pi pi-comment"></i>
            Message
          </div>
          <div class="property-value-container">
            <div class="property-value message-text">
              <pre>{{ selectedEntry.message }}</pre>
            </div>
            <Button
              :icon="copiedField === 'message' ? 'pi pi-check' : 'pi pi-copy'"
              :class="{ 'copy-success': copiedField === 'message' }"
              size="small"
              text
              rounded
              class="copy-btn"
              @click="copyToClipboard(selectedEntry.message, 'message')"
              v-tooltip.top="
                copiedField === 'message' ? 'Copied!' : 'Copy message'
              "
            />
          </div>
        </div>

        <div class="property-group" v-if="selectedEntry.eventId">
          <div class="property-title">
            <i class="pi pi-key"></i>
            Event ID
          </div>
          <div class="property-value-container">
            <div class="property-value">{{ selectedEntry.eventId }}</div>
            <Button
              :icon="copiedField === 'eventId' ? 'pi pi-check' : 'pi pi-copy'"
              :class="{ 'copy-success': copiedField === 'eventId' }"
              size="small"
              text
              rounded
              class="copy-btn"
              @click="copyToClipboard(selectedEntry.eventId!, 'eventId')"
              v-tooltip.top="
                copiedField === 'eventId' ? 'Copied!' : 'Copy event ID'
              "
            />
          </div>
        </div>

        <div class="property-group" v-if="selectedEntry.template">
          <div class="property-title">
            <i class="pi pi-file-edit"></i>
            Message Template
          </div>
          <div class="property-value-container">
            <div class="property-value template-text">
              <pre>{{ selectedEntry.template }}</pre>
            </div>
            <Button
              :icon="copiedField === 'template' ? 'pi pi-check' : 'pi pi-copy'"
              :class="{ 'copy-success': copiedField === 'template' }"
              size="small"
              text
              rounded
              class="copy-btn"
              @click="copyToClipboard(selectedEntry.template!, 'template')"
              v-tooltip.top="
                copiedField === 'template' ? 'Copied!' : 'Copy template'
              "
            />
          </div>
        </div>

        <div class="property-group" v-if="selectedEntry.properties">
          <div class="property-title">
            <i class="pi pi-cog"></i>
            Properties
          </div>
          <div class="property-value-container">
            <div class="property-value json-text">
              <pre>{{ JSON.stringify(selectedEntry.properties, null, 2) }}</pre>
            </div>
            <Button
              :icon="
                copiedField === 'properties' ? 'pi pi-check' : 'pi pi-copy'
              "
              :class="{ 'copy-success': copiedField === 'properties' }"
              size="small"
              text
              rounded
              class="copy-btn"
              @click="
                copyToClipboard(
                  JSON.stringify(selectedEntry.properties, null, 2),
                  'properties',
                )
              "
              v-tooltip.top="
                copiedField === 'properties' ? 'Copied!' : 'Copy properties'
              "
            />
          </div>
        </div>

        <div class="property-group" v-if="selectedEntry.exception">
          <div class="property-title">
            <i class="pi pi-exclamation-triangle"></i>
            Exception
          </div>
          <div class="property-value-container">
            <div class="property-value exception-text">
              <pre>{{ selectedEntry.exception }}</pre>
            </div>
            <Button
              :icon="copiedField === 'exception' ? 'pi pi-check' : 'pi pi-copy'"
              :class="{ 'copy-success': copiedField === 'exception' }"
              size="small"
              text
              rounded
              class="copy-btn"
              @click="copyToClipboard(selectedEntry.exception!, 'exception')"
              v-tooltip.top="
                copiedField === 'exception' ? 'Copied!' : 'Copy exception'
              "
            />
          </div>
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

.property-value-container {
  position: relative;
  display: flex;
  align-items: flex-start;
  gap: 8px;
}

.property-value-container .property-value {
  flex: 1;
  margin: 0;
}

.copy-btn {
  opacity: 0;
  transition: opacity 0.2s ease;
  align-self: flex-start;
  margin-top: 8px;
}

.property-group:hover .copy-btn {
  opacity: 1;
}

.copy-btn.copy-success {
  opacity: 1;
  color: var(--p-green-500) !important;
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
