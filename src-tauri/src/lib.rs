mod config;

use config::{Browser, Config};
use std::process::Command;

#[tauri::command]
fn get_browsers(app_handle: tauri::AppHandle) -> Result<Vec<Browser>, String> {
    let config = Config::load(&app_handle)?;
    Ok(config.browsers)
}

#[tauri::command]
async fn is_default_browser() -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        // Check the UserChoice for HTTP protocol
        let user_choice_path =
            r"Software\Microsoft\Windows\Shell\Associations\UrlAssociations\http\UserChoice";

        match hkcu.open_subkey(user_choice_path) {
            Ok(key) => {
                let prog_id: Result<String, _> = key.get_value("ProgId");
                match prog_id {
                    Ok(id) => Ok(id.contains("PickBrowser") || id.contains("pick_browser")),
                    Err(_) => Ok(false),
                }
            }
            Err(_) => Ok(false),
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // For non-Windows platforms, return false for now
        Ok(false)
    }
}

#[tauri::command]
async fn make_default_browser() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::env;
        use winreg::enums::*;
        use winreg::RegKey;

        let exe_path =
            env::current_exe().map_err(|e| format!("Failed to get executable path: {}", e))?;
        let exe_path_str = exe_path.to_string_lossy().to_string();

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        // Register the application capabilities
        let app_key_path = r"Software\PickBrowser";
        let (app_key, _) = hkcu
            .create_subkey(app_key_path)
            .map_err(|e| format!("Failed to create app registry key: {}", e))?;

        app_key
            .set_value("ApplicationName", &"Pick Browser")
            .map_err(|e| format!("Failed to set ApplicationName: {}", e))?;
        app_key
            .set_value("ApplicationDescription", &"Browser Picker Application")
            .map_err(|e| format!("Failed to set ApplicationDescription: {}", e))?;

        // Register URL capabilities
        let capabilities_path = r"Software\PickBrowser\Capabilities";
        let (cap_key, _) = hkcu
            .create_subkey(capabilities_path)
            .map_err(|e| format!("Failed to create capabilities key: {}", e))?;

        cap_key
            .set_value("ApplicationName", &"Pick Browser")
            .map_err(|e| format!("Failed to set capability ApplicationName: {}", e))?;
        cap_key
            .set_value(
                "ApplicationDescription",
                &"Choose which browser to use for each link",
            )
            .map_err(|e| format!("Failed to set capability ApplicationDescription: {}", e))?;

        // Register URL associations
        let url_assoc_path = r"Software\PickBrowser\Capabilities\URLAssociations";
        let (url_key, _) = hkcu
            .create_subkey(url_assoc_path)
            .map_err(|e| format!("Failed to create URL associations key: {}", e))?;

        url_key
            .set_value("http", &"PickBrowserURL")
            .map_err(|e| format!("Failed to set http association: {}", e))?;
        url_key
            .set_value("https", &"PickBrowserURL")
            .map_err(|e| format!("Failed to set https association: {}", e))?;

        // Register the ProgID for URL handling
        let prog_id_path = r"Software\Classes\PickBrowserURL";
        let (prog_key, _) = hkcu
            .create_subkey(prog_id_path)
            .map_err(|e| format!("Failed to create ProgID key: {}", e))?;

        prog_key
            .set_value("", &"Pick Browser URL")
            .map_err(|e| format!("Failed to set ProgID default value: {}", e))?;
        prog_key
            .set_value("URL Protocol", &"")
            .map_err(|e| format!("Failed to set URL Protocol: {}", e))?;

        // Set the shell open command
        let command_path = r"Software\Classes\PickBrowserURL\shell\open\command";
        let (cmd_key, _) = hkcu
            .create_subkey(command_path)
            .map_err(|e| format!("Failed to create command key: {}", e))?;

        let command = format!("\"{}\" \"%1\"", exe_path_str);
        cmd_key
            .set_value("", &command)
            .map_err(|e| format!("Failed to set command: {}", e))?;

        // Register in RegisteredApplications
        let reg_apps_path = r"Software\RegisteredApplications";
        let (reg_apps_key, _) = hkcu
            .create_subkey(reg_apps_path)
            .map_err(|e| format!("Failed to create RegisteredApplications key: {}", e))?;

        reg_apps_key
            .set_value("PickBrowser", &r"Software\PickBrowser\Capabilities")
            .map_err(|e| format!("Failed to register application: {}", e))?;

        // Open Windows Settings to let user set default browser
        // Windows 10+ doesn't allow programmatic changes to default browser
        Command::new("cmd")
            .args(["/C", "start", "ms-settings:defaultapps"])
            .spawn()
            .map_err(|e| format!("Failed to open settings: {}", e))?;

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("Setting default browser is only supported on Windows".to_string())
    }
}

