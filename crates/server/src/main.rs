//! Terminal United Game Server
//!
//! A simple WebSocket server for the Terminal United multiplayer game.
//! Handles room management and player synchronization.

mod room;

use axum::{
    Router,
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::get,
};
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

use room::Room;
use terminal_united_shared::{ClientMessage, Player, ServerMessage};

const SERVER_VERSION: &str = "0.1.0";

/// Application state shared across all connections
#[derive(Clone)]
struct AppState {
    /// All active rooms
    rooms: Arc<DashMap<String, Room>>,
}

impl AppState {
    fn new() -> Self {
        let rooms = Arc::new(DashMap::new());

        // Pre-create some default rooms
        for name in ["world", "arena", "dungeon", "tavern"] {
            rooms.insert(name.to_string(), Room::new(name));
        }

        Self { rooms }
    }

    fn get_or_create_room(&self, name: &str) -> Room {
        self.rooms
            .entry(name.to_string())
            .or_insert_with(|| Room::new(name))
            .clone()
    }
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("terminal_united_server=info".parse().unwrap()),
        )
        .init();

    let state = AppState::new();

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/version", get(version_handler))
        .with_state(state);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("🎮 Terminal United Server listening on {}", addr);
    info!("   Connect via ws://localhost:3000/ws");

    axum::serve(listener, app).await.unwrap();
}

/// Handle WebSocket upgrade requests
async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn version_handler() -> impl IntoResponse {
    axum::Json(serde_json::json!({ "version": SERVER_VERSION }))
}

/// Handle an individual WebSocket connection
async fn handle_socket(socket: WebSocket, state: AppState) {
    let session_id = Uuid::new_v4().to_string();
    let (mut sender, mut receiver) = socket.split();

    // Wait for the Join message first
    let (username, room_name) = match wait_for_join(&mut receiver).await {
        Some(join_info) => join_info,
        None => {
            warn!("Connection closed before joining");
            return;
        }
    };

    info!("{} ({}) joining room '{}'", username, session_id, room_name);

    // Get or create the room
    let room = state.get_or_create_room(&room_name);

    // Create the player
    let player = Player {
        session_id: session_id.clone(),
        username: username.clone(),
        x: 5,
        y: 5,
    };

    // Add player to room and get current state
    let current_players = room.add_player(player.clone());

    // Subscribe to room broadcasts
    let mut room_rx = room.subscribe();

    // Send Init message with current players
    let init_msg = ServerMessage::Init {
        session_id: session_id.clone(),
        players: current_players,
    };
    if sender
        .send(Message::Text(
            serde_json::to_string(&init_msg).unwrap().into(),
        ))
        .await
        .is_err()
    {
        room.remove_player(&session_id);
        return;
    }

    // Broadcast that this player joined
    room.broadcast(ServerMessage::PlayerJoined {
        player: player.clone(),
    });

    // Clone what we need for the tasks
    let room_for_recv = room.clone();
    let session_id_for_recv = session_id.clone();
    let username_for_recv = username.clone();

    // Spawn task to forward room broadcasts to this client
    let mut send_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));
        loop {
            tokio::select! {
                Ok(msg) = room_rx.recv() => {
                    let text = serde_json::to_string(&msg).unwrap();
                    if sender.send(Message::Text(text.into())).await.is_err() {
                        break;
                    }
                }
                _ = interval.tick() => {
                    if sender.send(Message::Ping(vec![])).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    // Handle incoming messages from this client
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    match client_msg {
                        ClientMessage::Move { dx, dy } => {
                            if let Some(new_pos) =
                                room_for_recv.move_player(&session_id_for_recv, dx, dy)
                            {
                                room_for_recv.broadcast(ServerMessage::PlayerMoved {
                                    session_id: session_id_for_recv.clone(),
                                    x: new_pos.0,
                                    y: new_pos.1,
                                });
                            }
                        }
                        ClientMessage::Chat { message } => {
                            // Get sender's current position
                            let (x, y) = room_for_recv
                                .get_player_position(&session_id_for_recv)
                                .unwrap_or((0, 0));

                            // Broadcast chat message to all players
                            room_for_recv.broadcast(ServerMessage::ChatMessage {
                                username: username_for_recv.clone(),
                                message,
                                x,
                                y,
                            });
                        }
                        ClientMessage::Leave => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    // Clean up
    info!("{} ({}) left room '{}'", username, session_id, room_name);
    room.remove_player(&session_id);
    room.broadcast(ServerMessage::PlayerLeft {
        session_id: session_id.clone(),
    });
}

/// Wait for the initial Join message from a client
async fn wait_for_join(
    receiver: &mut futures_util::stream::SplitStream<WebSocket>,
) -> Option<(String, String)> {
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            if let Ok(ClientMessage::Join { username, room }) = serde_json::from_str(&text) {
                return Some((username, room));
            }
        }
    }
    None
}
