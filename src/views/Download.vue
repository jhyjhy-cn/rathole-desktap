<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from "element-plus";
import { useI18n } from "vue-i18n";

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
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + " KB";
    return (bytes / 1024 / 1024).toFixed(2) + " MB";
}

function formatDate(date: string) {
    return new Date(date).toLocaleDateString();
}

function copyLink(url: string) {
    navigator.clipboard.writeText(url);
    ElMessage.success(t("common.copy"));
}
</script>

<template>
    <div class="download-page">
        <el-row :gutter="20" v-loading="loading">
            <el-col
                :span="12"
                v-for="release in releases"
                :key="release.name"
                class="release-col"
            >
                <el-card class="release-card" shadow="hover">
                    <div class="card-header">
                        <span class="version-name">{{ release.name }}</span>
                    </div>
                    <div class="card-body">
                        <div class="info-item">
                            <span class="label">{{ t("download.size") }}:</span>
                            <span class="value">{{
                                formatSize(release.size)
                            }}</span>
                        </div>
                        <div class="info-item">
                            <span class="label"
                                >{{ t("download.downloads") }}:</span
                            >
                            <span class="value">{{
                                release.downloads.toLocaleString()
                            }}</span>
                        </div>
                        <div class="info-item">
                            <span class="label"
                                >{{ t("download.releaseDate") }}:</span
                            >
                            <span class="value">{{
                                formatDate(release.published_at)
                            }}</span>
                        </div>
                    </div>
                    <div class="card-actions">
                        <el-button
                            type="primary"
                            :loading="downloading === release.name"
                            @click="startDownload(release)"
                            class="download-btn"
                        >
                            {{ t("download.downloadBtn") }}
                        </el-button>
                        <el-button
                            link
                            @click="copyLink(release.download_url)"
                            class="copy-btn"
                        >
                            {{ t("download.copyLink") }}
                        </el-button>
                    </div>
                </el-card>
            </el-col>
        </el-row>

        <el-empty
            v-if="!loading && releases.length === 0"
            :description="$t('common.loading')"
        />
    </div>
</template>

<style scoped>
.download-page {
    padding: 20px;
    padding-top: 0;
}

.release-col {
    margin-bottom: 20px;
}

.release-card {
    height: 100%;
    transition: all 0.3s;
}

.release-card:hover {
    transform: translateY(-4px);
}

.card-header {
    border-bottom: 1px solid var(--el-border-color-lighter);
    padding-bottom: 12px;
    margin-bottom: 12px;
}

.version-name {
    font-size: 18px;
    font-weight: 600;
    color: var(--el-color-primary);
}

.card-body {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.info-item {
    display: flex;
    justify-content: space-between;
    font-size: 13px;
}

.label {
    color: var(--el-text-color-secondary);
}

.value {
    color: var(--el-text-color-primary);
    font-weight: 500;
}

.card-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 16px;
    padding-top: 12px;
    border-top: 1px solid var(--el-border-color-lighter);
}

.download-btn {
    flex: 1;
    margin-right: 8px;
}

.copy-btn {
    flex-shrink: 0;
}
</style>
