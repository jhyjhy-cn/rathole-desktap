<script setup lang="ts">
import { useRatholeStore } from "../stores/rathole";
import { storeToRefs } from "pinia";
import { ref, watch, nextTick, computed } from "vue";
import PageHeader from "../components/PageHeader.vue";
import { Search, Delete, Refresh, RefreshRight } from "@element-plus/icons-vue";
import { ElInput, ElCheckbox, ElButton, ElMessage } from "element-plus";

const store = useRatholeStore();
const { logs } = storeToRefs(store);
const { loadLogsFromFile } = store;
const logContainer = ref<HTMLElement | null>(null);
const autoScroll = ref(true);
const searchQuery = ref("");
const pausedLogs = ref<string[]>([]);
const isPaused = ref(false);
const isLoading = ref(false);

// 从日志行解析日志级别
function getLogLevel(log: string): string {
    const upperLog = log.toUpperCase();
    if (upperLog.includes("ERROR") || upperLog.includes("[ERR]"))
        return "error";
    if (upperLog.includes("WARN")) return "warn";
    if (upperLog.includes("INFO")) return "info";
    return "default";
}

// 根据级别获取日志颜色
function getLogColor(level: string): string {
    switch (level) {
        case "error":
            return "#f56c6c";
        case "warn":
            return "#e6a23c";
        case "info":
            return "#67c23a";
        default:
            return "#dcdcdc";
    }
}

// 高亮搜索匹配
function highlightMatch(text: string, query: string): string {
    if (!query) return text;
    const regex = new RegExp(
        `(${query.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")})`,
        "gi",
    );
    return text.replace(regex, "<mark>$1</mark>");
}

// 过滤和高亮的日志
const filteredLogs = computed(() => {
    const logList = isPaused.value ? pausedLogs.value : logs.value;
    if (!searchQuery.value) return logList;

    const query = searchQuery.value.toLowerCase();
    return logList.filter((log) => log.toLowerCase().includes(query));
});

// 监听新日志
watch(
    logs,
    () => {
        nextTick(() => {
            if (logContainer.value && autoScroll.value && !isPaused.value) {
                logContainer.value.scrollTop = logContainer.value.scrollHeight;
            }
        });
    },
    { deep: true },
);

// 清空日志
function clearLogs() {
    logs.value = [];
    pausedLogs.value = [];
    ElMessage.success("日志已清空");
}

// 切换暂停
function togglePause() {
    isPaused.value = !isPaused.value;
    if (isPaused.value) {
        pausedLogs.value = [...logs.value];
    } else {
        pausedLogs.value = [];
    }
}

