mod config;

use config::{Browser, Config};

#[tauri::command]
fn get_browsers(app_handle: tauri::AppHandle) -> Result<Vec<Browser>, String> {
    let config = Config::load(&app_handle)?;
    Ok(config.browsers)
}

#[tauri::command]
async fn is_default_browser() -> Result<bool, String> {
    // Check if this application is the default browser
    todo!()
}

#[tauri::command]
async fn make_default_browser() -> Result<(), String> {
    // Make this application the default browser
    todo!()
}

#[tauri::command]
async fn url_to_open() -> Result<String, String> {
    // The URL passed to this program to open, if one was provided.
    // Otherwise return an empty string
    todo!()
}

#[tauri::command]
async fn open_url_in_browser(url: String, id: String) -> Result<(), String> {
    // Open the given URL with a specific browser id
    todo!()
}

#[tauri::command]
async fn open_config_in_vscode() -> Result<(), String> {
    // Open the JSON config file (if it exists) in VS Code for editing
    todo!()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_browsers,
            is_default_browser,
            make_default_browser,
            url_to_open,
            open_url_in_browser,
            open_config_in_vscode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
