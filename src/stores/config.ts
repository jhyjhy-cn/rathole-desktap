import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { parse, stringify } from 'smol-toml'

const CONFIG_STORAGE_KEY = 'rathole-client-config'

// Clean deprecated fields from config
function cleanConfig(config: any): any {
  if (!config || !config.client) return config

  const cleaned = JSON.parse(JSON.stringify(config))

  // Remove deprecated fields from client level
  if (cleaned.client.server_addr) {
    delete cleaned.client.server_addr
  }
  if (cleaned.client.token) {
    delete cleaned.client.token
  }

  return cleaned
}

export const useConfigStore = defineStore('config', () => {
  const currentConfig = ref<any>({})
  const rawConfig = ref('')

  // Load from localStorage on init
  function loadFromStorage() {
    const saved = localStorage.getItem(CONFIG_STORAGE_KEY)
    if (saved) {
      try {
        let config = JSON.parse(saved)
        config = cleanConfig(config)
        currentConfig.value = config
        rawConfig.value = stringify(config)
        // Save cleaned config back to localStorage
        localStorage.setItem(CONFIG_STORAGE_KEY, JSON.stringify(config))
      } catch (e) {
        console.warn('Failed to load config from storage', e)
      }
    }
  }

  // Save to localStorage whenever config changes
  watch(currentConfig, (val) => {
    if (val && Object.keys(val).length > 0) {
      const cleaned = cleanConfig(val)
      localStorage.setItem(CONFIG_STORAGE_KEY, JSON.stringify(cleaned))
      rawConfig.value = stringify(cleaned)
    }
  }, { deep: true })

  async function loadConfig(path: string) {
    try {
      const content = await invoke<string>('read_config', { path })
      rawConfig.value = content
      try {
          let config = parse(content)
          config = cleanConfig(config)
          currentConfig.value = config
          // Also save to localStorage
          localStorage.setItem(CONFIG_STORAGE_KEY, JSON.stringify(config))
      } catch (e) {
          console.warn("Failed to parse TOML", e)
      }
    } catch (error) {
      console.error(error)
      throw error
    }
  }

  async function saveConfig(path: string, content?: string) {
    try {
      const toSave = content || rawConfig.value
      await invoke('write_config', { path, content: toSave })
      // Update local state
      if (content) {
          rawConfig.value = content
          try {
              let config = parse(content)
              config = cleanConfig(config)
              currentConfig.value = config
              localStorage.setItem(CONFIG_STORAGE_KEY, JSON.stringify(config))
          } catch (e) {}
      }
    } catch (error) {
      console.error(error)
      throw error
    }
  }

  function updateFromObject(obj: any) {
      const cleaned = cleanConfig(obj)
      currentConfig.value = cleaned
      rawConfig.value = stringify(cleaned)
  }

  // Initialize from storage
  loadFromStorage()

  return { currentConfig, rawConfig, loadConfig, saveConfig, updateFromObject, loadFromStorage }
})
