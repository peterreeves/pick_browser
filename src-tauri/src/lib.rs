mod config;

use config::{Browser, Config};

#[tauri::command]
fn get_browsers(app_handle: tauri::AppHandle) -> Result<Vec<Browser>, String> {
    let config = Config::load(&app_handle)?;
    Ok(config.browsers)
}

#[tauri::command]
fn get_config_path(app_handle: tauri::AppHandle) -> Result<String, String> {
    let path = Config::get_config_path(&app_handle)?;
    Ok(path.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_browsers, get_config_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
