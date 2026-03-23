mod config;

use config::{copy_bundled_icon, get_known_browser_asset, Browser, Config, Rule};
use std::process::Command;
use std::sync::Mutex;

#[cfg(target_os = "macos")]
const BUNDLE_ID: &str = "website.peterreeves.pick-browser";

/// Stores the URL received via Apple Events (macOS) so the frontend can retrieve it.
struct OpenedUrl(Mutex<Option<String>>);

#[tauri::command]
fn get_browsers(app_handle: tauri::AppHandle) -> Result<Vec<Browser>, String> {
    let config = Config::load(&app_handle)?;
    Ok(config.browsers)
}

#[tauri::command]
async fn is_default_browser() -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        use windows::core::HSTRING;
        use windows::Win32::System::Com::{
            CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
        };
        use windows::Win32::UI::Shell::{
            ApplicationAssociationRegistration, IApplicationAssociationRegistration, AL_EFFECTIVE,
            AT_URLPROTOCOL,
        };

        // SAFETY: COM is initialised on this thread before use. The
        // IApplicationAssociationRegistration interface is a stable Windows API
        // and QueryCurrentDefault returns an owned PWSTR that is valid until dropped.
        unsafe {
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            let reg: IApplicationAssociationRegistration = CoCreateInstance(
                &ApplicationAssociationRegistration,
                None,
                CLSCTX_INPROC_SERVER,
            )
            .map_err(|e| format!("Failed to create COM instance: {}", e))?;

            for scheme in &["http", "https"] {
                let scheme_h = HSTRING::from(*scheme);
                let prog_id = reg
                    .QueryCurrentDefault(&scheme_h, AT_URLPROTOCOL, AL_EFFECTIVE)
                    .map_err(|e| format!("Failed to query default for {}: {}", scheme, e))?;

                let prog_id_str = prog_id
                    .to_string()
                    .map_err(|e| format!("Failed to read ProgId string: {}", e))?;
                if !prog_id_str.contains("PickBrowser") && !prog_id_str.contains("pick_browser") {
                    return Ok(false);
                }
            }

            Ok(true)
        }
    }

    #[cfg(target_os = "macos")]
    {
        use core_foundation::base::TCFType;
        use core_foundation::string::CFString;

        extern "C" {
            fn LSCopyDefaultHandlerForURLScheme(
                url_scheme: core_foundation::string::CFStringRef,
            ) -> core_foundation::string::CFStringRef;
        }

        let scheme = CFString::new("https");
        // SAFETY: `scheme` is a valid CFString that outlives this call.
        // LSCopyDefaultHandlerForURLScheme is a stable Core Foundation API that
        // returns a newly-created CFStringRef (or null), which we check below.
        let handler = unsafe { LSCopyDefaultHandlerForURLScheme(scheme.as_concrete_TypeRef()) };

        if handler.is_null() {
            return Ok(false);
        }

        // SAFETY: `handler` is non-null (checked above) and was returned by a
        // "Copy" function, so we own the reference. `wrap_under_create_rule`
        // adopts ownership and will release it when `handler_cf` is dropped.
        let handler_cf = unsafe { CFString::wrap_under_create_rule(handler) };
        let handler_str = handler_cf.to_string();

        Ok(handler_str.eq_ignore_ascii_case(BUNDLE_ID))
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
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

    #[cfg(target_os = "macos")]
    {
        use core_foundation::base::TCFType;
        use core_foundation::string::CFString;

        extern "C" {
            fn LSSetDefaultHandlerForURLScheme(
                url_scheme: core_foundation::string::CFStringRef,
                handler_bundle_id: core_foundation::string::CFStringRef,
            ) -> i32;
        }

        let bundle_id = CFString::new(BUNDLE_ID);

        for scheme in &["http", "https"] {
            let scheme_cf = CFString::new(scheme);
            // SAFETY: Both `scheme_cf` and `bundle_id` are valid CFStrings that
            // outlive this call. LSSetDefaultHandlerForURLScheme is a stable
            // Launch Services API; the OSStatus return code is checked below.
            let result = unsafe {
                LSSetDefaultHandlerForURLScheme(
                    scheme_cf.as_concrete_TypeRef(),
                    bundle_id.as_concrete_TypeRef(),
                )
            };
            if result != 0 {
                return Err(format!(
                    "Failed to set default handler for {}: OSStatus {}",
                    scheme, result
                ));
            }
        }

        Ok(())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        Err("Setting default browser is not supported on this platform".to_string())
    }
}

