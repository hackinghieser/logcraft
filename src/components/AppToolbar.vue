<script setup lang="ts">

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
  openSettings: [];
}>();

async function openFile() {
  emit('openFile');
}



function openSettings() {
  emit('openSettings');
}


</script>

<template>
  <Toolbar class="app-toolbar">
    <template #start>
      <div class="toolbar-left">
        <h1 class="app-title">LogCraft</h1>
        <div class="file-info" v-if="logFile">
          <i class="pi pi-file"></i>
          {{ logFile.path }} • {{ logFile.totalCount.toLocaleString() }} entries
        </div>
      </div>
    </template>
    
    <template #end>
      <div class="toolbar-right">
        <Button icon="pi pi-folder-open" label="Open File" @click="openFile" size="small" />
        
        <Button icon="pi pi-cog" label="Settings" text @click="openSettings" size="small" />
        
      </div>
    </template>
  </Toolbar>
</template>

<style scoped>
.app-toolbar {
  background: transparent;
  border: none;
  padding: 8px 16px;
  min-height: 48px;
}

.toolbar-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.app-title {
  font-size: 20px;
  font-weight: 700;
  margin: 0;
  color: var(--p-text-color);
}

.file-info {
  font-size: 13px;
  color: var(--p-text-muted-color);
  display: flex;
  align-items: center;
  gap: 6px;
}

.toolbar-right {
  display: flex;
  align-items: center;
  gap: 8px;
}
</style>