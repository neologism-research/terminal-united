use crate::AppState;
use terminal_united_shared::{ServerMessage, WorldCommand};
use tracing::info;

pub async fn handle(cmd: WorldCommand, room_id: &str, state: &AppState, session_id: &str) {
    let room = state.room.clone();

    match cmd {
        WorldCommand::Move { dx, dy } => {
            if let Some((x, y)) = room.move_player(session_id, dx, dy) {
                room.broadcast(ServerMessage::PlayerMoved {
                    session_id: session_id.to_string(),
                    x,
                    y,
                });
            }
        }
        WorldCommand::Interact => {
            info!("Player {} interacting in room {}", session_id, room_id);
        }
    }
}
