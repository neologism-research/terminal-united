pub mod world;

pub use world::WorldMode;

use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::Rect;

use super::{PageAction, UpdateContext, RenderContext};

pub enum GameMode {
    World(WorldMode),
    // Poker(PokerMode),    // Future
    // Combat(CombatMode),  // Future
}

impl GameMode {
    pub fn render(&mut self, frame: &mut Frame, area: Rect, ctx: &RenderContext) {
        match self {
            GameMode::World(mode) => mode.render(frame, area, ctx),
        }
    }

    pub fn handle_input(&mut self, key: KeyCode, ctx: &mut UpdateContext) -> PageAction {
        match self {
            GameMode::World(mode) => mode.handle_input(key, ctx),
        }
    }
}
