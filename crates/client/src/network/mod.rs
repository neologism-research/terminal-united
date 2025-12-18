// src/network/mod.rs
//
// Network module for WebSocket communication with the game server.

pub mod client;

pub use client::NetworkClient;

// Re-export shared types
pub use terminal_united_shared::Player;

/// Type alias for backward compatibility
pub type RemotePlayer = Player;
