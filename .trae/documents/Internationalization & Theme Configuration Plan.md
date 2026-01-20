I will implement Internationalization (i18n) and Visual Theme Configuration for Rathole Desktop.

### 1. Internationalization (i18n) Setup
- **Install Dependency**: Add `vue-i18n` to the project.
- **Resource Files**:
    - Create `src/locales/zh-CN.ts` containing all Chinese translations.
    - Create `src/locales/en-US.ts` for English fallback.
- **Initialization**:
    - Create `src/i18n.ts` to configure the i18n instance.
    - Set default locale to `zh-CN`.
    - Implement persistence using `localStorage` to save user language preference.
- **Integration**: Register the i18n instance in `src/main.ts`.

### 2. UI Translation
- **Text Extraction**: Systematically review all Vue components (`src/views/*.vue`, `src/components/*.vue`) and extract hardcoded English strings.
- **Implementation**: Replace strings with the `$t()` function.
    - **Sidebar**: Dashboard, Proxy Config, Download, etc.
    - **Dashboard**: "Service Control", "Running", "Stopped", "Select Config File", etc.
    - **Proxy Config**: "Add Service", "Client", "Server", "Token", etc.
    - **Download**: "Download Rathole", "Version", "Download & Install", etc.
    - **Settings**: "Advanced Configuration", "Load", "Save".
    - **About**: Description and version info.
- **Element Plus**: Configure Element Plus to use its Chinese locale (`zh-cn`).

### 3. Visual Mode (Theme) Configuration
- **Theme Store**: Create `src/stores/theme.ts` using Pinia to manage theme state (`light` / `dark`).
- **Default Light Mode**:
    - Remove the hardcoded `class="dark"` from `index.html`.
    - Ensure the store defaults to `light` if no preference is saved.
- **Persistence**: Save the selected theme to `localStorage`.
- **Style Adjustments**:
    - Review `src/style.css` and component styles to ensure they use CSS variables (e.g., `var(--el-bg-color)`, `var(--el-text-color)`) instead of hardcoded colors, ensuring compatibility with both modes.
- **Settings UI**:
    - Update `Settings.vue` to include a "General" section with:
        - Language Switcher (中文 / English)
        - Theme Switcher (Light / Dark)

### 4. Verification
- Verify all pages display in Chinese by default.
- Test language switching persists after reload.
- Test theme switching applies correctly and persists.
- Ensure UI consistency in both Light and Dark modes.
