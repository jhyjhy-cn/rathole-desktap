<script setup lang="ts">
import { useRatholeStore } from '../stores/rathole'
import { storeToRefs } from 'pinia'
import { ref, watch, nextTick } from 'vue'

const store = useRatholeStore()
const { logs } = storeToRefs(store)
const logContainer = ref<HTMLElement | null>(null)

watch(logs, () => {
    nextTick(() => {
        if (logContainer.value) {
            logContainer.value.scrollTop = logContainer.value.scrollHeight
        }
    })
}, { deep: true })
</script>

<template>
  <div class="logs-page">
    <div class="header">
        <h2>{{ $t('logs.title') }}</h2>
        <el-button @click="logs = []">{{ $t('logs.clear') }}</el-button>
    </div>
    <div class="log-container" ref="logContainer">
        <div v-for="(log, index) in logs" :key="index" class="log-line">
            {{ log }}
        </div>
    </div>
  </div>
</template>

<style scoped>
.logs-page {
    height: 100%;
    display: flex;
    flex-direction: column;
}
.header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
}
.log-container {
    flex: 1;
    background: #1e1e1e;
    padding: 10px;
    border-radius: 4px;
    font-family: monospace;
    overflow-y: auto;
    white-space: pre-wrap;
    word-break: break-all;
}
.log-line {
    color: #dcdcdc;
    margin-bottom: 2px;
}
</style>
