use crate::state::AppState;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager, State,
};

pub fn create_tray(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>, None)?;
    let about = MenuItem::with_id(app, "about", "About Veskto", true, None::<&str>, None)?;
    let reset = MenuItem::with_id(app, "reset", "Reset Settings", true, None::<&str>, None)?;
    let restart = MenuItem::with_id(app, "restart", "Restart", true, None::<&str>, None)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>, None)?;

    let menu = Menu::with_items(app, &[&show, &about, &reset, &restart, &quit])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "about" => {
                // TODO: open about window
            }
            "reset" => {
                let state = app.state::<AppState>();
                let mut settings = state.settings.write().unwrap();
                *settings = Default::default();
                let _ = state.save_settings();
            }
            "restart" => {
                let _ = app.restart();
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::DoubleClick { .. } = event {
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}

#[tauri::command]
pub fn set_tray_icon(app: tauri::AppHandle, _icon_path: String) -> Result<(), String> {
    // TODO: load custom icon from path
    // For now, use default
    Ok(())
}

#[tauri::command]
pub fn set_tray_tooltip(app: tauri::AppHandle, tooltip: String) {
    if let Some(tray) = app.tray_by_id("tray") {
        let _ = tray.set_tooltip(Some(&tooltip));
    }
}
