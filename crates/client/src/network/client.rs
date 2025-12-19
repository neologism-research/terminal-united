use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use terminal_united_shared::{
    ChatEntry, ClientMessage, Player, ServerMessage, DEFAULT_SPAWN_X, DEFAULT_SPAWN_Y,
    PROXIMITY_DISTANCE,
};

pub type PlayersState = Arc<Mutex<HashMap<String, Player>>>;
pub type ChatState = Arc<Mutex<Vec<ChatEntry>>>;

pub struct NetworkClient {
    tx: mpsc::UnboundedSender<ClientMessage>,
    pub players: PlayersState,
    #[allow(dead_code)]
    session_id: Arc<Mutex<Option<String>>>,
    pub chat_log: ChatState,
    pub local_pos: Arc<Mutex<(i32, i32)>>,
}

impl NetworkClient {
    pub async fn connect(
        server_url: &str,
        username: &str,
        room_name: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let ws_url = format!("{}/ws", server_url);
        let (ws_stream, _) = connect_async(&ws_url).await?;
        let (mut write, mut read) = ws_stream.split();

        let (tx, mut rx) = mpsc::unbounded_channel::<ClientMessage>();

        let players: PlayersState = Arc::new(Mutex::new(HashMap::new()));
        let session_id_state: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
        let chat_log: ChatState = Arc::new(Mutex::new(Vec::new()));
        let local_pos: Arc<Mutex<(i32, i32)>> =
            Arc::new(Mutex::new((DEFAULT_SPAWN_X, DEFAULT_SPAWN_Y)));

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
        let chat_log_clone = chat_log.clone();
        let local_pos_clone = local_pos.clone();

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
                                    // Add system message
                                    chat_log_clone.lock().await.push(ChatEntry {
                                        username: "System".to_string(),
                                        message: "Connected to server!".to_string(),
                                        is_proximity: false,
                                        is_system: true,
                                    });
                                }
                                ServerMessage::PlayerJoined { player } => {
                                    let my_id = session_id_clone.lock().await.clone();
                                    if Some(&player.session_id) != my_id.as_ref() {
                                        let name = player.username.clone();
                                        players_clone.lock().await.insert(player.session_id.clone(), player);
                                        // Add join message
                                        chat_log_clone.lock().await.push(ChatEntry {
                                            username: "System".to_string(),
                                            message: format!("{} joined", name),
                                            is_proximity: false,
                                            is_system: true,
                                        });
                                    }
                                }
                                ServerMessage::PlayerMoved { session_id, x, y } => {
                                    if let Some(player) = players_clone.lock().await.get_mut(&session_id) {
                                        player.x = x;
                                        player.y = y;
                                    }
                                }
                                ServerMessage::PlayerLeft { session_id } => {
                                    let mut players = players_clone.lock().await;
                                    if let Some(player) = players.remove(&session_id) {
                                        chat_log_clone.lock().await.push(ChatEntry {
                                            username: "System".to_string(),
                                            message: format!("{} left", player.username),
                                            is_proximity: false,
                                            is_system: true,
                                        });
                                    }
                                }
                                ServerMessage::ChatMessage { username, message, x, y, .. } => {
                                    let (my_x, my_y) = *local_pos_clone.lock().await;
                                    let distance = (x - my_x).abs() + (y - my_y).abs();
                                    let is_proximity = distance <= PROXIMITY_DISTANCE;
                                    
                                    chat_log_clone.lock().await.push(ChatEntry {
                                        username,
                                        message,
                                        is_proximity,
                                        is_system: false,
                                    });
                                }
                                ServerMessage::Error { message } => {
                                    chat_log_clone.lock().await.push(ChatEntry {
                                        username: "Error".to_string(),
                                        message,
                                        is_proximity: false,
                                        is_system: true,
                                    });
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
            chat_log,
            local_pos,
        })
    }

    pub fn send_move(&self, dx: i32, dy: i32) {
        let _ = self.tx.send(ClientMessage::Move { dx, dy });
    }

    pub fn send_chat(&self, message: String) {
        let _ = self.tx.send(ClientMessage::Chat { message });
    }

    pub fn update_local_pos(&self, x: i32, y: i32, runtime: &tokio::runtime::Runtime) {
        let local_pos = self.local_pos.clone();
        runtime.block_on(async {
            *local_pos.lock().await = (x, y);
        });
    }
}
