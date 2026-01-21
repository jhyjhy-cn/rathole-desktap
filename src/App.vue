<script setup lang="ts">
import Sidebar from "./components/Sidebar.vue";
</script>

<template>
    <el-container class="layout-container">
        <el-aside width="auto">
            <Sidebar />
        </el-aside>
        <el-container>
            <el-header> </el-header>
            <el-main>
                <router-view v-slot="{ Component }">
                    <transition name="fade" mode="out-in">
                        <component :is="Component" />
                    </transition>
                </router-view>
            </el-main>
        </el-container>
    </el-container>
</template>

<style scoped>
.layout-container {
    height: 100vh;
}
.el-header {
    background-color: var(--el-bg-color);
    border-bottom: 1px solid var(--el-border-color);
    display: flex;
    align-items: center;
}
.el-aside {
    background-color: var(--el-bg-color-overlay);
    border-right: 1px solid var(--el-border-color);
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
