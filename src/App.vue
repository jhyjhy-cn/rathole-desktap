<script setup lang="ts">
import Sidebar from "./components/Sidebar.vue";
</script>

<template>
    <el-container class="layout-container">
        <el-aside width="60px">
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