#[tauri::command]
async fn url_to_open(state: tauri::State<'_, OpenedUrl>) -> Result<String, String> {
    // Check for URL received via Apple Events (macOS)
    if let Some(url) = state.0.lock().unwrap().as_ref() {
        return Ok(url.clone());
    }

    // Fall back to command-line arguments (Windows)
    let args: Vec<String> = std::env::args().collect();
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
    close: bool,
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

    if close {
        app_handle.exit(0);
    }

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

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(["-a", "Visual Studio Code", &config_path.to_string_lossy()])
            .spawn()
            .map_err(|e| format!("Failed to open VS Code: {}", e))?;
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
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
        // No custom icon provided — try to use a bundled icon for known browsers
        get_known_browser_asset(&name).and_then(|asset| copy_bundled_icon(&app_handle, asset, &id))
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
    icon: Option<String>, // Base64-encoded image data (None = keep existing)
    icon_mime: Option<String>, // MIME type like "image/png"
    remove_icon: bool,    // If true, remove the existing icon
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

#[tauri::command]
async fn delete_browser(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let mut config = Config::load(&app_handle)?;

    let browser_idx = config
        .browsers
        .iter()
        .position(|b| b.id == id)
        .ok_or_else(|| format!("Browser with id '{}' not found", id))?;

    // Remove icon file if it exists
    if let Some(ext) = &config.browsers[browser_idx].icon {
        let icons_dir = get_icons_dir(&app_handle)?;
        let icon_path = icons_dir.join(format!("{}.{}", id, ext));
        let _ = std::fs::remove_file(icon_path);
    }

    // Remove browser from config
    config.browsers.remove(browser_idx);
    config.save(&app_handle)?;

    Ok(())
}

#[tauri::command]
fn get_rules(app_handle: tauri::AppHandle) -> Result<Vec<Rule>, String> {
    let config = Config::load(&app_handle)?;
    Ok(config.rules)
}

#[tauri::command]
async fn add_rule(
    app_handle: tauri::AppHandle,
    pattern: String,
    browser_id: String,
) -> Result<(), String> {
    // Validate the regex pattern
    regex::Regex::new(&pattern).map_err(|e| format!("Invalid regex pattern: {}", e))?;

    let mut config = Config::load(&app_handle)?;

    // Validate browser_id exists (empty means "prompt to choose")
    if !browser_id.is_empty() && !config.browsers.iter().any(|b| b.id == browser_id) {
        return Err(format!("Browser with id '{}' not found", browser_id));
    }

    config.rules.push(Rule {
        id: cuid2::create_id(),
        pattern,
        browser_id,
    });
    config.save(&app_handle)?;

    Ok(())
}

#[tauri::command]
async fn update_rule(
    app_handle: tauri::AppHandle,
    id: String,
    pattern: String,
    browser_id: String,
) -> Result<(), String> {
    // Validate the regex pattern
    regex::Regex::new(&pattern).map_err(|e| format!("Invalid regex pattern: {}", e))?;

    let mut config = Config::load(&app_handle)?;

    // Validate browser_id exists (empty means "prompt to choose")
    if !browser_id.is_empty() && !config.browsers.iter().any(|b| b.id == browser_id) {
        return Err(format!("Browser with id '{}' not found", browser_id));
    }

    let rule = config
        .rules
        .iter_mut()
        .find(|r| r.id == id)
        .ok_or_else(|| format!("Rule with id '{}' not found", id))?;

    rule.pattern = pattern;
    rule.browser_id = browser_id;
    config.save(&app_handle)?;

    Ok(())
}

#[tauri::command]
async fn delete_rule(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let mut config = Config::load(&app_handle)?;

    let rule_idx = config
        .rules
        .iter()
        .position(|r| r.id == id)
        .ok_or_else(|| format!("Rule with id '{}' not found", id))?;

    config.rules.remove(rule_idx);
    config.save(&app_handle)?;

    Ok(())
}

/// Check if a URL matches any rule. Returns the browser_id of the first matching rule, or null.
/// An empty browser_id means "prompt to choose" — return null to let the user pick.
#[tauri::command]
fn check_rules(app_handle: tauri::AppHandle, url: String) -> Result<Option<String>, String> {
    let config = Config::load(&app_handle)?;

    for rule in &config.rules {
        let re = regex::Regex::new(&rule.pattern)
            .map_err(|e| format!("Invalid regex pattern '{}': {}", rule.pattern, e))?;
        if re.is_match(&url) {
            // Empty browser_id means "prompt to choose" — stop checking further rules
            if rule.browser_id.is_empty() {
                return Ok(None);
            }
            // Verify the browser still exists
            if config.browsers.iter().any(|b| b.id == rule.browser_id) {
                return Ok(Some(rule.browser_id.clone()));
            }
        }
    }

    Ok(None)
}

#[tauri::command]
async fn exit_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(OpenedUrl(Mutex::new(None)))
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
            delete_browser,
            get_browser_icon,
            get_rules,
            add_rule,
            update_rule,
            delete_rule,
            check_rules,
            exit_app
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, _event| {
            #[cfg(target_os = "macos")]
            {
                use tauri::Manager;
                if let tauri::RunEvent::Opened { urls } = _event {
                    if let Some(url) = urls.first() {
                        let url_str = url.to_string();
                        if let Some(state) = _app.try_state::<OpenedUrl>() {
                            *state.0.lock().unwrap() = Some(url_str.clone());
                        }
                        use tauri::Emitter;
                        let _ = _app.emit("url-opened", url_str);
                    }
                }
            }
        });
}
