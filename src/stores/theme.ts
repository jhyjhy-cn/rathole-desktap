import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export const useThemeStore = defineStore('theme', () => {
  // 默认为亮色模式（空字符串或 'light'），但检查 localStorage
  // 之前的默认是暗色（通过 html 类），现在我们想要亮色默认
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
