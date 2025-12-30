use std::{thread, time::Duration};
use arboard::Clipboard;
use uuid::Uuid;
use tauri::Emitter;
use crate::utils::{now_ts, classify_clipboard};
use crate::{
    models::clipboard_item::ClipboardItem,
    state::clipboard_state::ClipboardState,
    state::app_state::AppState,
};

pub fn init_clipboard_listener(app: tauri::AppHandle, clipboard_state: ClipboardState, app_state: AppState,) {
    thread::spawn(move || {
        
        let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
        
        let expires: i64 = 60 * 60 * 24;
        
        while app_state.is_running() {
            if let Ok(text) = clipboard.get_text() {
                if !text.trim().is_empty() {
                    let now = now_ts();
                    let kind = classify_clipboard(&text);
                    let item = ClipboardItem {
                        id: Uuid::new_v4().to_string(),
                        kind: kind,
                        content: text,
                        file_path: None,
                        created_at: now,
                        expires_at: now + expires,
                    };

                    let app = app.clone();
                    app.emit("clipboard:change", &item).unwrap();

                    if  clipboard_state.push_item(item.clone()) {
                        app.emit("clipboard:new", item).unwrap();
                    }

                }
            }
            thread::sleep(Duration::from_millis(500));
        }
    });
}
