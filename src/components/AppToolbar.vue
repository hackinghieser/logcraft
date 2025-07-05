<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import Toolbar from "primevue/toolbar";
import Button from "primevue/button";

interface LogFileInfo {
  path: string;
  totalCount: number;
  logLevels: string[];
  dateRange?: [string, string];
}

interface Props {
  logFile?: LogFileInfo | null;
}

defineProps<Props>();

const emit = defineEmits<{
  openFile: [];
  toggleTheme: [];
  openSettings: [];
}>();

async function openFile() {
  emit('openFile');
}

function toggleTheme() {
  emit('toggleTheme');
}

function openSettings() {
  emit('openSettings');
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
        <Button icon="pi pi-moon" label="Theme" text @click="toggleTheme" />
        <Button icon="pi pi-cog" label="Settings" text @click="openSettings" />
        <Button icon="pi pi-code" label="Test Cleverlib" @click="testCleverlib" outlined />
      </div>
    </template>
  </Toolbar>
</template>

<style scoped>
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
</style>