import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export const useRatholeStore = defineStore('rathole', () => {
  const isRunning = ref(false)
  const logs = ref<string[]>([])
  const configPath = ref('') // Store current config path

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
    } catch (e) {
      console.warn('Failed to check rathole status:', e)
    }
  }

  // Initialize status check
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

  return { isRunning, logs, start, stop, configPath, checkStatus }
})
