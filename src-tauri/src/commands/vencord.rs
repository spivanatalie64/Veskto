use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_vencord_script(state: State<'_, AppState>) -> Result<String, String> {
    let loader = state.vencord_loader.lock().map_err(|e| e.to_string())?;
    loader
        .get_injection_script()
        .await
        .map_err(|e| e.to_string())
}
