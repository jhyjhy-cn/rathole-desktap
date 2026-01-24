<script setup lang="ts">
import { computed } from "vue";
import Sidebar from "./components/Sidebar.vue";
import { useUiStore } from "./stores/ui";

const uiStore = useUiStore();
const sidebarWidth = computed(() => (uiStore.isCollapsed ? "60px" : "180px"));
</script>

<template>
    <el-container class="layout-container">
        <el-aside :width="sidebarWidth">
            <Sidebar />
        </el-aside>
        <el-main class="main-content">
            <router-view v-slot="{ Component }">
                <transition name="fade" mode="out-in">
                    <component :is="Component" />
                </transition>
            </router-view>
        </el-main>
    </el-container>
</template>

<style scoped>
.layout-container {
    height: 100vh;
}
.el-aside {
    background-color: var(--el-bg-color-overlay);
    border-right: 1px solid var(--el-border-color);
    transition: width 0.3s ease;
}
.main-content {
    height: 100%;
    padding: 0;
    display: flex;
    flex-direction: column;
}
.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}
</style>
