import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface SystemStats {
  cpuUsage: number
  memoryUsage: number
}

export const useSystemStatsStore = defineStore('systemStats', () => {
  const cpuUsage = ref(0)
  const memoryUsage = ref(0)
  const isLoading = ref(false)

  // Fetch system stats
  async function fetchStats() {
    isLoading.value = true
    try {
      const [cpu, mem] = await invoke<[number, number]>('get_system_stats')
      cpuUsage.value = cpu
      memoryUsage.value = mem
    } catch (e) {
      console.error('Failed to fetch system stats:', e)
    } finally {
      isLoading.value = false
    }
  }

  // Start polling (every 2 seconds)
  let pollingInterval: number | null = null

  function startPolling(interval = 2000) {
    fetchStats()
    pollingInterval = window.setInterval(fetchStats, interval)
  }

  function stopPolling() {
    if (pollingInterval) {
      clearInterval(pollingInterval)
      pollingInterval = null
    }
  }

  return {
    cpuUsage,
    memoryUsage,
    isLoading,
    fetchStats,
    startPolling,
    stopPolling
  }
})
