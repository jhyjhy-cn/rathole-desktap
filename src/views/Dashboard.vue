<script setup lang="ts">
import { useRatholeStore } from "../stores/rathole";
import { useConfigStore } from "../stores/config";
import { storeToRefs } from "pinia";
import { open } from "@tauri-apps/plugin-dialog";
import { ElMessage } from "element-plus";
import PageHeader from "../components/PageHeader.vue";
import { useI18n } from "vue-i18n";

const ratholeStore = useRatholeStore();
const configStore = useConfigStore();
const { isRunning } = storeToRefs(ratholeStore);
const { t } = useI18n();

async function selectAndStart() {
    if (isRunning.value) {
        await ratholeStore.stop();
        ElMessage.info(t("dashboard.stopped"));
    } else {
        const file = await open({
            multiple: false,
            filters: [
                {
                    name: "Config",
                    extensions: ["toml"],
                },
            ],
        });
        if (file) {
            await configStore.loadConfig(file as string);
            await ratholeStore.start(file as string, false);
            ElMessage.success(t("dashboard.running"));
        }
    }
}
</script>

<template>
    <div class="dashboard-page">
        <PageHeader>
            <template #header></template>
        </PageHeader>
        <div class="home-page">
            <div class="container">
                <!-- Title -->
                <h1 class="title">{{ $t("dashboard.title") }}</h1>
                <p class="subtitle">{{ $t("dashboard.subtitle") }}</p>

                <!-- Main control -->
                <div class="control-section">
                    <div class="status-badge" :class="{ active: isRunning }">
                        <span class="dot"></span>
                        <span>{{
                            isRunning
                                ? $t("dashboard.running")
                                : $t("dashboard.stopped")
                        }}</span>
                    </div>

                    <button
                        :class="['main-button', { running: isRunning }]"
                        @click="selectAndStart"
                    >
                        <svg
                            v-if="!isRunning"
                            class="icon"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                        >
                            <path d="M8 5v14l11-7z" />
                        </svg>
                        <svg
                            v-else
                            class="icon"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                        >
                            <rect x="6" y="4" width="4" height="16" rx="1" />
                            <rect x="14" y="4" width="4" height="16" rx="1" />
                        </svg>
                        <span>{{
                            isRunning
                                ? $t("dashboard.stop")
                                : $t("dashboard.start")
                        }}</span>
                    </button>
                </div>

                <!-- Quick links -->
                <div class="quick-links">
                    <router-link to="/proxy" class="link-item">
                        <svg
                            class="link-icon"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <path
                                d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                            />
                        </svg>
                        <span>{{ $t("dashboard.configFile") }}</span>
                    </router-link>
                    <router-link to="/settings" class="link-item">
                        <svg
                            class="link-icon"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <path
                                d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                            />
                            <circle cx="12" cy="12" r="3" />
                        </svg>
                        <span>{{ $t("sidebar.settings") }}</span>
                    </router-link>
                    <router-link to="/logs" class="link-item">
                        <svg
                            class="link-icon"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <path
                                d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                            />
                        </svg>
                        <span>{{ $t("sidebar.logs") }}</span>
                    </router-link>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.dashboard-page {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
}

.home-page {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--el-bg-color-page);
}

.container {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    max-width: 400px;
    width: 100%;
    padding: 24px;
    margin: 0 auto;
}

.title {
    font-size: 24px;
    font-weight: 600;
    color: var(--el-text-color-primary);
    margin: 0 0 8px 0;
}

.subtitle {
    font-size: 14px;
    color: var(--el-text-color-secondary);
    margin: 0 0 48px 0;
}

.control-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 24px;
    margin-bottom: 48px;
}

.status-badge {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 6px 14px;
    background: var(--el-fill-color-light);
    border-radius: 16px;
    font-size: 13px;
    color: var(--el-text-color-secondary);
}

.status-badge.active {
    color: var(--el-color-success);
}

.dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--el-text-color-disabled);
}

.status-badge.active .dot {
    background: var(--el-color-success);
}

.main-button {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    width: 120px;
    height: 120px;
    border-radius: 50%;
    border: none;
    background: var(--el-color-primary);
    color: white;
    cursor: pointer;
    transition:
        background-color 0.2s ease,
        transform 0.2s ease;
}

.main-button:hover {
    background: var(--el-color-primary-light-3);
    transform: scale(1.05);
}

.main-button:active {
    transform: scale(0.98);
}

.main-button.running {
    background: var(--el-color-danger);
}

.main-button.running:hover {
    background: var(--el-color-danger-light-3);
}

.main-button .icon {
    width: 40px;
    height: 40px;
}

.main-button span {
    font-size: 14px;
    font-weight: 500;
}

.quick-links {
    display: flex;
    justify-content: center;
    gap: 16px;
}

.link-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 16px;
    background: var(--el-bg-color);
    border: 1px solid var(--el-border-color);
    border-radius: 12px;
    text-decoration: none;
    color: var(--el-text-color-regular);
    font-size: 13px;
    cursor: pointer;
    transition: all 0.2s ease;
}

.link-item:hover {
    border-color: var(--el-color-primary);
    color: var(--el-color-primary);
}

.link-icon {
    width: 24px;
    height: 24px;
}

@media (max-width: 480px) {
    .quick-links {
        flex-direction: column;
    }

    .link-item {
        flex-direction: row;
        justify-content: flex-start;
        padding: 12px 16px;
    }
}
</style>
