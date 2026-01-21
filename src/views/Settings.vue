<script setup lang="ts">
import { useConfigStore } from '../stores/config'
import { useThemeStore } from '../stores/theme'
import { storeToRefs } from 'pinia'
import { ref, computed } from 'vue'
import { open, save } from '@tauri-apps/plugin-dialog'
import { ElMessage } from 'element-plus'
import { useI18n } from 'vue-i18n'
import { Moon, Sunny } from '@element-plus/icons-vue'

const store = useConfigStore()
const themeStore = useThemeStore()
const { currentConfig } = storeToRefs(store)
const { isDark } = storeToRefs(themeStore)
const currentFile = ref('')
const { locale } = useI18n()

// Client settings
const clientSettings = computed({
  get: () => ({
    serverAddr: currentConfig.value?.client?.server_addr || '',
    token: currentConfig.value?.client?.token || ''
  }),
  set: (val) => {
    const cfg = JSON.parse(JSON.stringify(currentConfig.value || {}))
    if (!cfg.client) cfg.client = {}
    cfg.client.server_addr = val.serverAddr
    cfg.client.token = val.token
    store.updateFromObject(cfg)
  }
})

async function loadFile() {
    const file = await open({
        filters: [{ name: 'Config', extensions: ['toml'] }]
    })
    if (file) {
        currentFile.value = file as string
        await store.loadConfig(file as string)
    }
}

async function saveFile() {
    if (!currentFile.value) {
        const file = await save({
            filters: [{ name: 'Config', extensions: ['toml'] }]
        })
        if (file) currentFile.value = file as string
        else return
    }
    await store.saveConfig(currentFile.value)
    ElMessage.success('保存成功！')
}

function handleLocaleChange(val: string) {
    locale.value = val
    localStorage.setItem('rathole-locale', val)
}
</script>

<template>
  <div class="settings-page">
    <div class="header">
        <h2>{{ $t('settings.title') }}</h2>
        <div class="actions">
            <el-button @click="loadFile">{{ $t('common.load') }}</el-button>
            <el-button type="primary" @click="saveFile">{{ $t('common.save') }}</el-button>
        </div>
    </div>

    <el-card class="settings-card">
        <template #header>
            <div class="card-header">
                <span>{{ $t('settings.general') }}</span>
            </div>
        </template>

        <div class="settings-content">
            <!-- Language & Theme -->
            <div class="section">
                <h4 class="section-title">{{ $t('settings.general') }}</h4>
                <div class="form-row">
                    <label>{{ $t('settings.language') }}</label>
                    <el-select v-model="locale" @change="handleLocaleChange" style="width: 200px;">
                        <el-option label="中文 (简体)" value="zh-CN" />
                        <el-option label="English" value="en-US" />
                    </el-select>
                </div>
                <div class="form-row">
                    <label>{{ $t('settings.theme') }}</label>
                    <el-switch
                        v-model="isDark"
                        :active-action-icon="Moon"
                        :inactive-action-icon="Sunny"
                        :active-text="$t('settings.dark')"
                        :inactive-text="$t('settings.light')"
                        @change="themeStore.setTheme(isDark ? 'dark' : 'light')"
                    />
                </div>
            </div>

            <el-divider />

            <!-- Client Config -->
            <div class="section">
                <h4 class="section-title">客户端配置</h4>
                <div class="form-row">
                    <label>服务端地址</label>
                    <el-input v-model="clientSettings.serverAddr" placeholder="server.example.com:2333" style="width: 300px;" />
                </div>
                <div class="form-row">
                    <label>令牌 (Token)</label>
                    <el-input v-model="clientSettings.token" type="password" show-password placeholder="your-token" style="width: 300px;" />
                </div>
            </div>

            <el-divider />

            <!-- Services Summary -->
            <div class="section">
                <h4 class="section-title">服务列表</h4>
                <p class="hint">请在"代理配置"页面管理服务</p>
                <div v-if="currentConfig?.client?.services" class="services-summary">
                    <el-tag v-for="(svc, name) in currentConfig.client.services" :key="name" style="margin: 4px;">
                        {{ name }}: {{ (svc as any).local_addr }}
                    </el-tag>
                </div>
                <el-empty v-else description="暂无服务" :image-size="60" />
            </div>
        </div>
    </el-card>
  </div>
</template>

<style scoped>
.settings-page {
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 20px;
}
.header {
    display: flex;
    justify-content: space-between;
    align-items: center;
}
.settings-card {
    flex: 1;
    overflow: auto;
}
.card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
}
.settings-content {
    padding: 10px 0;
}
.section {
    margin-bottom: 24px;
}
.section:last-child {
    margin-bottom: 0;
}
.section-title {
    margin: 0 0 16px 0;
    font-size: 16px;
    font-weight: 500;
    color: var(--el-text-color-primary);
}
.form-row {
    display: flex;
    align-items: center;
    margin-bottom: 16px;
    gap: 16px;
}
.form-row label {
    width: 120px;
    text-align: right;
    color: var(--el-text-color-regular);
}
.hint {
    color: var(--el-text-color-secondary);
    font-size: 13px;
    margin: 0 0 12px 0;
}
.services-summary {
    display: flex;
    flex-wrap: wrap;
}
</style>

