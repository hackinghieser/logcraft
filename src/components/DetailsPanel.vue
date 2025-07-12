<script setup lang="ts">
import { ref } from "vue";
import Card from "primevue/card";
import Panel from "primevue/panel";
import Tag from "primevue/tag";
import Button from "primevue/button";
import LogEntry from "../types/LogEntry";

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
  <div class="details-panel">
    <div class="panel-header">
      <i class="pi pi-info-circle" />
      Log Entry Details
    </div>

    <div class="panel-content">
      <div v-if="selectedEntry">
        <!-- Basic Information Panel -->
        <Panel header="Basic Information">
          <div style="display: flex; flex-direction: column; gap: 1rem">
            <div
              class="hover-copy-container"
              style="display: flex; align-items: center; gap: 1rem">
              <div
                style="
                  display: flex;
                  align-items: center;
                  gap: 0.5rem;
                  min-width: 120px;
                ">
                <i class="pi pi-clock" />
                <span>Timestamp:</span>
              </div>
              <span style="flex: 1">{{
                formatTimestamp(selectedEntry['@t'])
              }}</span>
              <Button
                v-tooltip.top="
                  copiedField === 'timestamp' ? 'Copied!' : 'Copy timestamp'
                "
                :icon="
                  copiedField === 'timestamp' ? 'pi pi-check' : 'pi pi-copy'
                "
                :severity="
                  copiedField === 'timestamp' ? 'success' : 'secondary'
                "
                size="small"
                text
                class="copy-btn"
                @click="
                  copyToClipboard(
                    formatTimestamp(selectedEntry.timestamp),
                    'timestamp',
                  )
                " />
            </div>

            <div
              class="hover-copy-container"
              style="display: flex; align-items: center; gap: 1rem">
              <div
                style="
                  display: flex;
                  align-items: center;
                  gap: 0.5rem;
                  min-width: 120px;
                ">
                <i class="pi pi-tag" />
                <span>Level:</span>
              </div>
              <div style="flex: 1">
                <Tag
                  :value="selectedEntry['@l']"
                  :severity="getLevelSeverity(selectedEntry['@l'])" />
              </div>
              <Button
                v-tooltip.top="
                  copiedField === 'level' ? 'Copied!' : 'Copy level'
                "
                :icon="copiedField === 'level' ? 'pi pi-check' : 'pi pi-copy'"
                :severity="copiedField === 'level' ? 'success' : 'secondary'"
                size="small"
                text
                class="copy-btn"
                @click="copyToClipboard(selectedEntry['@l'], 'level')" />
            </div>
          </div>
        </Panel>

        <!-- Message Panel -->
        <Panel header="Message" style="margin-top: 1rem">
          <div
            class="hover-copy-container"
            style="display: flex; align-items: flex-start; gap: 1rem">
            <pre style="margin: 0; white-space: pre-wrap; flex: 1">{{
              selectedEntry['@m']
            }}</pre>
            <Button
              v-tooltip.top="
                copiedField === 'message' ? 'Copied!' : 'Copy message'
              "
              :icon="copiedField === 'message' ? 'pi pi-check' : 'pi pi-copy'"
              :severity="copiedField === 'message' ? 'success' : 'secondary'"
              size="small"
              text
              class="copy-btn"
              @click="copyToClipboard(selectedEntry['@m'], 'message')" />
          </div>
        </Panel>

        <!-- Event ID Panel -->
        <Panel
          v-if="selectedEntry['@i']"
          header="Event ID"
          style="margin-top: 1rem">
          <div
            class="hover-copy-container"
            style="display: flex; align-items: center; gap: 1rem">
            <span style="flex: 1">{{ selectedEntry['@i'] }}</span>
            <Button
              v-tooltip.top="
                copiedField === 'eventId' ? 'Copied!' : 'Copy event ID'
              "
              :icon="copiedField === 'eventId' ? 'pi pi-check' : 'pi pi-copy'"
              :severity="copiedField === 'eventId' ? 'success' : 'secondary'"
              size="small"
              text
              class="copy-btn"
              @click="copyToClipboard(selectedEntry['@i']!, 'eventId')" />
          </div>
        </Panel>

        <!-- Message Template Panel -->
        <Panel
          v-if="selectedEntry['@mt']"
          header="Message Template"
          style="margin-top: 1rem">
          <div
            class="hover-copy-container"
            style="display: flex; align-items: flex-start; gap: 1rem">
            <pre style="margin: 0; white-space: pre-wrap; flex: 1">{{
              selectedEntry['@mt']
            }}</pre>
            <Button
              v-tooltip.top="
                copiedField === 'template' ? 'Copied!' : 'Copy template'
              "
              :icon="copiedField === 'template' ? 'pi pi-check' : 'pi pi-copy'"
              :severity="copiedField === 'template' ? 'success' : 'secondary'"
              size="small"
              text
              class="copy-btn"
              @click="copyToClipboard(selectedEntry['@mt']!, 'template')" />
          </div>
        </Panel>

        <!-- Renderings Panel -->
        <Panel
          v-if="selectedEntry['@r'] && selectedEntry['@r'].length > 0"
          header="Renderings"
          style="margin-top: 1rem">
          <div
            class="hover-copy-container"
            style="display: flex; align-items: flex-start; gap: 1rem">
            <pre style="margin: 0; white-space: pre-wrap; flex: 1">{{
              JSON.stringify(selectedEntry['@r'], null, 2)
            }}</pre>
            <Button
              v-tooltip.top="
                copiedField === 'renderings' ? 'Copied!' : 'Copy renderings'
              "
              :icon="
                copiedField === 'renderings' ? 'pi pi-check' : 'pi pi-copy'
              "
              :severity="copiedField === 'renderings' ? 'success' : 'secondary'"
              size="small"
              text
              class="copy-btn"
              @click="
                copyToClipboard(
                  JSON.stringify(selectedEntry['@r'], null, 2),
                  'renderings',
                )
              " />
          </div>
        </Panel>

        <!-- Exception Panel -->
        <Panel
          v-if="selectedEntry['@x']"
          header="Exception"
          style="margin-top: 1rem">
          <div
            class="hover-copy-container"
            style="display: flex; align-items: flex-start; gap: 1rem">
            <pre style="margin: 0; white-space: pre-wrap; flex: 1">{{
              selectedEntry['@x']
            }}</pre>
            <Button
              v-tooltip.top="
                copiedField === 'exception' ? 'Copied!' : 'Copy exception'
              "
              :icon="copiedField === 'exception' ? 'pi pi-check' : 'pi pi-copy'"
              :severity="copiedField === 'exception' ? 'success' : 'secondary'"
              size="small"
              text
              class="copy-btn"
              @click="copyToClipboard(selectedEntry['@x']!, 'exception')" />
          </div>
        </Panel>
      </div>

      <div v-else style="text-align: center; padding: 2rem">
        <i
          class="pi pi-info-circle"
          style="font-size: 3rem; opacity: 0.5; margin-bottom: 1rem" />
        <p>Select a log entry to view details</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.hover-copy-container {
  position: relative;
}

.hover-copy-container .copy-btn {
  opacity: 0;
  transition: opacity 0.2s ease;
}

.hover-copy-container:hover .copy-btn {
  opacity: 1;
}

.copy-btn.copy-success {
  opacity: 1 !important;
}

.details-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 1rem;
  font-weight: 600;
  font-size: 1.125rem;
  border-bottom: 1px solid var(--p-surface-border);
}

.panel-content {
  flex: 1;
  padding: 1rem;
  overflow-y: auto;
}
</style>
