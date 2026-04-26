use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, RwLock};
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default = "default_true")]
    pub tray: bool,
    #[serde(default = "default_true")]
    pub minimize_to_tray: bool,
    #[serde(default = "default_false")]
    pub disable_min_size: bool,
    #[serde(default = "default_false")]
    pub ar_rpc: bool,
    #[serde(default = "default_false")]
    pub autostart: bool,
    #[serde(default = "default_false")]
    pub hardware_acceleration: bool,
    #[serde(default = "default_discord_branch")]
    pub discord_branch: String,
    #[serde(default = "default_false")]
    pub first_launch: bool,
}

fn default_true() -> bool {
    true
}
fn default_false() -> bool {
    false
}
fn default_discord_branch() -> String {
    "stable".to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            tray: true,
            minimize_to_tray: true,
            disable_min_size: false,
            ar_rpc: false,
            autostart: false,
            hardware_acceleration: false,
            discord_branch: "stable".to_string(),
            first_launch: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    #[serde(default)]
    pub maximized: bool,
    #[serde(default)]
    pub window_bounds: Option<WindowBounds>,
    #[serde(default = "default_layout_version")]
    pub steam_os_layout_version: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

fn default_layout_version() -> Option<u32> {
    None
}

impl Default for State {
    fn default() -> Self {
        Self {
            maximized: false,
            window_bounds: None,
            steam_os_layout_version: None,
        }
    }
}

pub struct AppState {
    pub app_handle: AppHandle,
    pub settings: RwLock<Settings>,
    pub state: RwLock<State>,
    pub settings_path: Mutex<PathBuf>,
    pub state_path: Mutex<PathBuf>,
}

impl AppState {
    pub fn new(app_handle: AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let data_dir = Self::get_data_dir()?;
        std::fs::create_dir_all(&data_dir)?;

        let settings_path = data_dir.join("settings.json");
        let state_path = data_dir.join("state.json");

        let settings = Self::load_json(&settings_path).unwrap_or_default();
        let state = Self::load_json(&state_path).unwrap_or_default();

        Ok(Self {
            app_handle,
            settings: RwLock::new(settings),
            state: RwLock::new(state),
            settings_path: Mutex::new(settings_path),
            state_path: Mutex::new(state_path),
        })
    }

    fn get_data_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Ok(data_dir) = std::env::var("VESKTO_DATA_DIR") {
            return Ok(PathBuf::from(data_dir));
        }

        let project_dirs = dirs::data_local_dir()
            .ok_or("Could not determine data directory")?
            .join("veskto");

        Ok(project_dirs)
    }

    fn load_json<T: Default + for<'de> Deserialize<'de>>(path: &PathBuf) -> Option<T> {
        if path.exists() {
            let content = std::fs::read_to_string(path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    pub fn save_settings(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings = self.settings.read().unwrap();
        let content = serde_json::to_string_pretty(&*settings)?;
        let path = self.settings_path.lock().unwrap();
        std::fs::write(&*path, content)?;
        Ok(())
    }

    pub fn save_state(&self) -> Result<(), Box<dyn std::error::Error>> {
        let state = self.state.read().unwrap();
        let content = serde_json::to_string_pretty(&*state)?;
        let path = self.state_path.lock().unwrap();
        std::fs::write(&*path, content)?;
        Ok(())
    }

    pub fn emit_settings_changed(&self) {
        let _ = self.app_handle.emit("settings-changed", ());
    }

    pub fn emit_state_changed(&self) {
        let _ = self.app_handle.emit("state-changed", ());
    }
}
