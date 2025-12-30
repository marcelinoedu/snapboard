pub mod app;
pub mod items;
pub mod overlay;

pub fn register() -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync {
    tauri::generate_handler![
        items::list_clipboard_history,
        items::get_current_clipboard_item,
        items::delete_item,
        items::clear_all,
        items::select_item_to_clipboard,
        overlay::toggle_overlay,
        overlay::finalize_overlay_close,
        app::stop_app,
    ]
}
