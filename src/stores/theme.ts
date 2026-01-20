import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export const useThemeStore = defineStore('theme', () => {
  // Default to light mode (empty string or 'light'), but check localStorage
  // Previous default was dark (via html class), now we want light default
  const savedTheme = localStorage.getItem('rathole-theme') || 'light'
  const isDark = ref(savedTheme === 'dark')

  function toggleTheme() {
    isDark.value = !isDark.value
  }

  function setTheme(theme: 'light' | 'dark') {
    isDark.value = theme === 'dark'
  }

  watch(isDark, (val) => {
    const html = document.documentElement
    if (val) {
      html.classList.add('dark')
      localStorage.setItem('rathole-theme', 'dark')
    } else {
      html.classList.remove('dark')
      localStorage.setItem('rathole-theme', 'light')
    }
  }, { immediate: true })

  return { isDark, toggleTheme, setTheme }
})
