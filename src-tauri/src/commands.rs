use std::sync::Arc;

use tauri::State;
use tokio::sync::broadcast;

use crate::state::AppState;
use crate::types::{CardState, ServerMessage};

pub struct SharedState {
    pub app_state: Arc<AppState>,
    pub tx: broadcast::Sender<String>,
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
