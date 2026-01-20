<script setup lang="ts">
import { ref } from 'vue'
import {
  Menu as IconMenu,
  Setting,
  Download,
  VideoPlay,
  Monitor,
  InfoFilled
} from '@element-plus/icons-vue'

import { useRouter } from 'vue-router'

const isCollapse = ref(false)
const activeIndex = ref('1')
const router = useRouter()

const handleSelect = (key: string) => {
  switch (key) {
    case '1': router.push('/'); break;
    case '2': router.push('/proxy'); break;
    case '3': router.push('/download'); break;
    case '4': router.push('/settings'); break;
    case '5': router.push('/logs'); break;
    case '6': router.push('/about'); break;
  }
}
</script>

<template>
  <el-menu
    :default-active="activeIndex"
    class="el-menu-vertical"
    :collapse="isCollapse"
    @select="handleSelect"
  >
    <div class="logo-container" @click="isCollapse = !isCollapse">
      <img src="/vite.svg" alt="Logo" class="logo" />
      <span v-if="!isCollapse" class="title">{{ $t('sidebar.title') }}</span>
    </div>
    
    <el-menu-item index="1">
      <el-icon><VideoPlay /></el-icon>
      <template #title>{{ $t('sidebar.dashboard') }}</template>
    </el-menu-item>
    
    <el-menu-item index="2">
      <el-icon><IconMenu /></el-icon>
      <template #title>{{ $t('sidebar.proxyConfig') }}</template>
    </el-menu-item>
    
    <el-menu-item index="3">
      <el-icon><Download /></el-icon>
      <template #title>{{ $t('sidebar.download') }}</template>
    </el-menu-item>

    <el-menu-item index="4">
      <el-icon><Setting /></el-icon>
      <template #title>{{ $t('sidebar.settings') }}</template>
    </el-menu-item>

    <el-menu-item index="5">
      <el-icon><Monitor /></el-icon>
      <template #title>{{ $t('sidebar.logs') }}</template>
    </el-menu-item>

    <el-menu-item index="6">
      <el-icon><InfoFilled /></el-icon>
      <template #title>{{ $t('sidebar.about') }}</template>
    </el-menu-item>
  </el-menu>
</template>

<style scoped>
.el-menu-vertical {
  height: 100%;
  border-right: none;
}
.el-menu-vertical:not(.el-menu--collapse) {
  width: 200px;
}
.logo-container {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  border-bottom: 1px solid var(--el-border-color);
}
.logo {
  width: 30px;
  height: 30px;
}
.title {
  margin-left: 10px;
  font-weight: bold;
  font-size: 18px;
}
</style>
