// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod server;
mod state;
mod types;

use std::sync::Arc;
use std::path::PathBuf;

use commands::SharedState;
use state::AppState;
use tauri::Manager;
use tokio::sync::broadcast;

fn get_port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(7777)
}

fn main() {
    let port = get_port();
    let app_state = Arc::new(AppState::new());
    let (tx, _rx) = broadcast::channel::<String>(64);

    let shared = SharedState {
        app_state: app_state.clone(),
        tx: tx.clone(),
        port,
    };

    tauri::Builder::default()
        .manage(shared)
        .setup(move |app| {
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
            commands::set_card,
            commands::next_card,
            commands::prev_card,
            commands::toggle_reveal,
            commands::hide_card,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
