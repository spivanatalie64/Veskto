#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;
mod utils;

use state::AppState;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use utils::cli::{parse_args, print_help, print_version};

fn main() {
    let cli = parse_args();

    if cli.show_help {
        print_help();
        return;
    }

    if cli.show_version {
        print_version();
        return;
    }

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

            utils::protocol::register_protocol(&app_handle)
                .expect("Failed to register vesktop:// protocol");

            let state = AppState::new(app_handle.clone())?;
            let data_dir = {
                let loader = state.vencord_loader.lock().unwrap();
                loader.vencord_dir().clone()
            };

            tokio::spawn(async move {
                let loader = VencordLoader::new(data_dir);
                if let Err(e) = loader.ensure_vencord_files().await {
                    log::error!("Failed to ensure Vencord files: {}", e);
                }
            });

            app.manage(state);

            let state = app.state::<AppState>();
            let settings = state.settings.read().unwrap();
            let discord_branch = settings.discord_branch.clone();
            let discord_url = state::app_state::get_discord_url(&discord_branch);
            let app_state = state.state.read().unwrap();

            let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::App(discord_url.into()))
                .title("Veskto")
                .inner_size(1280.0, 720.0)
                .resizable(true)
                .visible(!cli.start_minimized)
                .decorations(false)
                .transparent(true);

            if let Some(bounds) = &app_state.window_bounds {
                builder = builder
                    .position(bounds.x as f64, bounds.y as f64)
                    .inner_size(bounds.width as f64, bounds.height as f64);
            }

            let window = builder.build()?;

            if app_state.maximized {
                let _ = window.maximize();
            }

            if let Some(url) = &cli.discord_url {
                let url = url.clone();
                let app_handle = app_handle.clone();
                tauri::async_runtime::spawn(async move {
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    if let Some(win) = app_handle.get_webview_window("main") {
                        let _ = win.eval(&format!("window.location.href = '{}';", url));
                    }
                });
            }

            let app_handle_clone = app_handle.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    let state = app_handle_clone.state::<AppState>();
                    let settings = state.settings.read().unwrap();
                    if settings.minimize_to_tray {
                        api.prevent_close();
                        if let Some(win) = app_handle_clone.get_webview_window("main") {
                            let _ = win.hide();
                        }
                    }
                }
            });

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
            commands::window::close_splash,
            commands::window::save_window_state,
            commands::window::minimize_window,
            commands::window::maximize_window,
            commands::window::close_window,
            commands::window::show_window,
            commands::window::flash_window,
            commands::vencord::get_vencord_script,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Veskto");
}
