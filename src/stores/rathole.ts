import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export const useRatholeStore = defineStore('rathole', () => {
  const isRunning = ref(false)
  const logs = ref<string[]>([])
  const configPath = ref('') // Store current config path

  // Load logs from file on init
  async function loadLogsFromFile() {
    try {
      const fileLogs = await invoke<string[]>('read_logs', { lines: 1000 })
      // Remove timestamps from file logs for display
      const cleanLogs = fileLogs.map((log: string) => {
        // Remove timestamp prefix like [2026-01-25 13:14:15.123]
        return log.replace(/^\[\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\.\d+\]\s*/, '')
      })
      logs.value = cleanLogs
    } catch (e) {
      console.warn('Failed to load logs from file:', e)
    }
  }

  // Listen for logs
  try {
      listen<string>('rathole-log', (event) => {
        logs.value.push(event.payload)
        if (logs.value.length > 1000) {
          logs.value.shift()
        }
      }).catch(err => console.warn("Failed to setup listener", err));
  } catch (e) {
      console.warn("Failed to setup listener", e);
  }

  // Check if rathole is already running on init
  async function checkStatus() {
    try {
      const running = await invoke<boolean>('is_rathole_running')
      isRunning.value = running
      if (running) {
        logs.value.push('--- Service is running (detected on startup) ---')
      }
      // Load logs from file regardless of running status
      await loadLogsFromFile()
    } catch (e) {
      console.warn('Failed to check rathole status:', e)
      // Still try to load logs
      await loadLogsFromFile()
    }
  }

  // Initialize status check and load logs
  checkStatus()

  async function start(path: string, isServer: boolean) {
    try {
      const result = await invoke('start_rathole', { configPath: path, isServer })
      // Check if "Already running" was returned
      if (result === 'Already running') {
        isRunning.value = true
        logs.value.push('--- Service is already running ---')
        return
      }
      isRunning.value = true
      configPath.value = path
      logs.value.push('--- Service Started ---')
    } catch (error) {
      console.error(error)
      logs.value.push(`Error starting service: ${error}`)
      throw error
    }
  }

  async function stop() {
    try {
      await invoke('stop_rathole')
      isRunning.value = false
      logs.value.push('--- Service Stopped ---')
    } catch (error) {
      console.error(error)
      logs.value.push(`Error stopping service: ${error}`)
      throw error
    }
  }

  return { isRunning, logs, start, stop, configPath, checkStatus, loadLogsFromFile }
})