#[tauri::command]
async fn url_to_open() -> Result<String, String> {
    let args: Vec<String> = std::env::args().collect();
    // The URL is typically passed as the first argument after the executable
    // Filter for arguments that look like URLs (start with http:// or https://)
    for arg in args.iter().skip(1) {
        if arg.starts_with("http://") || arg.starts_with("https://") {
            return Ok(arg.clone());
        }
    }
    Ok(String::new())
}

#[tauri::command]
async fn open_url_in_browser(
    app_handle: tauri::AppHandle,
    url: String,
    id: String,
) -> Result<(), String> {
    let config = Config::load(&app_handle)?;

    let browser = config
        .browsers
        .iter()
        .find(|b| b.id == id)
        .ok_or_else(|| format!("Browser with id '{}' not found", id))?;

    Command::new(&browser.path)
        .arg(&url)
        .spawn()
        .map_err(|e| format!("Failed to open browser '{}': {}", browser.name, e))?;

    Ok(())
}

#[tauri::command]
async fn open_config_in_vscode(app_handle: tauri::AppHandle) -> Result<(), String> {
    let config_path = Config::get_config_path(&app_handle)?;

    if !config_path.exists() {
        return Err("Config file does not exist yet".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "code", &config_path.to_string_lossy()])
            .spawn()
            .map_err(|e| format!("Failed to open VS Code: {}", e))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new("code")
            .arg(&config_path)
            .spawn()
            .map_err(|e| format!("Failed to open VS Code: {}", e))?;
    }

    Ok(())
}

#[derive(serde::Serialize)]
pub struct BrowserIcon {
    pub data: String,      // Base64-encoded image data
    pub mime_type: String, // e.g., "image/png"
}

#[tauri::command]
async fn add_new_browser(
    app_handle: tauri::AppHandle,
    name: String,
    path: String,
    icon: Option<String>,      // Base64-encoded image data
    icon_mime: Option<String>, // MIME type like "image/png"
) -> Result<(), String> {
    let id = cuid2::create_id();

    // If icon provided, save it to the icons directory
    let icon_ext = if let (Some(icon_data), Some(mime)) = (&icon, &icon_mime) {
        let icons_dir = get_icons_dir(&app_handle)?;

        // Create icons directory if it doesn't exist
        std::fs::create_dir_all(&icons_dir)
            .map_err(|e| format!("Failed to create icons directory: {}", e))?;

        // Determine file extension from MIME type
        let ext = match mime.as_str() {
            "image/png" => "png",
            "image/jpeg" => "jpg",
            "image/webp" => "webp",
            "image/avif" => "avif",
            _ => return Err(format!("Unsupported image format: {}", mime)),
        };

        // Decode base64 and save
        use base64::Engine;
        let image_bytes = base64::engine::general_purpose::STANDARD
            .decode(icon_data)
            .map_err(|e| format!("Failed to decode icon data: {}", e))?;

        let icon_path = icons_dir.join(format!("{}.{}", id, ext));
        std::fs::write(&icon_path, image_bytes)
            .map_err(|e| format!("Failed to save icon: {}", e))?;

        Some(ext.to_string())
    } else {
        None
    };

    // Load config, add browser, and save
    let mut config = Config::load(&app_handle)?;
    config.browsers.push(Browser {
        id,
        name,
        path,
        icon: icon_ext,
    });
    config.save(&app_handle)?;

    Ok(())
}

