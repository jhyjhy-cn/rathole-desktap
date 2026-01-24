<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from "element-plus";
import { useI18n } from "vue-i18n";
import PageHeader from "../components/PageHeader.vue";

interface Release {
    name: string;
    size: number;
    downloads: number;
    published_at: string;
    download_url: string;
}

const { t } = useI18n();
const loading = ref(false);
const releases = ref<Release[]>([]);
const downloading = ref<string>("");

onMounted(async () => {
    await fetchReleases();
});

async function fetchReleases() {
    loading.value = true;
    try {
        const response = await fetch(
            "https://api.github.com/repos/rapiz1/rathole/releases",
        );
        const data = await response.json();

        releases.value = data
            .flatMap((release: any) => {
                if (release.prerelease) return [];
                const asset = release.assets.find(
                    (a: any) =>
                        a.name.includes("aarch64-apple-darwin") ||
                        a.name.includes("x86_64-apple-darwin"),
                );
                if (!asset) return [];

                return [
                    {
                        name: release.tag_name,
                        size: asset.size,
                        downloads: asset.download_count,
                        published_at: release.published_at,
                        download_url: asset.browser_download_url,
                    },
                ];
            })
            .slice(0, 10);
    } catch (e) {
        ElMessage.error(`Failed to fetch releases: ${e}`);
    } finally {
        loading.value = false;
    }
}

async function startDownload(release: Release) {
    downloading.value = release.name;
    try {
        const path = await invoke<string>("download_rathole", {
            version: release.name,
        });
        ElMessage.success(t("download.success", { path }));
    } catch (e) {
        ElMessage.error(t("download.failed", { error: e }));
    } finally {
        downloading.value = "";
    }
}

function formatSize(bytes: number) {
    if (bytes < 1024) return bytes + " B";
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + " MB";
    return (bytes / 1024 / 1024).toFixed(2) + " MB";
}

function formatDate(date: string) {
    return new Date(date).toLocaleDateString("zh-CN");
}

function copyLink(url: string) {
    navigator.clipboard.writeText(url);
    ElMessage.success(t("common.copy"));
}
</script>

<template>
<div class="download-page">
  <PageHeader>
    <template #header>
      <el-button type="primary" @click="fetchReleases">
        <el-icon><Refresh /></el-icon>
        刷新
      </el-button>
      <el-button type="primary">
        <el-icon><Upload /></el-icon>
        导入
      </el-button>
    </template>
  </PageHeader>

  <div class="download-content">
    <el-row :gutter="20" v-loading="loading">
        <el-col
            :span="12"
            v-for="release in releases"
            :key="release.name"
            class="release-col"
        >
            <div class="release-card">
                <div class="card-header">
                    <span class="version-name">{{ release.name }}</span>
                </div>
                
                <div class="card-body">
                    <div class="info-row">
                        <span class="info-label">{{ t("download.size") }}:</span>
                        <span class="info-value size-badge">{{ formatSize(release.size) }}</span>
                    </div>
                    
                    <div class="info-row">
                        <span class="info-label">{{ t("download.downloads") }}:</span>
                        <span class="info-value">{{ release.downloads.toLocaleString() }}</span>
                    </div>
                    
                    <div class="info-row">
                        <span class="info-label">{{ t("download.releaseDate") }}:</span>
                        <span class="info-value">{{ formatDate(release.published_at) }}</span>
                    </div>
                </div>
                
                <div class="card-actions">
                    <el-button
                        type="primary"
                        :loading="downloading === release.name"
                        @click="startDownload(release)"
                        class="download-btn"
                    >
                        <el-icon><Download /></el-icon>
                        {{ t("download.downloadBtn") }}
                    </el-button>
                    
                    <el-button
                        link
                        @click="copyLink(release.download_url)"
                        class="copy-btn"
                    >
                        <el-icon><Link /></el-icon>
                        {{ t("download.copyLink") }}
                    </el-button>
                </div>
            </div>
        </el-col>
    </el-row>

    <el-empty
        v-if="!loading && releases.length === 0"
        :description="$t('common.loading')"
    />
  </div>
</div>
</template>

<style scoped>
.download-page {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
}

.download-content {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
}

.release-col {
    margin-bottom: 16px;
}

.release-card {
    border: 1px solid var(--el-border-color);
    border-radius: 12px;
    padding: 16px;
    background: white;
    transition: all 0.3s ease;
    min-height: 140px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
}

.release-card:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    border-color: var(--el-color-primary-light-5);
    transform: translateY(-2px);
}

.card-header {
    display: flex;
    align-items: center;
    margin-bottom: 12px;
}

.version-name {
    font-size: 16px;
    font-weight: 600;
    color: var(--el-text-color-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.card-body {
    flex: 1;
    display: flex;
    flex-wrap: wrap;
    gap: 8px 16px;
    margin-bottom: 12px;
    overflow: hidden;
}

.info-row {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    min-width: 0;
}

.info-label {
    color: var(--el-text-color-secondary);
    font-weight: 500;
    flex-shrink: 0;
}

.info-value {
    color: var(--el-text-color-primary);
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.size-badge {
    padding: 3px 10px;
    background: linear-gradient(135deg, var(--el-color-primary-light-9), var(--el-color-primary-light-8));
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    color: var(--el-color-primary);
    white-space: nowrap;
}

.card-actions {
    display: flex;
    gap: 8px;
    padding-top: 10px;
    border-top: 1px solid var(--el-border-color-lighter);
}

.download-btn {
    flex: 1;
    background: linear-gradient(135deg, var(--el-color-primary), var(--el-color-primary-light-3));
    border: none;
    padding: 8px 16px;
    font-size: 13px;
    font-weight: 500;
    border-radius: 8px;
}

.download-btn:hover {
    background: linear-gradient(135deg, var(--el-color-primary-light-3), var(--el-color-primary-light-5));
    transform: scale(1.02);
}

.copy-btn {
    flex-shrink: 0;
    color: var(--el-color-primary);
    padding: 8px 12px;
    font-size: 13px;
    border-radius: 8px;
    background: var(--el-color-primary-light-9);
}

.copy-btn:hover {
    background: var(--el-color-primary-light-8);
    color: var(--el-color-primary-dark-2);
}
</style>
