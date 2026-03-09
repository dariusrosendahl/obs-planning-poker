use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

use crate::state::AppState;
use crate::types::{ClientMessage, ServerMessage};

pub struct ServerState {
    pub app_state: Arc<AppState>,
    pub tx: broadcast::Sender<String>,
}

pub async fn start_server(app_state: Arc<AppState>, tx: broadcast::Sender<String>, card_dir: String) {
    let server_state = Arc::new(ServerState {
        app_state,
        tx,
    });

    let app = Router::new()
        .route("/", get(ws_handler))
        .nest_service("/card", ServeDir::new(card_dir))
        .with_state(server_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("Card overlay server running on http://localhost:3000/card");

    axum::serve(listener, app).await.expect("Server error");
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<ServerState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<ServerState>) {
    let (mut sender, mut receiver) = socket.split();

    // Send current state on connect
    let connected = ServerMessage::Connected {
        state: state.app_state.get(),
    };
    if let Ok(json) = serde_json::to_string(&connected) {
        let _ = sender.send(Message::Text(json.into())).await;
    }

    // Subscribe to broadcast updates
    let mut rx = state.tx.subscribe();

    // Spawn task to forward broadcasts to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages from this client
    let app_state = state.app_state.clone();
    let tx = state.tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    let new_state = match client_msg {
                        ClientMessage::SetCard { value } => app_state.set_card(&value),
                        ClientMessage::NextCard => app_state.next_card(),
                        ClientMessage::PrevCard => app_state.prev_card(),
                        ClientMessage::ToggleReveal => app_state.toggle_reveal(),
                        ClientMessage::HideCard => app_state.hide_card(),
                    };
                    let update = ServerMessage::CardUpdate { state: new_state };
                    if let Ok(json) = serde_json::to_string(&update) {
                        let _ = tx.send(json);
                    }
                }
            }
        }
    });

    // If either task finishes, abort the other
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
}
