import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export const useUiStore = defineStore('ui', () => {
    // 从 localStorage 读取侧边栏折叠状态，默认为 false (展开)
    const savedCollapsed = localStorage.getItem('rathole-sidebar-collapsed')
    const isCollapsed = ref(savedCollapsed === 'true')

    function toggleSidebar() {
        isCollapsed.value = !isCollapsed.value
    }

    function setSidebarCollapsed(collapsed: boolean) {
        isCollapsed.value = collapsed
    }

    // 监听状态变化并持久化到 localStorage
    watch(isCollapsed, (val) => {
        localStorage.setItem('rathole-sidebar-collapsed', String(val))
    })

    return { isCollapsed, toggleSidebar, setSidebarCollapsed }
})
