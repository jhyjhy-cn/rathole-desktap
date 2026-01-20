# RatholeDesktop Development Plan

## 1. Project Initialization & Architecture
### Frontend (Vue 3 + TypeScript)
- Initialize with Vite, Vue 3, TypeScript.
- Install **Pinia** for state management.
- Install **Element Plus** and configure custom theme (Dark mode optimized).
- Configure **Vue Router** for navigation.

### Backend (Tauri 2 + Rust)
- Set up Tauri 2 environment.
- Configure `tauri.conf.json` for permissions (shell, fs, http, system-tray).
- Implement Rust modules:
    - `process_manager`: Handle `rathole` binary execution (spawn, kill, monitor).
    - `config_manager`: Read/Write `toml` configuration files.
    - `downloader`: Fetch and extract `rathole` binaries from GitHub/Mirrors.

## 2. Core Feature Implementation
### A. Service Management (Rust & Pinia)
- **Rust**:
    - `start_service(config_path)`: Spawns `rathole` process.
    - `stop_service()`: Terminates the process.
    - `emit_log`: Capture `stdout`/`stderr` and emit events to frontend.
- **Frontend**:
    - `useServiceStore`: Tracks running state, CPU/RAM usage (mock/real), and logs.

### B. Configuration Editor (Visual TOML)
- **Data Model**: Map TOML structure to TypeScript interfaces (`ClientConfig`, `Service`).
- **UI**:
    - **Global Settings**: `remote_addr`, `token`, `transport` (TCP/Noise/TLS).
    - **Service List**: Cards for each service (SSH, Web, etc.).
    - **Service Editor**: Form to add/edit `local_addr`, `token`, `type`.

### C. Download Manager
- Check local `rathole` binary existence/version.
- Fetch latest release info from GitHub API.
- Download and unzip logic with progress feedback.

### D. System Tray
- Cross-platform tray icon.
- Menu: "Show Window", "Start/Stop Service", "Exit".
- Sync tray tooltip/icon with service status.

## 3. UI/UX Design (Element Plus)
- **Layout**: Collapsible Sidebar + Main Content Area.
- **Theme**: Dark/Brand color scheme matching `rathole` (or generic modern tech blue/purple).
- **Responsive**: Grid system to adapt to mobile/desktop.
- **Animations**: Page transitions and list animations (300ms).

## 4. Implementation Steps
1.  **Scaffold**: Create project structure and install dependencies.
2.  **Rust Backend**: Implement `cmd` handlers and `process` logic.
3.  **UI Components**: Build Layout, Sidebar, and basic views.
4.  **Logic Integration**: Connect buttons to Tauri commands.
5.  **Refinement**: Error handling (Toast), logging window, polish UI.
6.  **Build**: Configure `tauri-action` or build scripts for release.

## 5. Verification
- **Unit Tests**: Check config parsing logic.
- **Manual Test**:
    - Download binary.
    - Create a valid config.
    - Start service and verify connection (logs).
    - Test Tray functionality.
