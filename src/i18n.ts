import { createI18n } from 'vue-i18n'
import zhCN from './locales/zh-CN'
import enUS from './locales/en-US'

// 从本地存储获取保存的语言或默认为 zh-CN
const savedLocale = localStorage.getItem('rathole-locale') || 'zh-CN'

const i18n = createI18n({
  legacy: false, // 使用 Composition API
  locale: savedLocale,
  fallbackLocale: 'en-US',
  messages: {
    'zh-CN': zhCN,
    'en-US': enUS
  }
})

export default i18n
