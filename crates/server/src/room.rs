//! Room management for the game server

use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

use terminal_united_shared::{Player, ServerMessage};

/// A game room that holds players
#[derive(Clone)]
pub struct Room {
    /// Room name/identifier
    #[allow(dead_code)]
    pub name: String,
    /// All players in this room
    players: Arc<DashMap<String, Player>>,
    /// Broadcast channel for room events
    tx: broadcast::Sender<ServerMessage>,
}

impl Room {
    /// Create a new room
    pub fn new(name: &str) -> Self {
        let (tx, _) = broadcast::channel(256);
        Self {
            name: name.to_string(),
            players: Arc::new(DashMap::new()),
            tx,
        }
    }

    /// Add a player to the room, returns list of existing players
    pub fn add_player(&self, player: Player) -> Vec<Player> {
        let existing: Vec<Player> = self.players.iter().map(|r| r.value().clone()).collect();
        self.players.insert(player.session_id.clone(), player);
        existing
    }

    /// Remove a player from the room
    pub fn remove_player(&self, session_id: &str) {
        self.players.remove(session_id);
    }

    /// Move a player, returns new position if successful
    pub fn move_player(&self, session_id: &str, dx: i32, dy: i32) -> Option<(i32, i32)> {
        if let Some(mut player) = self.players.get_mut(session_id) {
            // Simple bounds checking (adjust based on your map size)
            let new_x = (player.x + dx).clamp(0, 399);
            let new_y = (player.y + dy).clamp(0, 399);
            player.x = new_x;
            player.y = new_y;
            Some((new_x, new_y))
        } else {
            None
        }
    }

    /// Subscribe to room broadcasts
    pub fn subscribe(&self) -> broadcast::Receiver<ServerMessage> {
        self.tx.subscribe()
    }

    /// Broadcast a message to all players in the room
    pub fn broadcast(&self, msg: ServerMessage) {
        // Ignore errors (no receivers is fine)
        let _ = self.tx.send(msg);
    }

    /// Get number of players in the room
    #[allow(dead_code)]
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    /// Get a player's current position
    pub fn get_player_position(&self, session_id: &str) -> Option<(i32, i32)> {
        self.players.get(session_id).map(|p| (p.x, p.y))
    }
}
