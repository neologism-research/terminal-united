pub mod helpers;
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

pub enum PageAction {
    None,
    Quit,
    GoToRoomSelect { username: String },
    JoinRoom { username: String, room: String },
    BackToLogin,
    SendChat { message: String },
}

pub enum PageState {
    Login(LoginPage),
    RoomSelect(RoomSelectPage),
    World(WorldPage),
}

impl PageState {
    pub fn initial() -> Self {
        PageState::Login(LoginPage::new())
    }

    pub fn render(&mut self, frame: &mut Frame, ctx: &RenderContext) {
        match self {
            PageState::Login(page) => page.render(frame, ctx),
            PageState::RoomSelect(page) => page.render(frame),
            PageState::World(page) => page.render(frame, ctx),
        }
    }

    pub fn handle_input(&mut self, key: KeyCode, ctx: &mut UpdateContext) -> PageAction {
        match self {
            PageState::Login(page) => page.handle_input(key),
            PageState::RoomSelect(page) => page.handle_input(key),
            PageState::World(page) => page.handle_input(key, ctx),
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
}

pub struct UpdateContext<'a> {
    pub map: &'a crate::map::Map,
    pub player: &'a mut crate::player::Player,
    pub send_move: Option<&'a dyn Fn(i32, i32)>,
}
