use crate::state::AppState;
use tauri::{Manager, State, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
pub fn create_main_window(
    app: tauri::AppHandle,
    state: State<AppState>,
    start_minimized: bool,
) -> Result<(), String> {
    let settings = state.settings.read().map_err(|e| e.to_string())?;
    let app_state = state.state.read().map_err(|e| e.to_string())?;

    let mut builder = WebviewWindowBuilder::new(&app, "main", WebviewUrl::default())
        .title("Veskto")
        .inner_size(1280.0, 720.0)
        .resizable(true)
        .visible(!start_minimized)
        .decorations(false)
        .transparent(true);

    if let Some(bounds) = &app_state.window_bounds {
        builder = builder
            .position(bounds.x as f64, bounds.y as f64)
            .inner_size(bounds.width as f64, bounds.height as f64);
    }

    let window = builder.build().map_err(|e| e.to_string())?;

    if app_state.maximized {
        let _ = window.maximize();
    }

    Ok(())
}

#[tauri::command]
pub fn create_splash_window(app: tauri::AppHandle) -> Result<(), String> {
    WebviewWindowBuilder::new(&app, "splash", WebviewUrl::App("splash.html".into()))
        .title("Veskto")
        .inner_size(400.0, 300.0)
        .resizable(false)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .center()
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn close_splash(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("splash") {
        let _ = window.close();
    }
}

#[tauri::command]
pub fn save_window_state(app: tauri::AppHandle, state: State<AppState>) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("Main window not found")?;

    let outer_pos = window.outer_position().ok();
    let inner_size = window.inner_size().ok();
    let is_maximized = window.is_maximized().unwrap_or(false);

    let mut app_state = state.state.write().map_err(|e| e.to_string())?;

    if let (Some(pos), Some(size)) = (outer_pos, inner_size) {
        app_state.window_bounds = Some(crate::state::app_state::WindowBounds {
            x: pos.x,
            y: pos.y,
            width: size.width,
            height: size.height,
        });
    }

    app_state.maximized = is_maximized;
    state.save_state().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn minimize_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.minimize();
    }
}

#[tauri::command]
pub fn maximize_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_maximized().unwrap_or(false) {
            let _ = window.unmaximize();
        } else {
            let _ = window.maximize();
        }
    }
}

#[tauri::command]
pub fn close_window(app: tauri::AppHandle, state: State<AppState>) {
    let settings = state.settings.read().unwrap();
    let minimize_to_tray = settings.minimize_to_tray;
    drop(settings);

    if minimize_to_tray {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.hide();
        }
    } else {
        app.exit(0);
    }
}

#[tauri::command]
pub fn show_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[tauri::command]
pub fn flash_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_always_on_top(true);
        let _ = window.set_always_on_top(false);
    }
}
