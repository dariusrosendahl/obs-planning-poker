use std::path::PathBuf;
use std::sync::Arc;

use tauri::State;
use tokio::sync::broadcast;

use crate::config::{self, AppConfig};
use crate::state::AppState;
use crate::types::{CardState, ServerMessage};

pub struct SharedState {
    pub app_state: Arc<AppState>,
    pub tx: broadcast::Sender<String>,
    pub port: u16,
    pub config_dir: PathBuf,
}

fn broadcast_update(shared: &SharedState, new_state: &CardState) {
    let update = ServerMessage::CardUpdate {
        state: new_state.clone(),
    };
    if let Ok(json) = serde_json::to_string(&update) {
        let _ = shared.tx.send(json);
    }
}

#[tauri::command]
pub fn get_state(shared: State<'_, SharedState>) -> CardState {
    shared.app_state.get()
}

#[tauri::command]
pub fn get_port(shared: State<'_, SharedState>) -> u16 {
    shared.port
}

#[tauri::command]
pub fn set_port(port: u16, shared: State<'_, SharedState>) -> Result<(), String> {
    let cfg = AppConfig { port };
    config::save(&shared.config_dir, &cfg)
}

#[tauri::command]
pub fn set_card(value: String, shared: State<'_, SharedState>) -> CardState {
    let new_state = shared.app_state.set_card(&value);
    broadcast_update(&shared, &new_state);
    new_state
}

#[tauri::command]
pub fn next_card(shared: State<'_, SharedState>) -> CardState {
    let new_state = shared.app_state.next_card();
    broadcast_update(&shared, &new_state);
    new_state
}

#[tauri::command]
pub fn prev_card(shared: State<'_, SharedState>) -> CardState {
    let new_state = shared.app_state.prev_card();
    broadcast_update(&shared, &new_state);
    new_state
}

#[tauri::command]
pub fn toggle_reveal(shared: State<'_, SharedState>) -> CardState {
    let new_state = shared.app_state.toggle_reveal();
    broadcast_update(&shared, &new_state);
    new_state
}

#[tauri::command]
pub fn hide_card(shared: State<'_, SharedState>) -> CardState {
    let new_state = shared.app_state.hide_card();
    broadcast_update(&shared, &new_state);
    new_state
}

#[tauri::command]
pub fn open_url(url: String) {
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(&url).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", "", &url])
            .spawn();
    }
}
