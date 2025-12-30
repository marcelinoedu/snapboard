use tauri::State;
use crate::state::app_state::AppState;

#[tauri::command]
pub fn stop_app(
    app_state: State<'_, AppState>,
) {
    app_state.stop();
}