<script setup lang="ts">
import { computed, onMounted, onUnmounted } from "vue";
import {
    Menu as IconMenu,
    Setting,
    Download,
    Monitor,
    InfoFilled,
    House,
    DArrowLeft,
    DArrowRight,
    Cpu,
} from "@element-plus/icons-vue";

import { useRouter, useRoute } from "vue-router";
import { useUiStore } from "../stores/ui";
import { useSystemStatsStore } from "../stores/systemStats";
import { storeToRefs } from "pinia";

const uiStore = useUiStore();
const router = useRouter();
const route = useRoute();
const systemStatsStore = useSystemStatsStore();

const isCollapse = computed(() => uiStore.isCollapsed);

// 路由到菜单索引的映射
const routeToIndexMap: Record<string, string> = {
    "/": "1",
    "/proxy": "2",
    "/download": "3",
    "/settings": "4",
    "/logs": "5",
    "/about": "6",
};

// 根据当前路由计算激活的菜单项
const activeIndex = computed(() => routeToIndexMap[route.path] || "1");

const handleSelect = (key: string) => {
    switch (key) {
        case "1":
            router.push("/");
            break;
        case "2":
            router.push("/proxy");
            break;
        case "3":
            router.push("/download");
            break;
        case "4":
            router.push("/settings");
            break;
        case "5":
            router.push("/logs");
            break;
        case "6":
            router.push("/about");
            break;
    }
};

const menuItems = [
    { index: "1", icon: House, title: "sidebar.home" },
    { index: "2", icon: IconMenu, title: "sidebar.proxyConfig" },
    { index: "3", icon: Download, title: "sidebar.download" },
    { index: "4", icon: Setting, title: "sidebar.settings" },
    { index: "5", icon: Monitor, title: "sidebar.logs" },
    { index: "6", icon: InfoFilled, title: "sidebar.about" },
];

// Store refs
const { cpuUsage, memoryUsage, startPolling, stopPolling } = storeToRefs(systemStatsStore);

// Lifecycle hooks
onMounted(() => {
    startPolling();
});

onUnmounted(() => {
    stopPolling();
});

// Format percentage
function formatPercent(value: number): string {
    return `${value.toFixed(1)}%`;
}

// Get progress bar color based on usage
function getProgressColor(usage: number): string {
    if (usage < 50) return "#67c23a"; // green
    if (usage < 80) return "#e6a23c"; // orange
    return "#f56c6c"; // red
}

// Format memory to MB
function formatMemoryMB(value: number): string {
    return `${(value / 1024).toFixed(0)} MB`;
}
</script>

<template>
    <div class="sidebar-container">
        <div class="logo-section">
            <img src="/vite.svg" alt="Logo" class="logo-image" />
        </div>
        <el-menu
            :default-active="activeIndex"
            class="el-menu-vertical"
            :collapse="isCollapse"
            @select="handleSelect"
        >
            <el-tooltip
                v-for="item in menuItems"
                :key="item.index"
                :content="$t(item.title)"
                placement="right"
                :show-after="300"
                :disabled="!isCollapse"
            >
                <el-menu-item :index="item.index">
                    <el-icon class="menu-icon">
                        <component :is="item.icon" />
                    </el-icon>
                    <template #title>{{ $t(item.title) }}</template>
                </el-menu-item>
            </el-tooltip>
        </el-menu>

        <!-- System Stats -->
        <div class="system-stats">
            <div class="stat-row">
                <el-icon class="stat-icon" :style="{ color: getProgressColor(cpuUsage) }">
                    <Cpu />
                </el-icon>
                <span class="stat-text">CPU: {{ formatPercent(cpuUsage) }}</span>
            </div>
            <div class="stat-row">
                <el-icon class="stat-icon" :style="{ color: getProgressColor(memoryUsage) }">
                    <Monitor />
                </el-icon>
                <span class="stat-text">内存: {{ formatPercent(memoryUsage) }}</span>
            </div>
        </div>

        <div class="collapse-btn-wrapper">
            <el-tooltip
                :content="isCollapse ? $t('common.expand') : $t('common.collapse')"
                placement="right"
                :show-after="300"
                :disabled="!isCollapse"
            >
                <el-button
                    class="collapse-btn"
                    :icon="isCollapse ? DArrowRight : DArrowLeft"
                    circle
                    @click="uiStore.toggleSidebar"
                />
            </el-tooltip>
        </div>
    </div>
</template>

<style scoped>
.sidebar-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    transition: width 0.3s ease;
}

.logo-section {
    height: 60px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-bottom: 1px solid var(--el-border-color);
}

.logo-image {
    width: 50px;
    height: 50px;
}

.el-menu-vertical {
    flex: 1;
    border-right: none;
    overflow-x: hidden;
}

.el-menu-vertical:not(.el-menu--collapse) {
    width: 180px;
}

.el-menu-vertical.el-menu--collapse {
    width: 60px;
}

.el-menu-item {
    height: 48px;
    line-height: 48px;
}

.el-menu-item.is-active {
    background-color: var(--el-color-primary-light-9);
}

.el-menu-item.is-active .menu-icon {
    color: var(--el-color-primary);
}

.menu-icon {
    font-size: 20px;
    transition: transform 0.3s ease;
}

.el-menu-item:hover .menu-icon {
    animation: shake 0.5s ease-in-out;
}

.collapse-btn-wrapper {
    padding: 12px 0;
    display: flex;
    align-items: center;
    justify-content: center;
    border-top: 1px solid var(--el-border-color);
}

.system-stats {
    padding: 12px;
    border-top: 1px solid var(--el-border-color);
    background: var(--el-fill-color-extra-light);
}

.stat-row {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 8px;
}

.stat-row:last-child {
    margin-bottom: 0;
}

.stat-icon {
    font-size: 14px;
    flex-shrink: 0;
}

.stat-text {
    font-size: 12px;
    font-weight: 500;
    color: var(--el-text-color-primary);
    font-family: "SF Mono", "Monaco", "Consolas", monospace;
}

@keyframes shake {
    0%,
    100% {
        transform: rotate(0deg);
    }
    25% {
        transform: rotate(-10deg);
    }
    75% {
        transform: rotate(10deg);
    }
}
</style>
