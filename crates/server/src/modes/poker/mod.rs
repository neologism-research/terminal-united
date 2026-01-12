use crate::AppState;
use terminal_united_shared::PokerCommand;
use tracing::info;

pub async fn handle(cmd: PokerCommand, table_id: &str, _state: &AppState, session_id: &str) {
    match cmd {
        PokerCommand::Bet { amount } => {
            info!(
                "Player {} betting {} at table {}",
                session_id, amount, table_id
            );
        }
        PokerCommand::Fold => {
            info!("Player {} folding at table {}", session_id, table_id);
        }
    }
}
