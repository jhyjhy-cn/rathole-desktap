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
    border-radius: 8px;
    padding: 12px;
    background: white;
    transition: all 0.3s;
    height: 120px;
    display: flex;
    flex-direction: column;
}

.release-card:hover {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.card-header {
    padding-bottom: 8px;
    margin-bottom: 8px;
}

.version-name {
    font-size: 18px;
    font-weight: 600;
    color: var(--el-color-primary);
}

.card-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    margin-bottom: 8px;
}

.info-row {
    display: flex;
    justify-content: space-between;
    font-size: 13px;
    margin-bottom: 4px;
}

.info-label {
    color: #6b7280;
    font-weight: 500;
}

.info-value {
    color: #374151;
    font-weight: 500;
}

.size-badge {
    display: inline-block;
    padding: 2px 8px;
    background-color: #f3f4f6;
    border-radius: 4px;
    font-size: 12px;
    color: #6b7280;
    margin-top: 4px;
}

.card-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 8px;
    border-top: 1px solid #e5e7eb;
}

.download-btn {
    flex: 1;
    margin-right: 8px;
    background-color: var(--el-color-primary);
    border-color: var(--el-color-primary);
    padding: 6px 12px;
    font-size: 14px;
}

.download-btn:hover {
    background-color: var(--el-color-primary-light-3);
    border-color: var(--el-color-primary-light-3);
}

.copy-btn {
    flex-shrink: 0;
    color: var(--el-color-primary);
    padding: 6px 12px;
    font-size: 14px;
}

.copy-btn:hover {
    color: var(--el-color-primary-light-3);
}
</style>
