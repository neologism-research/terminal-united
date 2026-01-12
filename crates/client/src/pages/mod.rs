pub mod components;
pub mod helpers;
pub mod hints;
pub mod layout;
pub mod login;
pub mod modes;
pub mod world;

pub use layout::GameLayout;
pub use login::LoginPage;
pub use modes::GameMode;

use crossterm::event::KeyCode;
use ratatui::Frame;
use std::collections::HashMap;

use crate::network::{ChatEntry, RemotePlayer};

pub enum PageAction {
    None,
    Quit,
    JoinWorld { username: String },
    SendChat { message: String },
}

pub enum PageState {
    Login(LoginPage),
    InGame(GameLayout),
}

impl PageState {
    pub fn initial() -> Self {
        PageState::Login(LoginPage::new())
    }

    pub fn render(&mut self, frame: &mut Frame, ctx: &RenderContext) {
        match self {
            PageState::Login(page) => page.render(frame, ctx),
            PageState::InGame(layout) => layout.render(frame, ctx),
        }
    }

    pub fn handle_input(&mut self, key: KeyCode, ctx: &mut UpdateContext) -> PageAction {
        match self {
            PageState::Login(page) => page.handle_input(key),
            PageState::InGame(layout) => layout.handle_input(key, ctx),
        }
    }
}

pub struct RenderContext<'a> {
    pub map: &'a crate::map::Map,
    pub player: &'a crate::player::Player,
    pub remote_players: &'a HashMap<String, RemotePlayer>,
    pub chat_log: &'a Vec<ChatEntry>,
    pub is_connected: bool,
    pub update_available: Option<&'a String>,
    pub username: &'a str,
}

pub struct UpdateContext<'a> {
    pub map: &'a crate::map::Map,
    pub player: &'a mut crate::player::Player,
    pub send_move: Option<&'a dyn Fn(i32, i32)>,
}
