use tauri::{AppHandle, Manager, Emitter};

use crate::config::windows::overlay::OVERLAY_LABEL;
use crate::state::windows::WindowState;
use crate::windows::overlay::{apply_layout, create_overlay};

#[tauri::command]
pub fn toggle_overlay(app: AppHandle) -> tauri::Result<()> {
    create_overlay(&app)?;
    let state = app.state::<WindowState>();

    let window = app
        .get_webview_window(OVERLAY_LABEL)
        .expect("overlay window not found");

    let snapshot = state.snapshot();

    if snapshot.visible {
        window.emit("overlay:close", ())?;
        return Ok(());
    }
    apply_layout(&window, snapshot)?;
    window.show()?;
    let _ = window.set_focus();

    state.update(|s| s.visible = true);
    window.emit("overlay:open", ())?;

    Ok(())
}

#[tauri::command]
pub fn finalize_overlay_close(app: AppHandle) {
    let state = app.state::<WindowState>();

    if let Some(window) = app.get_webview_window(OVERLAY_LABEL) {
        let _ = window.hide();
    }

    state.update(|s| {
        s.visible = false;
        s.had_focus = false;
    });
}


