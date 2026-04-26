#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;
mod utils;

use state::AppState;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

fn main() {
    simple_logging::log_to_stderr(log::LevelFilter::Info);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            app.manage(AppState::new(app_handle)?);

            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_setting,
            commands::settings::set_setting,
            commands::settings::get_state,
            commands::settings::set_state,
            commands::app::relaunch,
            commands::app::get_version,
            commands::app::quit,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Veskto");
}
