use tauri::{
    AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent,
};

use crate::commands;
use crate::config::windows::overlay::{OVERLAY_LABEL};
use crate::state::windows::{WindowState, OverlayWindowState};
use std::thread;
use std::time::Duration;

pub fn create_overlay(app: &AppHandle) -> tauri::Result<()> {
    if let Some(_window) = app.get_webview_window(OVERLAY_LABEL) {
        return Ok(());
    }

    let window = WebviewWindowBuilder::new(
        app,
        OVERLAY_LABEL,
        WebviewUrl::App("/".into()),
    )
    .decorations(false)
    .transparent(true)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .shadow(false)
    .visible(false)
    .build()?;

    attach_focus_listener(window.clone(), app.clone());
    Ok(())
}

pub fn apply_layout(
    window: &WebviewWindow,
    _state: OverlayWindowState,
) -> tauri::Result<()> {
    let monitor = window
        .current_monitor()?
        .or(window.primary_monitor()?)
        .expect("No monitor available");

    let monitor_pos = monitor.position();
    let monitor_size = monitor.size();

    const OVERLAY_HEIGHT: u32 = 550;
    const SIDE_INSET: u32 = 0;
    const BOTTOM_INSET: u32 = 100;

    let width = monitor_size.width.saturating_sub(SIDE_INSET * 2);

    window.set_size(tauri::PhysicalSize {
        width,
        height: OVERLAY_HEIGHT,
    })?;

    window.set_position(tauri::PhysicalPosition {
        x: monitor_pos.x + SIDE_INSET as i32,
        y: monitor_pos.y
            + monitor_size.height as i32
            - OVERLAY_HEIGHT as i32
            - BOTTOM_INSET as i32,
    })?;

    Ok(())
}



pub fn attach_focus_listener(
    window: WebviewWindow,
    app: AppHandle,
) {
    window.clone().on_window_event(move |event| {
        let state = app.state::<WindowState>();

        match event {
            WindowEvent::Focused(true) => {
                state.update(|s| s.had_focus = true);
            }

            WindowEvent::Focused(false) => {
                let app = app.clone();

                // 🔥 Debounce para evitar glitch do WebView
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(80));

                    let state = app.state::<WindowState>();
                    let snapshot = state.snapshot();

                    if snapshot.visible && snapshot.had_focus {
                        if let Some(window) =
                            app.get_webview_window(OVERLAY_LABEL)
                        {
                            let _ = window.hide();
                        }

                        state.update(|s| {
                            s.visible = false;
                            s.had_focus = false;
                        });
                    }
                });
            }

            _ => {}
        }
    });
}

