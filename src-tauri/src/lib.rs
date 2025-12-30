mod app;
mod commands;
mod plugins;
mod state;
mod models;
mod tray;
mod windows;
mod listeners;
mod config;
mod utils;

use tauri::Builder;

pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(plugins::autostart::init())
        .invoke_handler(commands::register())
        .setup(|app| Ok(app::setup::init_app(app)?))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