// 导出日志
function exportLogs() {
    const logText = logs.value.join("\n");
    const blob = new Blob([logText], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `rathole-logs-${new Date().toISOString().slice(0, 19).replace(/[:-]/g, "")}.txt`;
    a.click();
    URL.revokeObjectURL(url);
    ElMessage.success("日志已导出");
}

// 从文件刷新日志
async function refreshLogs() {
    isLoading.value = true;
    try {
        await loadLogsFromFile();
        ElMessage.success("日志已刷新");
    } catch (e) {
        ElMessage.error("刷新失败");
    } finally {
        isLoading.value = false;
    }
}
</script>

<template>
    <div class="logs-page">
        <PageHeader>
            <template #header>
                <el-input
                    v-model="searchQuery"
                    placeholder="搜索日志..."
                    :prefix-icon="Search"
                    style="width: 300px; margin-right: 12px"
                    clearable
                />
                <el-checkbox v-model="autoScroll" style="margin-right: 12px"
                    >自动滚动</el-checkbox
                >
                <el-button
                    :icon="RefreshRight"
                    :loading="isLoading"
                    @click="refreshLogs"
                    style="margin-right: 12px"
                >
                    刷新
                </el-button>
                <el-button
                    :icon="isPaused ? Refresh : Delete"
                    @click="togglePause"
                    style="margin-right: 12px"
                >
                    {{ isPaused ? "恢复" : "暂停" }}
                </el-button>
                <el-button
                    :icon="Delete"
                    @click="clearLogs"
                    style="margin-right: 12px"
                    >清空</el-button
                >
                <el-button type="primary" @click="exportLogs">导出</el-button>
            </template>
        </PageHeader>

        <div class="log-stats" v-if="logs.length > 0">
            <span class="stat-item">
                <span class="stat-label">总计:</span>
                <span class="stat-value">{{ logs.length }}</span>
            </span>
            <span class="stat-item">
                <span class="stat-label error">ERROR:</span>
                <span class="stat-value">{{
                    logs.filter((l) => getLogLevel(l) === "error").length
                }}</span>
            </span>
            <span class="stat-item">
                <span class="stat-label warn">WARN:</span>
                <span class="stat-value">{{
                    logs.filter((l) => getLogLevel(l) === "warn").length
                }}</span>
            </span>
            <span class="stat-item">
                <span class="stat-label info">INFO:</span>
                <span class="stat-value">{{
                    logs.filter((l) => getLogLevel(l) === "info").length
                }}</span>
            </span>
            <span class="stat-item" v-if="searchQuery">
                <span class="stat-label">匹配:</span>
                <span class="stat-value">{{ filteredLogs.length }}</span>
            </span>
        </div>

        <div class="log-container" ref="logContainer">
            <div v-if="filteredLogs.length === 0" class="empty-logs">
                <p v-if="searchQuery">未找到匹配的日志</p>
                <p v-else>暂无日志</p>
            </div>
            <div
                v-for="(log, index) in filteredLogs"
                :key="index"
                class="log-line"
                :style="{ color: getLogColor(getLogLevel(log)) }"
                v-html="highlightMatch(log, searchQuery)"
            ></div>
        </div>
    </div>
</template>

<style>
mark {
    background-color: #ff9800;
    color: #000;
    padding: 0 2px;
    border-radius: 2px;
    font-weight: bold;
}
</style>

<style scoped>
.logs-page {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--el-bg-color-page);
}

.log-stats {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 12px 20px;
    background: var(--el-bg-color);
    border-bottom: 1px solid var(--el-border-color-light);
    font-size: 13px;
}

.stat-item {
    display: flex;
    align-items: center;
    gap: 6px;
}

.stat-label {
    font-weight: 500;
    color: var(--el-text-color-secondary);
}

.stat-label.error {
    color: #f56c6c;
}

.stat-label.warn {
    color: #e6a23c;
}

.stat-label.info {
    color: #67c23a;
}

.stat-value {
    font-weight: 600;
    color: var(--el-text-color-primary);
}

.log-container {
    flex: 1;
    margin: 16px;
    background: #1e1e1e;
    padding: 12px;
    border-radius: 8px;
    font-family:
        "SF Mono", "Monaco", "Consolas", "Liberation Mono", "Courier New",
        monospace;
    font-size: 13px;
    overflow-y: auto;
    overflow-x: auto;
    border: 1px solid var(--el-border-color-dark);
    box-shadow: inset 0 2px 8px rgba(0, 0, 0, 0.3);
}

.log-container::-webkit-scrollbar {
    width: 10px;
    height: 10px;
}

.log-container::-webkit-scrollbar-track {
    background: #2a2a2a;
    border-radius: 5px;
}

.log-container::-webkit-scrollbar-thumb {
    background: #555;
    border-radius: 5px;
}

.log-container::-webkit-scrollbar-thumb:hover {
    background: #666;
}

.log-line {
    margin-bottom: 4px;
    line-height: 1.6;
    word-break: break-all;
    white-space: pre-wrap;
    transition: background-color 0.15s ease;
    padding: 2px 6px;
    border-radius: 3px;
}

.log-line:hover {
    background: rgba(255, 255, 255, 0.05);
}

.empty-logs {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    color: #666;
    font-size: 14px;
}

.empty-logs p {
    margin: 0;
}
</style>
