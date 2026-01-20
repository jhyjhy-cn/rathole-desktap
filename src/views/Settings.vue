<script setup lang="ts">
import { useConfigStore } from '../stores/config'
import { useThemeStore } from '../stores/theme'
import { storeToRefs } from 'pinia'
import { ref } from 'vue'
import { open, save } from '@tauri-apps/plugin-dialog'
import { ElMessage } from 'element-plus'
import { useI18n } from 'vue-i18n'
import { Moon, Sunny } from '@element-plus/icons-vue'

const store = useConfigStore()
const themeStore = useThemeStore()
const { rawConfig } = storeToRefs(store)
const { isDark } = storeToRefs(themeStore)
const currentFile = ref('')
const { locale } = useI18n()

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
    ElMessage.success('Saved!')
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

    <el-card class="general-settings">
        <template #header>
            <div class="card-header">
                <span>{{ $t('settings.general') }}</span>
            </div>
        </template>
        <el-form label-width="120px">
            <el-form-item :label="$t('settings.language')">
                <el-select v-model="locale" @change="handleLocaleChange">
                    <el-option label="中文 (简体)" value="zh-CN" />
                    <el-option label="English" value="en-US" />
                </el-select>
            </el-form-item>
            <el-form-item :label="$t('settings.theme')">
                <el-switch
                    v-model="isDark"
                    :active-action-icon="Moon"
                    :inactive-action-icon="Sunny"
                    :active-text="$t('settings.dark')"
                    :inactive-text="$t('settings.light')"
                    @change="themeStore.setTheme(isDark ? 'dark' : 'light')"
                />
            </el-form-item>
        </el-form>
    </el-card>

    <el-input
        v-model="rawConfig"
        type="textarea"
        :rows="15"
        :placeholder="$t('settings.editorPlaceholder')"
        class="editor"
    />
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
.editor {
    flex: 1;
    font-family: monospace;
}
.general-settings {
    margin-bottom: 10px;
}
</style>

