use crate::state::AppState;
use serde_json::Value;
use tauri::State;

#[tauri::command]
pub fn get_setting(key: String, state: State<AppState>) -> Value {
    let settings = state.settings.read().unwrap();
    let settings_value = serde_json::to_value(&*settings).unwrap_or_default();
    settings_value
        .get(&key)
        .cloned()
        .unwrap_or(Value::Null)
}

#[tauri::command]
pub fn set_setting(key: String, value: Value, state: State<AppState>) -> Result<(), String> {
    let mut settings = state.settings.write().unwrap();
    let settings_value = serde_json::to_value(&mut *settings).map_err(|e| e.to_string())?;
    let mut settings_map = settings_value
        .as_object_mut()
        .ok_or("Settings is not an object")?
        .clone();
    settings_map.insert(key, value);
    let new_settings: crate::state::app_state::Settings =
        serde_json::from_value(serde_json::Value::Object(settings_map))
            .map_err(|e| e.to_string())?;
    *settings = new_settings;
    state.save_settings().map_err(|e| e.to_string())?;
    state.emit_settings_changed();
    Ok(())
}

#[tauri::command]
pub fn get_state(key: String, state: State<AppState>) -> Value {
    let state_data = state.state.read().unwrap();
    let state_value = serde_json::to_value(&*state_data).unwrap_or_default();
    state_value
        .get(&key)
        .cloned()
        .unwrap_or(Value::Null)
}

#[tauri::command]
pub fn set_state(key: String, value: Value, state: State<AppState>) -> Result<(), String> {
    let mut state_data = state.state.write().unwrap();
    let state_value = serde_json::to_value(&mut *state_data).map_err(|e| e.to_string())?;
    let mut state_map = state_value
        .as_object_mut()
        .ok_or("State is not an object")?
        .clone();
    state_map.insert(key, value);
    let new_state: crate::state::app_state::State =
        serde_json::from_value(serde_json::Value::Object(state_map))
            .map_err(|e| e.to_string())?;
    *state_data = new_state;
    state.save_state().map_err(|e| e.to_string())?;
    state.emit_state_changed();
    Ok(())
}
