# Rathole Desktop

A cross-platform GUI client for [Rathole](https://github.com/rapiz1/rathole), a secure, stable and high-performance reverse proxy for NAT traversal.

## Features

- **Visual Configuration**: Manage your rathole services (Client & Server) via a user-friendly interface.
- **Process Control**: Start, stop, and monitor the rathole service with one click.
- **Live Logs**: View real-time logs from the rathole process.
- **Download Manager**: Built-in downloader to fetch the latest rathole binary from GitHub.
- **System Tray**: Run in the background with system tray support.
- **Cross-Platform**: Support for Windows, macOS, and Linux.

## Tech Stack

- **Frontend**: Vue 3, TypeScript, Element Plus, Pinia, Vite.
- **Backend**: Tauri 2 (Rust).

## Development Setup

1.  **Prerequisites**:
    - Node.js (v18+)
    - Rust (latest stable)
    - System dependencies for Tauri (WebView2 on Windows, WebKitGTK on Linux).

2.  **Install Dependencies**:
    ```bash
    npm install
    ```

3.  **Run in Development Mode**:
    ```bash
    npm run tauri dev
    ```

4.  **Build for Production**:
    ```bash
    npm run tauri build
    ```

## Usage

1.  **Download Rathole**: Go to the "Download" tab and fetch the latest binary.
2.  **Configure**:
    - Use "Proxy Config" to add services visually.
    - Or use "Settings" to paste an existing TOML config.
3.  **Start**: Go to "Dashboard" and click the Play button.
4.  **Monitor**: Check "Logs" for connection status.

## License

MIT
