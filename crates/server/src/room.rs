use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

use terminal_united_shared::{Player, ServerMessage, MAP_HEIGHT, MAP_WIDTH};

#[derive(Clone)]
pub struct Room {
    #[allow(dead_code)]
    pub name: String,
    players: Arc<DashMap<String, Player>>,
    tx: broadcast::Sender<ServerMessage>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        let (tx, _) = broadcast::channel(256);
        Self {
            name: name.to_string(),
            players: Arc::new(DashMap::new()),
            tx,
        }
    }

    pub fn add_player(&self, player: Player) -> Vec<Player> {
        let existing: Vec<Player> = self.players.iter().map(|r| r.value().clone()).collect();
        self.players.insert(player.session_id.clone(), player);
        existing
    }

    pub fn remove_player(&self, session_id: &str) {
        self.players.remove(session_id);
    }

    pub fn move_player(&self, session_id: &str, dx: i32, dy: i32) -> Option<(i32, i32)> {
        self.players.get_mut(session_id).map(|mut player| {
            let new_x = (player.x + dx).clamp(0, MAP_WIDTH as i32 - 1);
            let new_y = (player.y + dy).clamp(0, MAP_HEIGHT as i32 - 1);
            player.x = new_x;
            player.y = new_y;
            (new_x, new_y)
        })
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ServerMessage> {
        self.tx.subscribe()
    }

    pub fn broadcast(&self, msg: ServerMessage) {
        let _ = self.tx.send(msg);
    }

    #[allow(dead_code)]
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    pub fn get_player_position(&self, session_id: &str) -> Option<(i32, i32)> {
        self.players.get(session_id).map(|p| (p.x, p.y))
    }
}
