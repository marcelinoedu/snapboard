use tauri::State;
use crate::state::clipboard_state::ClipboardState;
use crate::models::clipboard_item::ClipboardItem;
use arboard::Clipboard;


#[tauri::command]
pub fn list_clipboard_history(
    clipboard_state: State<'_, ClipboardState>,
) -> Result<Vec<ClipboardItem>, String> {
    Ok(clipboard_state.list_clipboard_history())
}

#[tauri::command]
pub fn get_current_clipboard_item(
    clipboard_state: State<'_, ClipboardState>,
) -> Result<Option<ClipboardItem>, String> {
    Ok(clipboard_state.get_current_clip())
}

#[tauri::command]
pub fn delete_item(
    clipboard_state: State<'_, ClipboardState>,
    item_id: String,
) -> Result<bool, String> {
    Ok(clipboard_state.remove_item(&item_id))
}

#[tauri::command]
pub fn clear_all(
    clipboard_state: State<'_, ClipboardState>,
) -> Result<(), String> {
    clipboard_state.delete_all();
    Ok(())
}

#[tauri::command]
pub fn select_item_to_clipboard(
    content: String,
) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(content).map_err(|e| e.to_string())?;
    Ok(())
}
