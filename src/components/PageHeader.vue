<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useRatholeStore } from '../stores/rathole'
import { useI18n } from 'vue-i18n'
import {
    House,
    Menu as IconMenu,
    Setting,
    Download,
    Monitor,
    InfoFilled
} from '@element-plus/icons-vue'
import { storeToRefs } from 'pinia'
import { computed } from 'vue'

const route = useRoute()
const ratholeStore = useRatholeStore()
const { t } = useI18n()
const { isRunning } = storeToRefs(ratholeStore)

// 页面标题和图标配置
const pageConfig = computed(() => {
  switch (route.name) {
    case 'Dashboard':
      return {
        icon: House,
        title: t('dashboard.title')
      }
    case 'ProxyConfig':
      return {
        icon: IconMenu,
        title: t('proxy.title')
      }
    case 'Settings':
      return {
        icon: Setting,
        title: t('settings.title')
      }
    case 'Logs':
      return {
        icon: Monitor,
        title: t('logs.title')
      }
    case 'About':
      return {
        icon: InfoFilled,
        title: t('about.title')
      }
    case 'Download':
      return {
        icon: Download,
        title: t('download.title')
      }
    default:
      return {
        icon: null,
        title: ''
      }
  }
})
</script>

<template>
  <div class="page-header">
    <div class="header-left">
      <el-icon v-if="pageConfig.icon" class="page-icon">
        <component :is="pageConfig.icon" />
      </el-icon>
      <h1 class="page-title">{{ pageConfig.title }}</h1>
      <div v-if="route.name === 'Dashboard'" class="status-indicator">
        <span class="status-dot" :class="{ active: isRunning }"></span>
        <span class="status-text">{{ isRunning ? '运行中' : '已停止' }}</span>
      </div>
    </div>
    <div class="header-right">
      <slot></slot>
    </div>
  </div>
</template>

<style scoped>
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 24px;
  height: 100%;
  width: 100%;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.page-icon {
  font-size: 22px;
  color: var(--el-color-primary);
}

.page-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 12px;
  background: var(--el-fill-color-light);
  border-radius: 12px;
  font-size: 13px;
  margin-left: 12px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--el-text-color-disabled);
}

.status-dot.active {
  background: var(--el-color-success);
}

.status-text {
  color: var(--el-text-color-secondary);
}

.header-right {
  display: flex;
  align-items: center;
  gap: 12px;
}
</style>