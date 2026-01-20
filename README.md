# Rathole Desktop

[Rathole](https://github.com/rapiz1/rathole) 的跨平台 GUI 客户端。Rathole 是一个安全、稳定且高性能的反向代理工具，用于 NAT 穿透。

## 功能特性

- **可视化配置**：通过用户友好的界面管理 rathole 服务（客户端和服务端）。
- **进程控制**：一键启动、停止和监控 rathole 服务。
- **实时日志**：查看 rathole 进程的实时运行日志。
- **下载管理器**：内置下载器，可从 GitHub 获取最新的 rathole 二进制文件。
- **系统托盘**：支持最小化到系统托盘后台运行。
- **跨平台**：支持 Windows、macOS 和 Linux。
- **多语言支持**：支持简体中文和英文界面。
- **主题切换**：支持亮色和暗色主题切换。

## 技术栈

- **前端**: Vue 3, TypeScript, Element Plus, Pinia, Vite.
- **后端**: Tauri 2 (Rust).

## 开发环境搭建

1.  **前置要求**:
    - Node.js (v18+)
    - Rust (最新稳定版)
    - Tauri 的系统依赖 (Windows 上是 WebView2, Linux 上是 WebKitGTK)。

2.  **安装依赖**:
    ```bash
    npm install
    ```

3.  **运行开发模式**:
    ```bash
    npm run tauri dev
    ```

4.  **构建生产版本**:
    ```bash
    npm run tauri build
    ```

## 使用指南

1.  **下载 Rathole**: 转到“下载”标签页并获取最新的二进制文件。
2.  **配置**:
    - 使用“代理配置”以可视化方式添加服务。
    - 或者使用“设置”粘贴现有的 TOML 配置文件。
3.  **启动**: 转到“仪表盘”并点击播放按钮。
4.  **监控**: 在“日志”中检查连接状态。

## 许可证

MIT
