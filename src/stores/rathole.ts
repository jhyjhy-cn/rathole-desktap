import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export const useRatholeStore = defineStore('rathole', () => {
  const isRunning = ref(false)
  const logs = ref<string[]>([])
  const configPath = ref('') // 保存当前配置路径

  // 初始化时从文件加载日志
  async function loadLogsFromFile() {
    try {
      const fileLogs = await invoke<string[]>('read_logs', { lines: 1000 })
      // 移除文件日志中的时间戳以供显示
      const cleanLogs = fileLogs.map((log: string) => {
        // 移除时间戳前缀，如 [2026-01-25 13:14:15.123]
        return log.replace(/^\[\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\.\d+\]\s*/, '')
      })
      logs.value = cleanLogs
    } catch (e) {
      console.warn('Failed to load logs from file:', e)
    }
  }

  // 监听日志
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

  // 初始化时检查 rathole 是否已在运行
  async function checkStatus() {
    try {
      const running = await invoke<boolean>('is_rathole_running')
      isRunning.value = running
      if (running) {
        logs.value.push('--- Service is running (detected on startup) ---')
      }
      // 无论运行状态如何都从文件加载日志
      await loadLogsFromFile()
    } catch (e) {
      console.warn('Failed to check rathole status:', e)
      // 仍然尝试加载日志
      await loadLogsFromFile()
    }
  }

  // 初始化状态检查并加载日志
  checkStatus()

  async function start(path: string, isServer: boolean) {
    try {
      const result = await invoke('start_rathole', { configPath: path, isServer })
      // 检查是否返回 "Already running"
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
