use tauri::{AppHandle, Manager, Emitter};
use std::sync::Mutex;
use std::process::{Command, Child, Stdio};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::fs::{self, File, OpenOptions};
use std::time::Duration;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use chrono::Local;
use sysinfo::System;

// Shared state to hold the child process
struct RatholeState {
    process: Mutex<Option<Child>>,
}

#[tauri::command]
fn read_config(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_config(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_logs(app: AppHandle, lines: usize) -> Result<Vec<String>, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let date_str = Local::now().format("%Y-%m-%d").to_string();
    let log_path = app_dir.join(format!("rathole-{}.log", date_str));

    if !log_path.exists() {
        return Ok(vec![]);
    }

    let file = File::open(&log_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let all_lines: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    // Return last N lines
    let start = if all_lines.len() > lines {
        all_lines.len() - lines
    } else {
        0
    };

    Ok(all_lines[start..].to_vec())
}

#[tauri::command]
fn get_system_stats() -> Result<(f32, f32), String> {
    let mut sys = System::new_all();

    // First refresh to initialize
    sys.refresh_all();

    // Wait a bit for CPU measurement
    std::thread::sleep(Duration::from_millis(200));

    // Second refresh to get actual CPU usage
    sys.refresh_all();

    // Get CPU usage (global_cpu_usage returns f32 percentage)
    let cpu_usage = sys.global_cpu_usage();

    // Get memory usage in MB
    let used_memory = sys.used_memory();
    let memory_mb = (used_memory / 1024 / 1024) as f32;

    Ok((cpu_usage, memory_mb))
}

#[tauri::command]
async fn download_rathole(app: AppHandle, version: String) -> Result<String, String> {
    let target = if cfg!(target_os = "windows") {
        "x86_64-pc-windows-msvc"
    } else if cfg!(target_os = "macos") {
        "x86_64-apple-darwin"
    } else {
        "x86_64-unknown-linux-gnu"
    };

    let url = format!("https://github.com/rapiz1/rathole/releases/download/{}/rathole-{}.zip", version, target);

    // Download
    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;

    // Save zip
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    }
    let zip_path = app_dir.join("rathole.zip");
    fs::write(&zip_path, bytes).map_err(|e| e.to_string())?;

    // Unzip
    let file = fs::File::open(&zip_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = app_dir.join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).map_err(|e| e.to_string())?;
                }
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }

    // Cleanup
    let _ = fs::remove_file(zip_path);

    // Set executable permissions on Unix-like systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let exe_name = if cfg!(target_os = "windows") { "rathole.exe" } else { "rathole" };
        let exe_path = app_dir.join(exe_name);
        if exe_path.exists() {
            let mut perms = fs::metadata(&exe_path).map_err(|e| e.to_string())?.permissions();
            perms.set_mode(perms.mode() | 0o111); // Add executable bit
            fs::set_permissions(&exe_path, perms).map_err(|e| e.to_string())?;
        }
    }

    // Return path
    let exe_name = if cfg!(target_os = "windows") { "rathole.exe" } else { "rathole" };
    Ok(app_dir.join(exe_name).to_string_lossy().to_string())
}

#[tauri::command]
fn start_rathole(app: AppHandle, state: tauri::State<RatholeState>, config_path: String, is_server: bool) -> Result<String, String> {
    let mut process_guard = state.process.lock().map_err(|e| e.to_string())?;

    // Check if we think it's already running
    if process_guard.is_some() {
        // Verify if actually running by checking child process
        if let Some(child) = process_guard.as_mut() {
            // try_wait returns Ok(Some(exit_status)) if exited
            // returns Ok(None) if still running
            match child.try_wait() {
                Ok(Some(_)) => {
                    // Process exited, clean up
                    *process_guard = None;
                }
                Ok(None) => {
                    // Still running
                    return Ok("Already running".to_string());
                }
                Err(_) => {
                    // Error checking, assume not running
                    *process_guard = None;
                }
            }
        }
    }

    // Get log file path
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    }

    // Create log file with current date
    let date_str = Local::now().format("%Y-%m-%d").to_string();
    let log_path = app_dir.join(format!("rathole-{}.log", date_str));

    // Check for local binary in app data dir
    let exe_name = if cfg!(target_os = "windows") { "rathole.exe" } else { "rathole" };
    let local_bin = app_dir.join(exe_name);

    let cmd_path = if local_bin.exists() {
        local_bin.to_string_lossy().to_string()
    } else {
        "rathole".to_string()
    };

    let mut cmd = Command::new(cmd_path);

    // On Windows, create_no_window to avoid popup console
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    if is_server {
        cmd.arg("--server");
    } else {
        cmd.arg("--client"); // Optional if config has [client]
    }
    cmd.arg(&config_path);

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    match cmd.spawn() {
        Ok(mut child) => {
            let stdout = child.stdout.take();
            let stderr = child.stderr.take();
            let log_path_clone = log_path.clone();

            if let Some(stdout) = stdout {
                let app_handle_out = app.clone();
                let log_path_out = log_path_clone.clone();
                thread::spawn(move || {
                    let reader = BufReader::new(stdout);
                    let mut log_file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&log_path_out)
                        .ok();

                    for line in reader.lines() {
                        if let Ok(l) = line {
                            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                            let log_line = format!("[{}] {}", timestamp, l);

                            // Emit to frontend
                            let _ = app_handle_out.emit("rathole-log", &l);

                            // Write to file
                            if let Some(ref mut file) = log_file {
                                let _ = writeln!(file, "{}", log_line);
                                let _ = file.flush();
                            }
                        }
                    }
                });
            }

            if let Some(stderr) = stderr {
                let app_handle_err = app.clone();
                thread::spawn(move || {
                    let reader = BufReader::new(stderr);
                    let mut log_file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&log_path_clone)
                        .ok();

                    for line in reader.lines() {
                        if let Ok(l) = line {
                            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                            let log_line = format!("[{}] [ERR] {}", timestamp, l);

                            // Emit to frontend
                            let _ = app_handle_err.emit("rathole-log", format!("[ERR] {}", l));

                            // Write to file
                            if let Some(ref mut file) = log_file {
                                let _ = writeln!(file, "{}", log_line);
                                let _ = file.flush();
                            }
                        }
                    }
                });
            }

            *process_guard = Some(child);
            Ok("Started successfully".to_string())
        },
        Err(e) => Err(format!("Failed to start: {}", e)),
    }
}

