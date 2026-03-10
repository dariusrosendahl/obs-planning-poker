// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod server;
mod state;
mod types;

use std::path::PathBuf;
use std::sync::Arc;

use commands::SharedState;
use state::AppState;
use tauri::menu::{AboutMetadata, MenuBuilder, SubmenuBuilder};
use tauri::Manager;
use tokio::sync::broadcast;

fn main() {
    let app_state = Arc::new(AppState::new());
    let (tx, _rx) = broadcast::channel::<String>(64);

    tauri::Builder::default()
        .menu(|handle| {
            let about_metadata = AboutMetadata {
                name: Some("Planning Poker".to_string()),
                copyright: Some("\u{00A9} 2026 Darius Rosendahl (DoorDarius)".to_string()),
                credits: Some("github.com/dariusrosendahl\nwww.doordarius.nl".to_string()),
                ..Default::default()
            };
            let app_submenu = SubmenuBuilder::new(handle, "Planning Poker")
                .about(Some(about_metadata))
                .separator()
                .services()
                .separator()
                .hide()
                .hide_others()
                .show_all()
                .separator()
                .quit()
                .build()?;
            let edit_submenu = SubmenuBuilder::new(handle, "Edit")
                .undo()
                .redo()
                .separator()
                .cut()
                .copy()
                .paste()
                .select_all()
                .build()?;
            let window_submenu = SubmenuBuilder::new(handle, "Window")
                .minimize()
                .separator()
                .close_window()
                .build()?;
            MenuBuilder::new(handle)
                .item(&app_submenu)
                .item(&edit_submenu)
                .item(&window_submenu)
                .build()
        })
        .setup(move |app| {
            // Load config from app config directory
            let config_dir = app.path().app_config_dir()?;
            let cfg = config::load(&config_dir);
            let port = cfg.port;

            let shared = SharedState {
                app_state: app_state.clone(),
                tx: tx.clone(),
                port,
                config_dir,
            };
            app.manage(shared);

            // Resolve card directory path
            let card_dir: PathBuf = app
                .path()
                .resource_dir()
                .map(|p| p.join("public/card"))
                .unwrap_or_else(|_| {
                    std::env::current_dir()
                        .unwrap()
                        .join("../public/card")
                });

            let card_dir_str = card_dir.to_string_lossy().to_string();

            // Spawn axum server on a background thread
            let server_app_state = app_state.clone();
            let server_tx = tx.clone();
            tauri::async_runtime::spawn(async move {
                server::start_server(server_app_state, server_tx, card_dir_str, port).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_state,
            commands::get_port,
            commands::set_port,
            commands::set_card,
            commands::next_card,
            commands::prev_card,
            commands::toggle_reveal,
            commands::hide_card,
            commands::open_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
