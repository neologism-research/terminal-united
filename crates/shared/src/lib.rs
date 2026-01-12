//! Shared types and constants for Terminal United

use serde::{Deserialize, Serialize};
pub mod constants;

// =============================================================================
// GAME CONSTANTS - Centralized configuration
// =============================================================================

pub const PROXIMITY_DISTANCE: i32 = 20;
pub const MAX_USERNAME_LENGTH: usize = 20;
pub const MAX_CHAT_LENGTH: usize = 200;
pub const DEFAULT_SPAWN_X: i32 = 5;
pub const DEFAULT_SPAWN_Y: i32 = 5;
pub const MAP_WIDTH: i32 = 400;
pub const MAP_HEIGHT: i32 = 400;

// =============================================================================
// CLIENT -> SERVER MESSAGES
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum PlayerMode {
    Lobby,
    Roaming { room_id: String },
    PlayingPoker { table_id: String },
    InCombat { battle_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum WorldCommand {
    Move { dx: i32, dy: i32 },
    Interact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PokerCommand {
    Bet { amount: u32 },
    Fold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum ClientMessage {
    Join { username: String },
    Chat { message: String },
    Leave,
    World(WorldCommand),
    Poker(PokerCommand),
}

// =============================================================================
// SERVER -> CLIENT MESSAGES
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ServerMessage {
    #[serde(rename_all = "camelCase")]
    Init {
        session_id: String,
        players: Vec<Player>,
    },

    #[serde(rename_all = "camelCase")]
    PlayerJoined { player: Player },

    #[serde(rename_all = "camelCase")]
    PlayerMoved { session_id: String, x: i32, y: i32 },

    #[serde(rename_all = "camelCase")]
    PlayerLeft { session_id: String },

    #[serde(rename_all = "camelCase")]
    ChatMessage {
        username: String,
        message: String,
        x: i32,
        y: i32,
    },

    #[serde(rename_all = "camelCase")]
    Error { message: String },
}

// =============================================================================
// DATA TYPES
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub session_id: String,
    pub username: String,
    pub x: i32,
    pub y: i32,
    pub mode: PlayerMode,
}

#[derive(Debug, Clone)]
pub struct ChatEntry {
    pub username: String,
    pub message: String,
    pub is_proximity: bool,
    pub is_system: bool,
}
