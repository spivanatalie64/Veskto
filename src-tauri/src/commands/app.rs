use crate::state::AppState;
use tauri::{Manager, State};

#[tauri::command]
pub fn relaunch(state: State<AppState>) -> Result<(), String> {
    state
        .app_handle
        .restart()
        .map_err(|e| format!("Failed to relaunch: {}", e))
}

#[tauri::command]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub fn quit(state: State<AppState>) {
    state.app_handle.exit(0);
}
