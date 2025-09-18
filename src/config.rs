use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ui: UiConfig,
    pub search: SearchConfig,
    pub hotkeys: HotkeyConfig,
    pub appearance: AppearanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub window_width: f32,
    pub window_height: f32,
    pub always_on_top: bool,
    pub hide_on_launch: bool,
    pub animation_speed: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub max_results: usize,
    pub scan_paths: Vec<PathBuf>,
    pub file_extensions: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub enable_fuzzy_search: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub toggle_launcher: String,
    pub clear_search: String,
    pub launch_first: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub theme_variant: String, // "fade", "neon", "minimal", etc.
    pub transparency: f32,
    pub blur_background: bool,
    pub show_particles: bool,
    pub gradient_animation: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ui: UiConfig::default(),
            search: SearchConfig::default(),
            hotkeys: HotkeyConfig::default(),
            appearance: AppearanceConfig::default(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            window_width: 800.0,
            window_height: 600.0,
            always_on_top: true,
            hide_on_launch: true,
            animation_speed: 1.0,
        }
    }
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            max_results: 10,
            scan_paths: Vec::new(), // Will be populated from system defaults
            file_extensions: vec![
                ".exe".to_string(),
                ".lnk".to_string(),
                ".bat".to_string(),
                ".cmd".to_string(),
            ],
            exclude_patterns: vec![
                "unins".to_string(),
                "setup".to_string(),
                "install".to_string(),
            ],
            enable_fuzzy_search: true,
        }
    }
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            toggle_launcher: "Ctrl+Space".to_string(),
            clear_search: "Escape".to_string(),
            launch_first: "Enter".to_string(),
        }
    }
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            theme_variant: "fade".to_string(),
            transparency: 0.95,
            blur_background: true,
            show_particles: true,
            gradient_animation: true,
        }
    }
}

impl Config {
    /// Load configuration from file or create default
    pub fn load() -> Self {
        if let Some(config_path) = Self::get_config_path() {
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path) {
                    if let Ok(config) = serde_json::from_str(&content) {
                        return config;
                    }
                }
            }
        }
        
        // Return default config if loading fails
        Config::default()
    }
    
    /// Save configuration to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config_path) = Self::get_config_path() {
            // Create directory if it doesn't exist
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            let content = serde_json::to_string_pretty(self)?;
            fs::write(config_path, content)?;
        }
        
        Ok(())
    }
    
    /// Get the configuration file path
    fn get_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join("fade-launcher").join("config.json"))
    }
    
    /// Update a specific configuration value
    pub fn update_ui_setting(&mut self, setting: &str, value: serde_json::Value) -> Result<(), String> {
        match setting {
            "window_width" => {
                if let Some(val) = value.as_f64() {
                    self.ui.window_width = val as f32;
                } else {
                    return Err("Invalid value for window_width".to_string());
                }
            }
            "window_height" => {
                if let Some(val) = value.as_f64() {
                    self.ui.window_height = val as f32;
                } else {
                    return Err("Invalid value for window_height".to_string());
                }
            }
            "always_on_top" => {
                if let Some(val) = value.as_bool() {
                    self.ui.always_on_top = val;
                } else {
                    return Err("Invalid value for always_on_top".to_string());
                }
            }
            _ => {
                return Err(format!("Unknown setting: {}", setting));
            }
        }
        
        Ok(())
    }
}