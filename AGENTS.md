# AGENTS.md - 代码库指南

## 项目概述
Rathole Desktop 是基于 Tauri 2 + Vue 3 + TypeScript 的桌面应用，提供 Rathole 反向代理工具的图形界面。

## 技术栈
- **前端**: Vue 3 (Composition API), TypeScript, Element Plus, Pinia, Vue Router, Vite
- **后端**: Tauri 2 (Rust)
- **国际化**: Vue I18n (支持简体中文/英文)

## 开发命令

```bash
npm run dev          # Vite 开发服务器（端口 1420）
npm run tauri dev    # 完整开发模式
npm run build        # 类型检查 + 前端构建
npm run tauri build  # 构建完整应用
npm run preview      # 预览构建
vue-tsc -b           # 仅类型检查
```

## 代码风格指南

### 命名约定
- **变量/函数**: camelCase (如 `isRunning`, `loadConfig`)
- **组件**: PascalCase (如 `Dashboard.vue`)
- **常量**: UPPER_SNAKE_CASE
- **Tauri 命令**: snake_case (如 `start_rathole`)
- **Pinia stores**: useXxxStore 模式

### 导入顺序
1. 第三方库（Vue、Element Plus、Tauri）
2. 组件导入
3. 本地模块（stores、views、components）
4. 样式文件

### Pinia Stores (Composition API)
```typescript
export const useConfigStore = defineStore('config', () => {
  const currentConfig = ref<any>({})
  async function loadConfig(path: string) {
    try {
      const content = await invoke<string>('read_config', { path })
      currentConfig.value = parse(content)
    } catch (error) {
      console.error(error)
      throw error
    }
  }
  return { currentConfig, loadConfig }
})
```

### 路由定义
```typescript
{ path: '/', component: Dashboard }
{ path: '/settings', component: () => import('../views/Settings.vue') }
```

### TypeScript 配置
- `strict: true` - 严格模式
- `noUnusedLocals: true` - 未使用变量报错
- `noUnusedParameters: true` - 未使用参数报错
- 路径别名: `@/*` → `src/*`

### 错误处理
```typescript
async function someFunction() {
  try {
    await operation()
  } catch (error) {
    console.error(error)
    ElMessage.error('操作失败')
    throw error
  }
}
```

### 国际化
```typescript
// 模板: {{ $t('dashboard.title') }}
// 脚本: const { locale } = useI18n()
const { t } = useI18n({ useScope: 'global' })
```

### 样式规范
- 使用 `<style scoped>`
- 优先使用 Element Plus CSS 变量（如 `var(--el-text-color-primary)`）
- 2 空格缩进

## Tauri 命令规范

```rust
#[tauri::command]
fn start_rathole(
    app: AppHandle,
    state: tauri::State<RatholeState>,
    config_path: String,
    is_server: bool
) -> Result<String, String> { ... }

// 注册
.invoke_handler(tauri::generate_handler![
    start_rathole, stop_rathole, read_config, write_config
])

// 调用
await invoke('start_rathole', { configPath: path, isServer: false })
```

## 文件结构
```
src/
├── components/     # 可复用组件
├── views/         # 页面组件
├── stores/        # Pinia 状态
├── router/        # 路由
├── locales/       # 国际化
├── i18n.ts        # i18n 配置
├── App.vue        # 根组件
└── main.ts        # 入口

src-tauri/src/
├── main.rs        # Tauri 入口
└── lib.rs         # 主要逻辑
```

## 重要注意事项

1. **没有测试框架**: 未配置测试
2. **开发端口**: 固定 1420 端口
3. **热重载**: 忽略 `src-tauri` 监视
4. **类型安全**: 避免 `any` 类型
5. **Element Plus 图标**: 全局注册，直接使用
6. **日志限制**: 1000 条，自动移除旧日志
7. **进程管理**: Mutex 保护共享状态

## 常见模式

### Tauri Dialog
```typescript
import { open, save } from '@tauri-apps/plugin-dialog'
const file = await open({
  multiple: false,
  filters: [{ name: 'Config', extensions: ['toml'] }]
})
```

### Element Plus 消息
```typescript
import { ElMessage } from 'element-plus'
ElMessage.success('操作成功')
```

### Pinia 响应式
```typescript
import { storeToRefs } from 'pinia'
const store = useConfigStore()
const { currentConfig } = storeToRefs(store)
```
