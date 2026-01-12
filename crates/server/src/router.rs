use crate::AppState;
use crate::modes;
use terminal_united_shared::{ClientMessage, PlayerMode, ServerMessage};
use tracing::warn;

pub async fn route_packet(state: &AppState, session_id: &str, msg: ClientMessage) {
    let mode = {
        if let Some(player) = state.room.get_player(session_id) {
            player.mode.clone()
        } else {
            warn!("Session {} not found", session_id);
            return;
        }
    };

    match (mode, msg) {
        // Global Commands
        (_, ClientMessage::Chat { message }) => {
            if let Some((x, y)) = state.room.get_player_position(session_id) {
                if let Some(player) = state.room.get_player(session_id) {
                    state.room.broadcast(ServerMessage::ChatMessage {
                        username: player.username.clone(),
                        message,
                        x,
                        y,
                    });
                }
            }
        }
        (_, ClientMessage::Join { .. }) => {
            warn!(
                "Received Join message for already joined player {}",
                session_id
            );
        }
        (_, ClientMessage::Leave) => {
            // Handled in main loop usually
        }

        // World Mode
        (PlayerMode::Roaming { room_id }, ClientMessage::World(cmd)) => {
            modes::world::handle(cmd, &room_id, state, session_id).await;
        }

        // Poker Mode
        (PlayerMode::PlayingPoker { table_id }, ClientMessage::Poker(cmd)) => {
            modes::poker::handle(cmd, &table_id, state, session_id).await;
        }

        // Invalid / Hacker Trap
        (mode, cmd) => {
            warn!("Invalid command {:?} for mode {:?}", cmd, mode);
        }
    }
}
