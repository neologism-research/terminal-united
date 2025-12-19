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
use terminal_united_shared::{ClientMessage, Player, ServerMessage, VERSION, DEFAULT_SPAWN_X, DEFAULT_SPAWN_Y};

#[derive(Clone)]
struct AppState {
    rooms: Arc<DashMap<String, Room>>,
}

impl AppState {
    fn new() -> Self {
        let rooms = Arc::new(DashMap::new());

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

    info!("🎮 Terminal United Server v{} listening on {}", VERSION, addr);

    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn version_handler() -> impl IntoResponse {
    axum::Json(serde_json::json!({ "version": VERSION }))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let session_id = Uuid::new_v4().to_string();
    let (mut sender, mut receiver) = socket.split();

    let (username, room_name) = match wait_for_join(&mut receiver).await {
        Some(join_info) => join_info,
        None => {
            warn!("Connection closed before joining");
            return;
        }
    };

    info!("{} ({}) joining room '{}'", username, session_id, room_name);

    let room = state.get_or_create_room(&room_name);

    let player = Player {
        session_id: session_id.clone(),
        username: username.clone(),
        x: DEFAULT_SPAWN_X,
        y: DEFAULT_SPAWN_Y,
    };

    let current_players = room.add_player(player.clone());
    let mut room_rx = room.subscribe();

    let init_msg = ServerMessage::Init {
        session_id: session_id.clone(),
        players: current_players,
    };

    if sender
        .send(Message::Text(serde_json::to_string(&init_msg).unwrap().into()))
        .await
        .is_err()
    {
        room.remove_player(&session_id);
        return;
    }

    room.broadcast(ServerMessage::PlayerJoined { player: player.clone() });

    let room_for_recv = room.clone();
    let session_id_for_recv = session_id.clone();
    let username_for_recv = username.clone();

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

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    match client_msg {
                        ClientMessage::Move { dx, dy } => {
                            if let Some((x, y)) = room_for_recv.move_player(&session_id_for_recv, dx, dy) {
                                room_for_recv.broadcast(ServerMessage::PlayerMoved {
                                    session_id: session_id_for_recv.clone(),
                                    x,
                                    y,
                                });
                            }
                        }
                        ClientMessage::Chat { message } => {
                            let (x, y) = room_for_recv
                                .get_player_position(&session_id_for_recv)
                                .unwrap_or((0, 0));

                            room_for_recv.broadcast(ServerMessage::ChatMessage {
                                username: username_for_recv.clone(),
                                message,
                                x,
                                y,
                            });
                        }
                        ClientMessage::Leave => break,
                        _ => {}
                    }
                }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    info!("{} ({}) left room '{}'", username, session_id, room_name);
    room.remove_player(&session_id);
    room.broadcast(ServerMessage::PlayerLeft { session_id: session_id.clone() });
}

async fn wait_for_join(
    receiver: &mut futures_util::stream::SplitStream<WebSocket>,
) -> Option<(String, String)> {
    while let Some(Ok(Message::Text(text))) = receiver.next().await {
        if let Ok(ClientMessage::Join { username, room }) = serde_json::from_str(&text) {
            return Some((username, room));
        }
    }
    None
}
