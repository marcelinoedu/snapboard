use tauri::{plugin::TauriPlugin, Runtime};
use tauri_plugin_autostart::MacosLauncher;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None)
}
