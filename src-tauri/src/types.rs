use serde::{Deserialize, Serialize};

pub const CARD_VALUES: &[&str] = &[
    "0", "1", "2", "3", "5", "8", "13", "21", "?", "\u{2615}", "\u{221E}",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardState {
    pub value: String,
    pub revealed: bool,
    pub index: usize,
}

// Messages from client -> server (WebSocket only, panel uses IPC)
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "SET_CARD")]
    SetCard { value: String },
    #[serde(rename = "NEXT_CARD")]
    NextCard,
    #[serde(rename = "PREV_CARD")]
    PrevCard,
    #[serde(rename = "TOGGLE_REVEAL")]
    ToggleReveal,
    #[serde(rename = "HIDE_CARD")]
    HideCard,
}

// Messages from server -> client (WebSocket)
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "CONNECTED")]
    Connected { state: CardState },
    #[serde(rename = "CARD_UPDATE")]
    CardUpdate { state: CardState },
}
