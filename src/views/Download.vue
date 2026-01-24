<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'

const loading = ref(false)
const version = ref('v0.5.0') // Default or fetch latest
const downloadPath = ref('')

async function startDownload() {
    loading.value = true
    try {
        const path = await invoke<string>('download_rathole', { version: version.value })
        downloadPath.value = path
        ElMessage.success(`Downloaded successfully to ${path}`)
    } catch (e) {
        ElMessage.error(`Download failed: ${e}`)
    } finally {
        loading.value = false
    }
}
</script>

<template>
  <div class="download-page">
    <h2>{{ $t('download.title') }}</h2>
    <el-card>
        <p>{{ $t('download.description') }}</p>
        <el-form label-width="120px">
            <el-form-item :label="$t('common.version')">
                <el-input v-model="version" placeholder="v0.5.0" />
            </el-form-item>
            <el-form-item>
                <el-button type="primary" :loading="loading" @click="startDownload">
                    {{ $t('download.downloadInstall') }}
                </el-button>
            </el-form-item>
        </el-form>
        
        <div v-if="downloadPath" class="result">
            <el-alert :title="$t('download.ready')" type="success" :description="`${$t('download.location')}: ${downloadPath}`" show-icon />
        </div>
    </el-card>
  </div>
</template>

<style scoped>
.download-page {
    max-width: 600px;
    margin: 0 auto;
    padding-top: 0;
}
.result {
    margin-top: 20px;
}
</style>
