<script setup lang="ts">
import { useConfigStore } from "../stores/config";
import { useThemeStore } from "../stores/theme";
import { useUiStore } from "../stores/ui";
import { storeToRefs } from "pinia";
import { ref, computed, onMounted, watch } from "vue";
import { open, save } from "@tauri-apps/plugin-dialog";
import { ElMessage } from "element-plus";
import { useI18n } from "vue-i18n";
import {
    Moon,
    Sunny,
    Setting as SettingIcon,
    Document,
    Download,
    CopyDocument,
} from "@element-plus/icons-vue";
import PageHeader from "../components/PageHeader.vue";

const store = useConfigStore();
const themeStore = useThemeStore();
const uiStore = useUiStore();
const { currentConfig, rawConfig } = storeToRefs(store);
const { isDark } = storeToRefs(themeStore);
const { isCollapsed } = storeToRefs(uiStore);
const currentFile = ref("");
const { locale } = useI18n();
const showConfigPreview = ref(false);

// Rathole version settings
const selectedVersion = ref("");
const downloadedVersions = ref<string[]>([]);

// Client config - use refs for proper v-model binding
const remoteAddr = ref("");

// Sync with store when config changes
watch(
    () => currentConfig.value?.client?.remote_addr || "",
    (val) => (remoteAddr.value = val),
    { immediate: true }
);

// Save client config to store
function saveClientConfig() {
    const cfg = JSON.parse(JSON.stringify(currentConfig.value || {}));
    if (!cfg.client) cfg.client = {};
    // Remove old token from client level if exists
    if (cfg.client.token) {
        delete cfg.client.token;
    }
    // Remove old server_addr if exists
    if (cfg.client.server_addr) {
        delete cfg.client.server_addr;
    }
    cfg.client.remote_addr = remoteAddr.value;
    store.updateFromObject(cfg);
}

// Load settings from localStorage
onMounted(() => {
    const savedVersion = localStorage.getItem("rathole-selected-version");
    if (savedVersion) {
        selectedVersion.value = savedVersion;
    }
    const saved = localStorage.getItem("rathole-downloaded-versions");
    if (saved) {
        downloadedVersions.value = JSON.parse(saved);
    }
});

async function loadFile() {
    const file = await open({
        filters: [{ name: "Config", extensions: ["toml"] }],
    });
    if (file) {
        currentFile.value = file as string;
        await store.loadConfig(file as string);
    }
}

async function saveFile() {
    if (!currentFile.value) {
        const file = await save({
            filters: [{ name: "Config", extensions: ["toml"] }],
        });
        if (file) currentFile.value = file as string;
        else return;
    }
    await store.saveConfig(currentFile.value);
    ElMessage.success("保存成功！");
}

function handleLocaleChange(val: string) {
    locale.value = val;
    localStorage.setItem("rathole-locale", val);
}

function handleVersionChange(val: string) {
    selectedVersion.value = val;
    localStorage.setItem("rathole-selected-version", val);
}

// Copy config to clipboard
async function copyConfig() {
    try {
        await navigator.clipboard.writeText(rawConfig.value);
        ElMessage.success("配置已复制到剪贴板");
    } catch (e) {
        ElMessage.error("复制失败");
    }
}

// Toggle config preview
function toggleConfigPreview() {
    showConfigPreview.value = !showConfigPreview.value;
}
</script>

