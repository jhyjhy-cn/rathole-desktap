<script setup lang="ts">
import { ref, computed } from "vue";
import {
    Menu as IconMenu,
    Setting,
    Download,
    Monitor,
    InfoFilled,
    House,
} from "@element-plus/icons-vue";

import { useRouter } from "vue-router";

const isCollapse = ref(true);
const activeIndex = ref("1");
const router = useRouter();

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
    { index: "6", icon: InfoFilled, title: "sidebar.about" }
];
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
            >
                <el-menu-item :index="item.index">
                    <el-icon class="menu-icon">
                        <component :is="item.icon" />
                    </el-icon>
                </el-menu-item>
            </el-tooltip>
        </el-menu>
    </div>
</template>

<style scoped>
.sidebar-container {
    height: 100%;
    display: flex;
    flex-direction: column;
}

.logo-section {
    height: 60px;
    width: 60px;
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
    width: 60px !important;
}

.el-menu-vertical.el-menu--collapse {
    width: 60px;
}

.el-menu-item {
    height: 40px;
    line-height: 40px;
    padding: 0;
    justify-content: center;
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

@keyframes shake {
    0%, 100% {
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
