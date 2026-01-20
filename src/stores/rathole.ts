import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export const useRatholeStore = defineStore('rathole', () => {
  const isRunning = ref(false)
  const logs = ref<string[]>([])
  const configPath = ref('') // Store current config path

  // Listen for logs
  // Need to await this or handle it better in setup, but for now we just fire and forget
  try {
      listen<string>('rathole-log', (event) => {
        logs.value.push(event.payload)
        // Keep log size manageable
        if (logs.value.length > 1000) {
            logs.value.shift()
        }
      }).catch(err => console.warn("Failed to setup listener (async)", err));
  } catch (e) {
      console.warn("Failed to setup listener (sync) - likely not in Tauri", e);
  }

  async function start(path: string, isServer: boolean) {
    try {
      await invoke('start_rathole', { configPath: path, isServer })
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

  return { isRunning, logs, start, stop, configPath }
})
