mod modes;
mod room;
mod router;

use axum::{
    Router,
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::get,
};
use futures_util::{SinkExt, StreamExt};
use tracing::{info, warn};
use uuid::Uuid;

use room::Room;
use terminal_united_shared::{
    ClientMessage, DEFAULT_SPAWN_X, DEFAULT_SPAWN_Y, Player, PlayerMode, ServerMessage,
    constants::VERSION,
};

#[derive(Clone)]
pub struct AppState {
    room: Room,
}

impl AppState {
    fn new() -> Self {
        Self {
            room: Room::new("world"),
        }
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

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!(
        "🎮 Terminal United Server v{} listening on {}",
        VERSION, addr
    );

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

    let username = match wait_for_join(&mut receiver).await {
        Some(username) => username,
        None => {
            warn!("Connection closed before joining");
            return;
        }
    };

    info!("{} ({}) joining world", username, session_id);

    let room = state.room.clone();

    let player = Player {
        session_id: session_id.clone(),
        username: username.clone(),
        x: DEFAULT_SPAWN_X,
        y: DEFAULT_SPAWN_Y,
        mode: PlayerMode::Roaming {
            room_id: "world".to_string(),
        },
    };

    let current_players = room.add_player(player.clone());
    let mut room_rx = room.subscribe();

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

    room.broadcast(ServerMessage::PlayerJoined {
        player: player.clone(),
    });

    let session_id_for_recv = session_id.clone();

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

    let state_for_recv = state.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    router::route_packet(&state_for_recv, &session_id_for_recv, client_msg).await;
                }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    info!("{} ({}) left world", username, session_id);
    room.remove_player(&session_id);
    room.broadcast(ServerMessage::PlayerLeft {
        session_id: session_id.clone(),
    });
}

async fn wait_for_join(
    receiver: &mut futures_util::stream::SplitStream<WebSocket>,
) -> Option<String> {
    while let Some(Ok(Message::Text(text))) = receiver.next().await {
        if let Ok(ClientMessage::Join { username }) = serde_json::from_str(&text) {
            return Some(username);
        }
    }
    None
}