#[tauri::command]
fn get_installed_version(app: AppHandle) -> Result<String, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let exe_name = if cfg!(target_os = "windows") { "rathole.exe" } else { "rathole" };
    let local_bin = app_dir.join(exe_name);

    if !local_bin.exists() {
        return Err("No installed rathole found".to_string());
    }

    // Try to get version by running rathole --version
    let output = Command::new(&local_bin)
        .arg("--version")
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let version_str = String::from_utf8_lossy(&output.stdout);
        // Parse version from output like "rathole 1.2.3"
        if let Some(v) = version_str.split_whitespace().nth(1) {
            return Ok(v.to_string());
        }
    }

    // Fallback: check if file exists
    Ok("unknown".to_string())
}

#[tauri::command]
fn save_temp_config(app: AppHandle, config: serde_json::Value) -> Result<String, String> {
    use toml::to_string_pretty;

    // Create app data dir if not exists
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    }

    // Convert JSON to TOML
    let toml_str = to_string_pretty(&config).map_err(|e| e.to_string())?;

    // Save to temp file
    let temp_path = app_dir.join("client_temp.toml");
    fs::write(&temp_path, toml_str).map_err(|e| e.to_string())?;

    Ok(temp_path.to_string_lossy().to_string())
}

#[tauri::command]
fn fix_rathole_permissions(app: AppHandle) -> Result<String, String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
        let exe_name = "rathole";
        let exe_path = app_dir.join(exe_name);

        if exe_path.exists() {
            let mut perms = fs::metadata(&exe_path).map_err(|e| e.to_string())?.permissions();
            perms.set_mode(perms.mode() | 0o111); // Add executable bit (rwxrwxrwx)
            fs::set_permissions(&exe_path, perms).map_err(|e| e.to_string())?;
            return Ok("Permissions fixed".to_string());
        }
        return Err("Rathole not found".to_string());
    }
    #[cfg(not(unix))]
    {
        return Ok("Not needed on this platform".to_string());
    }
}

#[tauri::command]
fn is_rathole_running(state: tauri::State<RatholeState>) -> bool {
    if let Ok(guard) = state.process.lock() {
        if guard.as_ref().is_some() {
            return true;
        }
    }
    false
}

#[tauri::command]
fn stop_rathole(state: tauri::State<RatholeState>) -> Result<String, String> {
    let mut process_guard = state.process.lock().map_err(|e| e.to_string())?;

    if let Some(mut child) = process_guard.take() {
        match child.kill() {
            Ok(_) => Ok("Stopped successfully".to_string()),
            Err(e) => Err(format!("Failed to kill process: {}", e)),
        }
    } else {
        Ok("Not running".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_http::init())
        .manage(RatholeState { process: Mutex::new(None) })
        .invoke_handler(tauri::generate_handler![
            start_rathole,
            stop_rathole,
            read_config,
            write_config,
            read_logs,
            download_rathole,
            get_installed_version,
            save_temp_config,
            fix_rathole_permissions,
            is_rathole_running,
            toggle_window,
            hide_window,
            get_system_stats
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Create menu items using AppHandle
            let show_i = MenuItem::with_id(&app_handle, "show", "显示窗口", true, None::<&str>)?;
            let hide_i = MenuItem::with_id(&app_handle, "hide", "隐藏窗口", true, None::<&str>)?;
            let separator1 = tauri::menu::PredefinedMenuItem::separator(&app_handle)?;
            let quit_i = MenuItem::with_id(&app_handle, "quit", "退出", true, None::<&str>)?;

            let menu = Menu::with_items(&app_handle, &[&show_i, &hide_i, &separator1, &quit_i])?;
            let app_handle_for_menu = app_handle.clone();

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("Rathole Desktop")
                .on_menu_event(move |handle, event| {
                    match event.id.as_ref() {
                        "show" => {
                            if let Some(window) = handle.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "hide" => {
                            if let Some(window) = handle.get_webview_window("main") {
                                let _ = window.hide();
                            }
                        }
                        "quit" => {
                            // Stop rathole if running before exit
                            if let Some(state) = handle.try_state::<RatholeState>() {
                                if let Ok(mut guard) = state.process.lock() {
                                    if let Some(mut child) = guard.take() {
                                        let _ = child.kill();
                                    }
                                }
                            }
                            handle.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    match event {
                        TrayIconEvent::Click { .. } | TrayIconEvent::DoubleClick { .. } => {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                        _ => {}
                    }
                })
                .build(&app_handle_for_menu)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn toggle_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().map_err(|e| e.to_string())? {
            window.hide().map_err(|e| e.to_string())?;
        } else {
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
fn hide_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}
