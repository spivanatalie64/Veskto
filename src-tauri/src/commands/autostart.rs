use crate::state::AppState;
use std::path::PathBuf;
use tauri::State;

const DESKTOP_ENTRY: &str = r#"[Desktop Entry]
Type=Application
Name=Veskto
Exec=veskto --start-minimized
Icon=veskto
Terminal=false
Categories=Network;InstantMessaging;
Hidden=false
X-GNOME-Autostart-enabled=true
"#;

fn get_autostart_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_local_dir()
        .ok_or("Could not determine config directory")?
        .join("autostart");
    Ok(config_dir)
}

fn get_desktop_file() -> PathBuf {
    get_autostart_dir()
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
        .join("veskto.desktop")
}

#[tauri::command]
pub fn is_autostart_enabled() -> bool {
    get_desktop_file().exists()
}

#[tauri::command]
pub fn enable_autostart() -> Result<(), String> {
    let dir = get_autostart_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let desktop_file = get_desktop_file();
    std::fs::write(&desktop_file, DESKTOP_ENTRY).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn disable_autostart() -> Result<(), String> {
    let desktop_file = get_desktop_file();
    if desktop_file.exists() {
        std::fs::remove_file(&desktop_file).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn is_steam_deck_game_mode() -> bool {
    std::env::var("SteamOS").as_deref() == Ok("1")
        && std::env::var("SteamGamepadUI").as_deref() == Ok("1")
        && std::env::var("XDG_CURRENT_DESKTOP").as_deref() == Ok("gamescope")
}

#[tauri::command]
pub fn apply_steam_deck_fixes() {
    if is_steam_deck_game_mode() {
        std::env::set_var("GTK_IM_MODULE", "None");
    }
}