<template>
    <div class="settings-page">
        <PageHeader>
            <template #header>
                <el-button @click="loadFile">
                    <el-icon><Document /></el-icon>
                    {{ $t("common.load") }}
                </el-button>
                <el-button type="primary" @click="saveFile">
                    <el-icon><Document /></el-icon>
                    {{ $t("common.save") }}
                </el-button>
                <el-button @click="toggleConfigPreview">
                    <el-icon><CopyDocument /></el-icon>
                    {{ showConfigPreview ? "隐藏配置预览" : "配置预览" }}
                </el-button>
            </template>
        </PageHeader>

        <div class="settings-content">
            <!-- General Settings -->
            <div class="setting-section">
                <div class="section-header">
                    <el-icon><SettingIcon /></el-icon>
                    <h3>{{ $t("settings.general") }}</h3>
                </div>
                <div class="section-body">
                    <div class="setting-row">
                        <div class="setting-label">
                            <el-icon><Document /></el-icon>
                            <span>{{ $t("settings.language") }}</span>
                        </div>
                        <el-select
                            v-model="locale"
                            @change="handleLocaleChange"
                            style="width: 200px"
                        >
                            <el-option label="中文 (简体)" value="zh-CN" />
                            <el-option label="English" value="en-US" />
                        </el-select>
                    </div>
                    <div class="setting-row">
                        <div class="setting-label">
                            <el-icon
                                ><component :is="isDark ? Moon : Sunny"
                            /></el-icon>
                            <span>{{ $t("settings.theme") }}</span>
                        </div>
                        <el-switch
                            v-model="isDark"
                            size="large"
                            :active-action-icon="Moon"
                            :inactive-action-icon="Sunny"
                            @change="
                                themeStore.setTheme(isDark ? 'dark' : 'light')
                            "
                        />
                    </div>
                    <div class="setting-row">
                        <div class="setting-label">
                            <el-icon><Download /></el-icon>
                            <span>Rathole 版本</span>
                        </div>
                        <el-select
                            v-model="selectedVersion"
                            @change="handleVersionChange"
                            style="width: 200px"
                            placeholder="选择版本"
                        >
                            <el-option
                                v-for="version in downloadedVersions"
                                :key="version"
                                :label="version"
                                :value="version"
                            />
                        </el-select>
                    </div>
                </div>
            </div>

            <!-- Client Config -->
            <div class="setting-section">
                <div class="section-header">
                    <el-icon><SettingIcon /></el-icon>
                    <h3>客户端配置</h3>
                </div>
                <div class="section-body">
                    <div class="form-item">
                        <label>服务端地址 (remote_addr)</label>
                        <el-input
                            v-model="remoteAddr"
                            @blur="saveClientConfig"
                            placeholder="server.example.com:2333"
                        >
                            <template #prefix>
                                <el-icon><SettingIcon /></el-icon>
                            </template>
                        </el-input>
                    </div>
                    <p class="hint">
                        Token 需要在每个服务下单独配置，请在"代理配置"页面设置
                    </p>
                </div>
            </div>

            <!-- Services -->
            <div class="setting-section">
                <div class="section-header">
                    <el-icon><SettingIcon /></el-icon>
                    <h3>服务列表</h3>
                </div>
                <div class="section-body">
                    <p class="hint">请在"代理配置"页面管理服务</p>
                    <div
                        v-if="currentConfig?.client?.services"
                        class="services-list"
                    >
                        <div
                            v-for="(svc, name) in currentConfig.client.services"
                            :key="name"
                            class="service-item"
                        >
                            <div class="service-info">
                                <span class="service-name">{{ name }}</span>
                                <span class="service-addr">{{
                                    (svc as any).local_addr
                                }}</span>
                            </div>
                            <el-tag
                                :type="(svc as any).token ? 'success' : 'danger'"
                                size="small"
                            >
                                {{ (svc as any).token ? "Token 已配置" : "Token 未设置" }}
                            </el-tag>
                        </div>
                    </div>
                    <div v-else class="empty-state">
                        <el-icon :size="32"><Document /></el-icon>
                        <p>暂无服务</p>
                    </div>
                </div>
            </div>

            <!-- Config Preview -->
            <div v-if="showConfigPreview" class="setting-section">
                <div class="section-header">
                    <el-icon><CopyDocument /></el-icon>
                    <h3>配置预览 (TOML)</h3>
                    <el-button
                        type="primary"
                        size="small"
                        @click="copyConfig"
                        style="margin-left: auto"
                    >
                        <el-icon><CopyDocument /></el-icon>
                        复制
                    </el-button>
                </div>
                <div class="section-body">
                    <div class="config-preview">
                        <pre>{{ rawConfig || "// 配置为空" }}</pre>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.settings-page {
    height: 100%;
    display: flex;
    flex-direction: column;
}

.settings-content {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
    max-width: 800px;
}

.setting-section {
    margin-bottom: 32px;
}

.section-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 16px;
    padding-bottom: 12px;
    border-bottom: 1px solid var(--el-border-color-light);
}

.section-header .el-icon {
    font-size: 20px;
    color: var(--el-color-primary);
}

.section-header h3 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
    color: var(--el-text-color-primary);
}

.section-body {
    padding-left: 32px;
}

.setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 0;
    min-height: 44px;
}

.setting-label {
    display: flex;
    align-items: center;
    gap: 10px;
    color: var(--el-text-color-regular);
    font-weight: 500;
}

.setting-label .el-icon {
    font-size: 18px;
    color: var(--el-color-primary);
}

.form-item {
    margin-bottom: 16px;
}

.form-item:last-child {
    margin-bottom: 0;
}

.form-item label {
    display: block;
    margin-bottom: 8px;
    font-size: 14px;
    font-weight: 500;
    color: var(--el-text-color-regular);
}

.hint {
    color: var(--el-text-color-secondary);
    font-size: 13px;
    margin: 0 0 16px 0;
}

.services-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.service-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-radius: 8px;
    background: var(--el-fill-color-light);
    border: 1px solid var(--el-border-color-lighter);
    transition: all 0.2s ease;
}

.service-item:hover {
    background: var(--el-fill-color);
    border-color: var(--el-color-primary-light-5);
}

.service-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.service-name {
    font-weight: 600;
    color: var(--el-text-color-primary);
    font-size: 14px;
}

.service-addr {
    font-size: 12px;
    color: var(--el-text-color-secondary);
    font-family: "SF Mono", "Monaco", "Consolas", monospace;
}

.empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px 20px;
    color: var(--el-text-color-secondary);
}

.empty-state .el-icon {
    margin-bottom: 12px;
}

.empty-state p {
    margin: 0;
    font-size: 14px;
}

.config-preview {
    background: var(--el-fill-color-light);
    border: 1px solid var(--el-border-color-light);
    border-radius: 8px;
    padding: 16px;
    overflow-x: auto;
}

.config-preview pre {
    margin: 0;
    font-family: "SF Mono", "Monaco", "Consolas", "Liberation Mono",
        "Courier New", monospace;
    font-size: 13px;
    line-height: 1.6;
    color: var(--el-text-color-primary);
    white-space: pre-wrap;
    word-wrap: break-word;
}
</style>
