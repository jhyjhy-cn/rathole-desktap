use tauri::{AppHandle, Manager, Emitter};
use std::sync::Mutex;
use std::process::{Command, Child, Stdio};
use std::io::{BufReader, BufRead};
use std::thread;
use std::fs;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};

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
    
    // Return path
    let exe_name = if cfg!(target_os = "windows") { "rathole.exe" } else { "rathole" };
    Ok(app_dir.join(exe_name).to_string_lossy().to_string())
}

#[tauri::command]
fn start_rathole(app: AppHandle, state: tauri::State<RatholeState>, config_path: String, is_server: bool) -> Result<String, String> {
    let mut process_guard = state.process.lock().map_err(|e| e.to_string())?;
    
    if process_guard.is_some() {
        return Err("Rathole is already running".to_string());
    }

    // Check for local binary in app data dir
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
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
            
            if let Some(stdout) = stdout {
                let app_handle_out = app.clone();
                thread::spawn(move || {
                    let reader = BufReader::new(stdout);
                    for line in reader.lines() {
                        if let Ok(l) = line {
                            let _ = app_handle_out.emit("rathole-log", l);
                        }
                    }
                });
            }

            if let Some(stderr) = stderr {
                let app_handle_err = app.clone();
                thread::spawn(move || {
                    let reader = BufReader::new(stderr);
                    for line in reader.lines() {
                        if let Ok(l) = line {
                            // Prefix errors or handle them differently
                            let _ = app_handle_err.emit("rathole-log", format!("[ERR] {}", l));
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
            download_rathole
        ])
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("Rathole Desktop")
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "quit" => {
                            // TODO: Kill process if running
                            app.exit(0);
                        }
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                     if let TrayIconEvent::Click { .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                     }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
