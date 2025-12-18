//! Shared types for Terminal United client and server
//!
//! This crate contains all message types and data structures that are
//! shared between the client and server for network communication.

use serde::{Deserialize, Serialize};

/// Messages sent from client to server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ClientMessage {
    /// Join a room with a username
    #[serde(rename_all = "camelCase")]
    Join { username: String, room: String },

    /// Request to move the player
    #[serde(rename_all = "camelCase")]
    Move { dx: i32, dy: i32 },

    /// Leave the current room
    Leave,
}

/// Messages sent from server to client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ServerMessage {
    /// Initial state when joining a room
    #[serde(rename_all = "camelCase")]
    Init {
        session_id: String,
        players: Vec<Player>,
    },

    /// A new player joined the room
    #[serde(rename_all = "camelCase")]
    PlayerJoined { player: Player },

    /// A player moved
    #[serde(rename_all = "camelCase")]
    PlayerMoved { session_id: String, x: i32, y: i32 },

    /// A player left the room
    #[serde(rename_all = "camelCase")]
    PlayerLeft { session_id: String },

    /// Error message
    #[serde(rename_all = "camelCase")]
    Error { message: String },
}

/// Represents a player in the game
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    /// Unique session ID for this player
    pub session_id: String,
    /// Display name
    pub username: String,
    /// X position on the map
    pub x: i32,
    /// Y position on the map
    pub y: i32,
}

/// Information about an available room
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInfo {
    /// Room identifier
    pub name: String,
    /// Display name
    pub display_name: String,
    /// Number of players currently in the room
    pub player_count: usize,
}
