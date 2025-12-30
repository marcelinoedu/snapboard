use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{App};

use crate::commands::overlay::toggle_overlay;

pub fn init_tray(app: &App) -> tauri::Result<()> {
    let handle = app.handle().clone();
    let icon = app.default_window_icon().unwrap().clone();

    TrayIconBuilder::new()
        .icon(icon)
        .on_tray_icon_event(move |_tray, event| {
            if let TrayIconEvent::Click {
                button,
                button_state,
                ..
            } = event
            {
                if button == MouseButton::Left && button_state == MouseButtonState::Up {
                    let app = handle.clone();
                    let _ = toggle_overlay(app);
                }
            }
        })
        .build(app)?;

    Ok(())
}
