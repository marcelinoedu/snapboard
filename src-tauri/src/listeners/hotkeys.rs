use global_hotkey::hotkey::{Code, Modifiers};
use tauri::{App};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::commands::overlay::toggle_overlay;

pub fn init_hotkeys(app: &App) -> tauri::Result<()> {
    let handle = app.handle().clone();

    let shortcut = Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::Space);

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state() == ShortcutState::Released {
                let app = handle.clone();
                let _ = toggle_overlay(app);
                
            }
        })
        .map_err(|e| tauri::Error::Anyhow(e.into()))?;

    Ok(())
}
