use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::Manager;
use tauri::path::BaseDirectory;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Browser {
    pub id: String, // Generate IDs using 'cuid2' crate
    pub name: String,
    pub path: String,
    pub icon: Option<String>, // File extension if icon exists (e.g., "png", "jpg")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub pattern: String,     // Regex pattern to match against URLs
    pub browser_id: String,  // ID of the browser to open matching URLs in
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub browsers: Vec<Browser>,
    #[serde(default)]
    pub rules: Vec<Rule>,
}

impl Config {
    /// Load config from the app data directory
    pub fn load(app_handle: &tauri::AppHandle) -> Result<Self, String> {
        let config_path = Self::get_config_path(app_handle)?;

        if !config_path.exists() {
            // Create default config if it doesn't exist
            let default_config = Self::create_default(app_handle)?;
            default_config.save(app_handle)?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let config: Config = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;

        Ok(config)
    }

    /// Save config to the app data directory
    pub fn save(&self, app_handle: &tauri::AppHandle) -> Result<(), String> {
        let config_path = Self::get_config_path(app_handle)?;

        // Ensure parent directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&config_path, content)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }

    /// Get the path to the config file
    pub fn get_config_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))?;

        Ok(app_data_dir.join("config.json"))
    }
}

/// Check whether a browser binary exists at the given path.
fn browser_is_installed(path: &str) -> bool {
    // Absolute paths: check the file directly
    if Path::new(path).is_absolute() {
        return Path::new(path).exists();
    }

    // Bare command names (Linux): look up via `which`
    std::process::Command::new("which")
        .arg(path)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Known browsers and their bundled icon asset filenames.
const KNOWN_BROWSER_ICONS: &[(&str, &str)] = &[
    ("chrome", "assets/chrome.png"),
    ("firefox", "assets/firefox.png"),
    ("edge", "assets/edge.png"),
    ("safari", "assets/safari.png"),
];

/// Returns the bundled asset path for a browser name if it matches a known browser.
/// Matching is case-insensitive and checks if the name contains the known browser key.
pub fn get_known_browser_asset(name: &str) -> Option<&'static str> {
    let name_normalized = name.trim().to_lowercase();
    KNOWN_BROWSER_ICONS
        .iter()
        .find(|(key, _)| name_normalized == *key)
        .map(|(_, asset)| *asset)
}

/// Copies a bundled browser icon asset to the user's icon directory.
/// Returns the file extension on success, or None if the asset can't be resolved.
pub fn copy_bundled_icon(
    app_handle: &tauri::AppHandle,
    asset_path: &str,
    browser_id: &str,
) -> Option<String> {
    let resource_path = app_handle
        .path()
        .resolve(asset_path, BaseDirectory::Resource)
        .ok()?;

    if !resource_path.exists() {
        return None;
    }

    let ext = Path::new(asset_path)
        .extension()?
        .to_str()?
        .to_string();

    let icons_dir = app_handle
        .path()
        .app_data_dir()
        .ok()?
        .join("icons");

    fs::create_dir_all(&icons_dir).ok()?;

    let dest = icons_dir.join(format!("{}.{}", browser_id, ext));
    fs::copy(&resource_path, &dest).ok()?;

    Some(ext)
}

impl Config {
    /// Create the default config by detecting installed browsers and copying
    /// bundled icons for known browsers.
    fn create_default(app_handle: &tauri::AppHandle) -> Result<Self, String> {
        let candidates: Vec<(&str, &str)> = vec![
            (
                "Chrome",
                if cfg!(target_os = "windows") {
                    "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe"
                } else if cfg!(target_os = "macos") {
                    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
                } else {
                    "google-chrome"
                },
            ),
            (
                "Firefox",
                if cfg!(target_os = "windows") {
                    "C:\\Program Files\\Mozilla Firefox\\firefox.exe"
                } else if cfg!(target_os = "macos") {
                    "/Applications/Firefox.app/Contents/MacOS/firefox"
                } else {
                    "firefox"
                },
            ),
            (
                "Edge",
                if cfg!(target_os = "windows") {
                    "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe"
                } else if cfg!(target_os = "macos") {
                    "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge"
                } else {
                    "microsoft-edge"
                },
            ),
            (
                "Safari",
                if cfg!(target_os = "macos") {
                    "/Applications/Safari.app/Contents/MacOS/Safari"
                } else {
                    "safari"
                },
            ),
        ];

        let browsers = candidates
            .into_iter()
            .filter(|(_, path)| browser_is_installed(path))
            .map(|(name, path)| {
                let id = cuid2::create_id();
                let icon = get_known_browser_asset(name)
                    .and_then(|asset| copy_bundled_icon(app_handle, asset, &id));
                Browser {
                    id,
                    name: name.to_string(),
                    path: path.to_string(),
                    icon,
                }
            })
            .collect();

        Ok(Config {
            browsers,
            rules: Vec::new(),
        })
    }
}
