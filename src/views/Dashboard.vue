<script setup lang="ts">
import { useRatholeStore } from '../stores/rathole'
import { storeToRefs } from 'pinia'
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'

const store = useRatholeStore()
const { isRunning, logs } = storeToRefs(store)
const selectedConfig = ref('')

async function selectConfig() {
  const file = await open({
    multiple: false,
    filters: [{
      name: 'Config',
      extensions: ['toml']
    }]
  })
  if (file) {
      selectedConfig.value = file as string
  }
}

async function toggleService() {
    if (isRunning.value) {
        await store.stop()
    } else {
        if (!selectedConfig.value) {
            // Attempt to pick
            await selectConfig()
            if (!selectedConfig.value) return
        }
        // TODO: Detect server/client mode? Or ask user?
        // For now, defaulting to client or adding a toggle
        await store.start(selectedConfig.value, false) // Default to client
    }
}
</script>

<template>
  <div class="dashboard">
    <el-card class="control-card">
      <template #header>
        <div class="card-header">
          <span>{{ $t('dashboard.serviceControl') }}</span>
          <el-tag :type="isRunning ? 'success' : 'danger'">
            {{ isRunning ? $t('dashboard.running') : $t('dashboard.stopped') }}
          </el-tag>
        </div>
      </template>
      
      <div class="controls">
        <el-input 
            v-model="selectedConfig" 
            :placeholder="$t('dashboard.selectConfig')" 
            readonly 
            class="config-input"
            @click="selectConfig"
        >
            <template #append>
                <el-button @click="selectConfig">{{ $t('common.browse') }}</el-button>
            </template>
        </el-input>
        
        <el-button 
            type="primary" 
            :type="isRunning ? 'danger' : 'success'" 
            size="large" 
            circle 
            class="start-btn"
            @click="toggleService"
        >
            <el-icon size="24">
                <component :is="isRunning ? 'SwitchButton' : 'VideoPlay'" />
            </el-icon>
        </el-button>
        <div class="status-text">{{ isRunning ? $t('dashboard.serviceActive') : $t('dashboard.serviceStopped') }}</div>
      </div>
    </el-card>

    <el-card class="logs-card">
      <template #header>
        <div class="card-header">
          <span>{{ $t('dashboard.recentLogs') }}</span>
          <router-link to="/logs">
             <el-button link type="primary">{{ $t('dashboard.viewAll') }}</el-button>
          </router-link>
        </div>
      </template>
      <div class="log-preview">
        <div v-for="(log, index) in logs.slice(-5)" :key="index" class="log-line">
            {{ log }}
        </div>
        <div v-if="logs.length === 0" class="no-logs">{{ $t('dashboard.noLogs') }}</div>
      </div>
    </el-card>
  </div>
</template>

<style scoped>
.dashboard {
  display: flex;
  flex-direction: column;
  gap: 20px;
}
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.controls {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
  padding: 20px;
}
.config-input {
    width: 100%;
}
.start-btn {
    width: 80px;
    height: 80px;
    font-size: 24px;
}
.log-preview {
    background: #1e1e1e;
    padding: 10px;
    border-radius: 4px;
    font-family: monospace;
    font-size: 12px;
    min-height: 100px;
}
.log-line {
    white-space: pre-wrap;
    word-break: break-all;
    color: #dcdcdc;
    margin-bottom: 4px;
}
.no-logs {
    color: #666;
    text-align: center;
    padding: 20px;
}
</style>
