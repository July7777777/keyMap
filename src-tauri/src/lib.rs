// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn set_window_position(app: tauri::AppHandle, x: f64, y: f64) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window
            .set_position(tauri::LogicalPosition { x, y })
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn get_window_position(app: tauri::AppHandle) -> Result<(f64, f64), String> {
    if let Some(window) = app.get_webview_window("main") {
        let position = window.outer_position().map_err(|e| e.to_string())?;
        let logical_pos = position.to_logical::<f64>(window.scale_factor().unwrap_or(1.0));
        Ok((logical_pos.x, logical_pos.y))
    } else {
        Ok((0.0, 0.0))
    }
}

mod keyboard_listener;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            set_window_position,
            get_window_position
        ])
        .setup(|app| {
            keyboard_listener::start_keyboard_listener(app.handle());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
