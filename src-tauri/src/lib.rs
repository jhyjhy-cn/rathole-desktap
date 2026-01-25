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
use sysinfo::{System, Pid};

// 用于保存子进程的共享状态
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

    // 返回最后 N 行
    let start = if all_lines.len() > lines {
        all_lines.len() - lines
    } else {
        0
    };

    Ok(all_lines[start..].to_vec())
}

#[tauri::command]
fn get_system_stats() -> Result<(f32, f32), String> {
    // 使用 new_all() 正确初始化 CPU 追踪
    let mut sys = System::new_all();

    // 获取当前进程 PID
    let current_pid = std::process::id();
    let pid = Pid::from_u32(current_pid);

    // 首次刷新以初始化
    sys.refresh_all();

    // 等待一小段时间以测量 CPU
    std::thread::sleep(Duration::from_millis(500));

    // 第二次刷新以获取实际 CPU 使用率
    sys.refresh_all();

    // 获取当前进程内存
    let memory_mb: f32 = sys.process(pid)
        .map(|p| (p.memory() / 1024 / 1024) as f32)
        .unwrap_or(0.0);

    // 获取当前进程 CPU 使用率
    let cpu_usage: f32 = sys.process(pid)
        .map(|p| p.cpu_usage())
        .unwrap_or(0.0);

    // 调试输出
    eprintln!("DEBUG: CPU usage = {}, Memory = {} MB", cpu_usage, memory_mb);

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

    // 下载
    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;

    // 保存 zip 文件
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    }
    let zip_path = app_dir.join("rathole.zip");
    fs::write(&zip_path, bytes).map_err(|e| e.to_string())?;

    // 解压
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

    // 清理
    let _ = fs::remove_file(zip_path);

    // 在类 Unix 系统上设置可执行权限
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let exe_name = if cfg!(target_os = "windows") { "rathole.exe" } else { "rathole" };
        let exe_path = app_dir.join(exe_name);
        if exe_path.exists() {
            let mut perms = fs::metadata(&exe_path).map_err(|e| e.to_string())?.permissions();
            perms.set_mode(perms.mode() | 0o111); // 添加可执行位
            fs::set_permissions(&exe_path, perms).map_err(|e| e.to_string())?;
        }
    }

    // 返回路径
    let exe_name = if cfg!(target_os = "windows") { "rathole.exe" } else { "rathole" };
    Ok(app_dir.join(exe_name).to_string_lossy().to_string())
}

#[tauri::command]
fn start_rathole(app: AppHandle, state: tauri::State<RatholeState>, config_path: String, is_server: bool) -> Result<String, String> {
    let mut process_guard = state.process.lock().map_err(|e| e.to_string())?;

    // 检查是否已在运行
    if process_guard.is_some() {
        // 通过检查子进程验证是否实际运行
        if let Some(child) = process_guard.as_mut() {
            // 如果进程已退出，try_wait 返回 Ok(Some(exit_status))
            // 如果仍在运行，返回 Ok(None)
            match child.try_wait() {
                Ok(Some(_)) => {
                    // 进程已退出，清理
                    *process_guard = None;
                }
                Ok(None) => {
                    // 仍在运行
                    return Ok("Already running".to_string());
                }
                Err(_) => {
                    // 检查出错，假设未运行
                    *process_guard = None;
                }
            }
        }
    }

    // 获取日志文件路径
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    }

    // 使用当前日期创建日志文件
    let date_str = Local::now().format("%Y-%m-%d").to_string();
    let log_path = app_dir.join(format!("rathole-{}.log", date_str));

    // 在应用数据目录中查找本地二进制文件
    let exe_name = if cfg!(target_os = "windows") { "rathole.exe" } else { "rathole" };
    let local_bin = app_dir.join(exe_name);

    let cmd_path = if local_bin.exists() {
        local_bin.to_string_lossy().to_string()
    } else {
        "rathole".to_string()
    };

    let mut cmd = Command::new(cmd_path);

    // 在 Windows 上，避免弹出控制台
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    if is_server {
        cmd.arg("--server");
    } else {
        cmd.arg("--client"); // 如果配置有 [client] 则可选
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

                            // 发送到前端
                            let _ = app_handle_out.emit("rathole-log", &l);

                            // 写入文件
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

                            // 发送到前端
                            let _ = app_handle_err.emit("rathole-log", format!("[ERR] {}", l));

                            // 写入文件
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

    // 尝试通过运行 rathole --version 获取版本
    let output = Command::new(&local_bin)
        .arg("--version")
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let version_str = String::from_utf8_lossy(&output.stdout);
        // 从输出如 "rathole 1.2.3" 中解析版本
        if let Some(v) = version_str.split_whitespace().nth(1) {
            return Ok(v.to_string());
        }
    }

    // 备选方案：检查文件是否存在
    Ok("unknown".to_string())
}

#[tauri::command]
fn save_temp_config(app: AppHandle, config: serde_json::Value) -> Result<String, String> {
    use toml::to_string_pretty;

    // 如果应用数据目录不存在则创建
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    }

    // 将 JSON 转换为 TOML
    let toml_str = to_string_pretty(&config).map_err(|e| e.to_string())?;

    // 保存到临时文件
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
            perms.set_mode(perms.mode() | 0o111); // 添加可执行位 (rwxrwxrwx)
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

            // 使用 AppHandle 创建菜单项
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
                            // 退出前停止 rathole（如果正在运行）
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
