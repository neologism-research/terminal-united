// src/network/client.rs
//
// WebSocket client for connecting to the Terminal United game server.
// Uses a simple JSON protocol over WebSocket.

use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use terminal_united_shared::{ClientMessage, Player, ServerMessage};

/// Thread-safe container for remote players
pub type PlayersState = Arc<Mutex<HashMap<String, Player>>>;

/// Network client for communicating with the game server
pub struct NetworkClient {
    /// Channel to send messages to the server
    tx: mpsc::UnboundedSender<ClientMessage>,
    /// Shared state of all remote players (excluding self)
    pub players: PlayersState,
    /// Our session ID from the server
    pub session_id: Arc<Mutex<Option<String>>>,
}

impl NetworkClient {
    /// Connect to the game server
    pub async fn connect(
        server_url: &str,
        username: &str,
        room_name: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Connect to the WebSocket endpoint
        let ws_url = format!("{}/ws", server_url);
        let (ws_stream, _) = connect_async(&ws_url).await?;
        let (mut write, mut read) = ws_stream.split();

        // Channel for outgoing messages
        let (tx, mut rx) = mpsc::unbounded_channel::<ClientMessage>();

        // Shared state
        let players: PlayersState = Arc::new(Mutex::new(HashMap::new()));
        let session_id_state: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

        // Send the Join message
        let join_msg = ClientMessage::Join {
            username: username.to_string(),
            room: room_name.to_string(),
        };
        write
            .send(Message::Text(serde_json::to_string(&join_msg)?))
            .await?;

        // Clone for the receive task
        let players_clone = players.clone();
        let session_id_clone = session_id_state.clone();

        // Spawn task to handle incoming messages
        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(server_msg) = serde_json::from_str::<ServerMessage>(&text) {
                            match server_msg {
                                ServerMessage::Init { session_id, players: init_players } => {
                                    *session_id_clone.lock().await = Some(session_id.clone());
                                    // Add all players except ourselves
                                    let mut players = players_clone.lock().await;
                                    for player in init_players {
                                        if player.session_id != session_id {
                                            players.insert(player.session_id.clone(), player);
                                        }
                                    }
                                }
                                ServerMessage::PlayerJoined { player } => {
                                    let my_id = session_id_clone.lock().await.clone();
                                    if Some(&player.session_id) != my_id.as_ref() {
                                        players_clone.lock().await.insert(player.session_id.clone(), player);
                                    }
                                }
                                ServerMessage::PlayerMoved { session_id, x, y } => {
                                    if let Some(player) = players_clone.lock().await.get_mut(&session_id) {
                                        player.x = x;
                                        player.y = y;
                                    }
                                }
                                ServerMessage::PlayerLeft { session_id } => {
                                    players_clone.lock().await.remove(&session_id);
                                }
                                ServerMessage::Error { message } => {
                                    eprintln!("Server error: {}", message);
                                }
                            }
                        }
                    }
                    Ok(Message::Close(_)) => break,
                    Err(_) => break,
                    _ => {}
                }
            }
        });

        // Spawn task to handle outgoing messages
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Ok(json) = serde_json::to_string(&msg) {
                    if write.send(Message::Text(json)).await.is_err() {
                        break;
                    }
                }
            }
        });

        Ok(Self {
            tx,
            players,
            session_id: session_id_state,
        })
    }

    /// Send a move command to the server
    pub fn send_move(&self, dx: i32, dy: i32) {
        let _ = self.tx.send(ClientMessage::Move { dx, dy });
    }

    /// Get a snapshot of current remote players
    #[allow(dead_code)]
    pub async fn get_players(&self) -> HashMap<String, Player> {
        self.players.lock().await.clone()
    }

    /// Get our session ID (if connected)
    #[allow(dead_code)]
    pub async fn get_session_id(&self) -> Option<String> {
        self.session_id.lock().await.clone()
    }
}
