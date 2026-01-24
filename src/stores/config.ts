import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { parse, stringify } from 'smol-toml'

const CONFIG_STORAGE_KEY = 'rathole-client-config'

export const useConfigStore = defineStore('config', () => {
  const currentConfig = ref<any>({})
  const rawConfig = ref('')

  // Load from localStorage on init
  function loadFromStorage() {
    const saved = localStorage.getItem(CONFIG_STORAGE_KEY)
    if (saved) {
      try {
        currentConfig.value = JSON.parse(saved)
        rawConfig.value = stringify(currentConfig.value)
      } catch (e) {
        console.warn('Failed to load config from storage', e)
      }
    }
  }

  // Save to localStorage whenever config changes
  watch(currentConfig, (val) => {
    if (val && Object.keys(val).length > 0) {
      localStorage.setItem(CONFIG_STORAGE_KEY, JSON.stringify(val))
      rawConfig.value = stringify(val)
    }
  }, { deep: true })

  async function loadConfig(path: string) {
    try {
      const content = await invoke<string>('read_config', { path })
      rawConfig.value = content
      try {
          currentConfig.value = parse(content)
          // Also save to localStorage
          localStorage.setItem(CONFIG_STORAGE_KEY, JSON.stringify(currentConfig.value))
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
              currentConfig.value = parse(content)
              localStorage.setItem(CONFIG_STORAGE_KEY, JSON.stringify(currentConfig.value))
          } catch (e) {}
      }
    } catch (error) {
      console.error(error)
      throw error
    }
  }

  function updateFromObject(obj: any) {
      currentConfig.value = obj
      rawConfig.value = stringify(obj)
  }

  // Initialize from storage
  loadFromStorage()

  return { currentConfig, rawConfig, loadConfig, saveConfig, updateFromObject, loadFromStorage }
})
