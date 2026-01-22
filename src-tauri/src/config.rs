use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Browser {
    pub id: String, // Generate IDs using 'cuid2' crate
    pub name: String,
    pub path: String,
    pub icon: Option<String>, // File extension if icon exists (e.g., "png", "jpg")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub browsers: Vec<Browser>,
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

impl Default for Config {
    fn default() -> Self {
        Config {
            browsers: vec![
                Browser {
                    id: cuid2::create_id(),
                    name: "Chrome".to_string(),
                    path: if cfg!(target_os = "windows") {
                        "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe".to_string()
                    } else if cfg!(target_os = "macos") {
                        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome".to_string()
                    } else {
                        "google-chrome".to_string()
                    },
                    icon: None,
                },
                Browser {
                    id: cuid2::create_id(),
                    name: "Firefox".to_string(),
                    path: if cfg!(target_os = "windows") {
                        "C:\\Program Files\\Mozilla Firefox\\firefox.exe".to_string()
                    } else if cfg!(target_os = "macos") {
                        "/Applications/Firefox.app/Contents/MacOS/firefox".to_string()
                    } else {
                        "firefox".to_string()
                    },
                    icon: None,
                },
            ],
        }
    }
}
