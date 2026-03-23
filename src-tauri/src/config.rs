use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::Manager;

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
            let default_config = Self::default();
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

impl Default for Config {
    fn default() -> Self {
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
            .map(|(name, path)| Browser {
                id: cuid2::create_id(),
                name: name.to_string(),
                path: path.to_string(),
                icon: None,
            })
            .collect();

        Config {
            browsers,
            rules: Vec::new(),
        }
    }
}