#[tauri::command]
async fn get_browser_icon(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<Option<BrowserIcon>, String> {
    // Find the browser to get its icon extension
    let config = Config::load(&app_handle)?;
    let browser = config.browsers.iter().find(|b| b.id == id);

    let Some(browser) = browser else {
        return Err(format!("Browser with id '{}' not found", id));
    };

    let Some(ext) = &browser.icon else {
        return Ok(None);
    };

    let icons_dir = get_icons_dir(&app_handle)?;
    let icon_path = icons_dir.join(format!("{}.{}", id, ext));

    if !icon_path.exists() {
        return Ok(None);
    }

    let image_bytes =
        std::fs::read(&icon_path).map_err(|e| format!("Failed to read icon: {}", e))?;

    use base64::Engine;
    let base64_data = base64::engine::general_purpose::STANDARD.encode(&image_bytes);

    let mime_type = match ext.as_str() {
        "png" => "image/png",
        "jpg" => "image/jpeg",
        "webp" => "image/webp",
        "avif" => "image/avif",
        _ => "application/octet-stream",
    }
    .to_string();

    Ok(Some(BrowserIcon {
        data: base64_data,
        mime_type,
    }))
}

fn get_icons_dir(app_handle: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    use tauri::Manager;
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    Ok(app_data_dir.join("icons"))
}

#[tauri::command]
async fn get_browser(app_handle: tauri::AppHandle, id: String) -> Result<Browser, String> {
    let config = Config::load(&app_handle)?;

    config
        .browsers
        .into_iter()
        .find(|b| b.id == id)
        .ok_or_else(|| format!("Browser with id '{}' not found", id))
}

#[tauri::command]
async fn update_browser(
    app_handle: tauri::AppHandle,
    id: String,
    name: String,
    path: String,
    icon: Option<String>,      // Base64-encoded image data (None = keep existing)
    icon_mime: Option<String>, // MIME type like "image/png"
    remove_icon: bool,         // If true, remove the existing icon
) -> Result<(), String> {
    let mut config = Config::load(&app_handle)?;

    let browser_idx = config
        .browsers
        .iter()
        .position(|b| b.id == id)
        .ok_or_else(|| format!("Browser with id '{}' not found", id))?;

    let icons_dir = get_icons_dir(&app_handle)?;

    // Handle icon changes
    let new_icon_ext = if remove_icon {
        // Remove existing icon file if it exists
        if let Some(old_ext) = &config.browsers[browser_idx].icon {
            let old_icon_path = icons_dir.join(format!("{}.{}", id, old_ext));
            let _ = std::fs::remove_file(old_icon_path);
        }
        None
    } else if let (Some(icon_data), Some(mime)) = (&icon, &icon_mime) {
        // New icon provided - save it
        std::fs::create_dir_all(&icons_dir)
            .map_err(|e| format!("Failed to create icons directory: {}", e))?;

        let ext = match mime.as_str() {
            "image/png" => "png",
            "image/jpeg" => "jpg",
            "image/webp" => "webp",
            "image/avif" => "avif",
            _ => return Err(format!("Unsupported image format: {}", mime)),
        };

        // Remove old icon if extension differs
        if let Some(old_ext) = &config.browsers[browser_idx].icon {
            if old_ext != ext {
                let old_icon_path = icons_dir.join(format!("{}.{}", id, old_ext));
                let _ = std::fs::remove_file(old_icon_path);
            }
        }

        use base64::Engine;
        let image_bytes = base64::engine::general_purpose::STANDARD
            .decode(icon_data)
            .map_err(|e| format!("Failed to decode icon data: {}", e))?;

        let icon_path = icons_dir.join(format!("{}.{}", id, ext));
        std::fs::write(&icon_path, image_bytes)
            .map_err(|e| format!("Failed to save icon: {}", e))?;

        Some(ext.to_string())
    } else {
        // Keep existing icon
        config.browsers[browser_idx].icon.clone()
    };

    // Update browser entry
    config.browsers[browser_idx].name = name;
    config.browsers[browser_idx].path = path;
    config.browsers[browser_idx].icon = new_icon_ext;

    config.save(&app_handle)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_browsers,
            get_browser,
            is_default_browser,
            make_default_browser,
            url_to_open,
            open_url_in_browser,
            open_config_in_vscode,
            add_new_browser,
            update_browser,
            get_browser_icon
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
