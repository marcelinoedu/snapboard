use tauri::{ActivationPolicy, Manager};

use crate::listeners::clipboard::init_clipboard_listener;
use crate::listeners::hotkeys::init_hotkeys;
use crate::state::app_state::AppState;
use crate::state::clipboard_state::ClipboardState;
use crate::state::windows::WindowState;
use crate::tray::tray::init_tray;
use crate::windows::overlay::create_overlay;

pub fn init_app(app: &mut tauri::App) -> tauri::Result<()> {
    #[cfg(target_os = "macos")]
    {
        app.set_activation_policy(ActivationPolicy::Accessory);
    }

    
    app.manage(AppState::new());
    app.manage(ClipboardState::new());
    app.manage(WindowState::new());

    
    create_overlay(&app.handle())?;
    
    init_hotkeys(app)?;
    init_tray(app)?;

    
    init_clipboard_listener(
        app.handle().clone(),
        app.state::<ClipboardState>().inner().clone(),
        app.state::<AppState>().inner().clone(),
    );

    Ok(())
}
