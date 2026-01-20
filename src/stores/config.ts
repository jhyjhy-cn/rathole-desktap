import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { parse, stringify } from 'smol-toml'

export const useConfigStore = defineStore('config', () => {
  const currentConfig = ref<any>({})
  const rawConfig = ref('')
  
  async function loadConfig(path: string) {
    try {
      const content = await invoke<string>('read_config', { path })
      rawConfig.value = content
      try {
          currentConfig.value = parse(content)
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

  return { currentConfig, rawConfig, loadConfig, saveConfig, updateFromObject }
})
