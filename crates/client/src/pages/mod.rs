// src/pages/mod.rs
//
// This module defines the Page trait and re-exports all page implementations.
// Each page is self-contained with its own state, rendering, and input handling.

pub mod hints;
pub mod login;
pub mod room_select;
pub mod world;

pub use login::LoginPage;
pub use room_select::RoomSelectPage;
pub use world::WorldPage;

use crossterm::event::KeyCode;
use ratatui::Frame;
use std::collections::HashMap;

use crate::network::{ChatEntry, RemotePlayer};

/// Represents actions a page can request from the application
pub enum PageAction {
    /// Stay on the current page, no action needed
    None,
    /// Transition to a different page state
    #[allow(dead_code)]
    Transition(PageState),
    /// Request the application to quit
    Quit,
    /// Go to room selection with username
    GoToRoomSelect { username: String },
    /// Join a specific room
    JoinRoom { username: String, room: String },
    /// Go back to login
    BackToLogin,
    /// Send a chat message
    SendChat { message: String },
}

/// The current page state of the application
pub enum PageState {
    Login(LoginPage),
    RoomSelect(RoomSelectPage),
    World(WorldPage),
}

impl PageState {
    /// Create the initial page state (Login screen)
    pub fn initial() -> Self {
        PageState::Login(LoginPage::new())
    }

    /// Render the current page
    pub fn render(&mut self, frame: &mut Frame, ctx: &RenderContext) {
        match self {
            PageState::Login(page) => page.render(frame, ctx),
            PageState::RoomSelect(page) => page.render(frame),
            PageState::World(page) => page.render(frame, ctx),
        }
    }

    /// Handle input for the current page
    pub fn handle_input(&mut self, key: KeyCode, ctx: &mut UpdateContext) -> PageAction {
        match self {
            PageState::Login(page) => page.handle_input(key),
            PageState::RoomSelect(page) => page.handle_input(key),
            PageState::World(page) => page.handle_input(key, ctx),
        }
    }
}

/// Read-only context passed to pages for rendering
/// Contains shared game state that pages might need to display
pub struct RenderContext<'a> {
    pub map: &'a crate::map::Map,
    pub player: &'a crate::player::Player,
    pub remote_players: &'a HashMap<String, RemotePlayer>,
    pub chat_log: &'a Vec<ChatEntry>,
    pub is_connected: bool,
    pub update_available: Option<&'a String>,
}

/// Mutable context passed to pages for updating game state
/// Pages can modify shared state through this context
pub struct UpdateContext<'a> {
    pub map: &'a crate::map::Map,
    pub player: &'a mut crate::player::Player,
    pub send_move: Option<&'a dyn Fn(i32, i32)>,
}

// Shared UI Helpers that any page can use
pub mod helpers {
    use ratatui::layout::{Constraint, Direction, Layout, Rect};

    /// Create a centered rectangle within a given area
    pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        let final_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1]);

        final_layout[1]
    }
}
