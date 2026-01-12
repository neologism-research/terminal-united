use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use super::components::{ChatWidget, PlayerListWidget, StatusBar};
use super::modes::GameMode;
use super::{PageAction, RenderContext, UpdateContext};

#[derive(PartialEq, Clone, Copy)]
enum FocusZone {
    Content,
    PlayerList,
    Chat,
}

impl FocusZone {
    fn next(self) -> Self {
        match self {
            FocusZone::Content => FocusZone::PlayerList,
            FocusZone::PlayerList => FocusZone::Chat,
            FocusZone::Chat => FocusZone::Content,
        }
    }
}

pub struct GameLayout {
    mode: GameMode,
    player_list: PlayerListWidget,
    chat: ChatWidget,
    status_bar: StatusBar,
    focused_zone: FocusZone,
}

impl GameLayout {
    pub fn new(mode: GameMode) -> Self {
        Self {
            mode,
            player_list: PlayerListWidget::new(),
            status_bar: StatusBar::new(),
            chat: ChatWidget::new(),
            focused_zone: FocusZone::Content,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, ctx: &RenderContext) {
        let screen_area = frame.area();

        // Vertical split: Main content + Status bar
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(3)])
            .split(screen_area);

        let main_area = vertical_layout[0];
        let status_area = vertical_layout[1];

        // Main horizontal split: Content | Sidebar
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(40), Constraint::Length(30)])
            .split(main_area);

        // Sidebar vertical split: PlayerList / Chat
        let sidebar_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[1]);

        // Render mode content
        self.mode.render(frame, main_layout[0], ctx);

        // Render player list
        self.player_list
            .set_focused(self.focused_zone == FocusZone::PlayerList);
        self.player_list.render(
            frame,
            sidebar_layout[0],
            ctx.remote_players,
            ctx.player.x as i32,
            ctx.player.y as i32,
        );

        // Render chat
        self.chat.set_focused(self.focused_zone == FocusZone::Chat);
        self.chat.render(frame, sidebar_layout[1], ctx.chat_log);

        // Render status bar
        let mode_name = match &self.mode {
            GameMode::World(_) => "World",
        };
        self.status_bar.render(
            frame,
            status_area,
            ctx.username,
            ctx.remote_players.len(),
            mode_name,
            ctx.is_connected,
        );
    }

    pub fn handle_input(&mut self, key: KeyCode, ctx: &mut UpdateContext) -> PageAction {
        // Status bar chat input has priority
        if self.status_bar.is_chat_mode() {
            return self.status_bar.handle_chat_input(key);
        }

        match key {
            KeyCode::Char('q') | KeyCode::Esc => PageAction::Quit,

            KeyCode::Enter => {
                self.status_bar.enter_chat_mode();
                PageAction::None
            }

            KeyCode::Tab => {
                self.focused_zone = self.focused_zone.next();
                PageAction::None
            }

            // Movement keys
            KeyCode::Up | KeyCode::Char('w') => self.handle_direction(0, -1, ctx),
            KeyCode::Down | KeyCode::Char('s') => self.handle_direction(0, 1, ctx),
            KeyCode::Left | KeyCode::Char('a') => self.handle_direction(-1, 0, ctx),
            KeyCode::Right | KeyCode::Char('d') => self.handle_direction(1, 0, ctx),

            // Pass other keys to current mode
            _ => self.mode.handle_input(key, ctx),
        }
    }

    fn handle_direction(&mut self, dx: i32, dy: i32, ctx: &mut UpdateContext) -> PageAction {
        match self.focused_zone {
            FocusZone::Content => self.mode.handle_input(
                if dx != 0 {
                    if dx > 0 {
                        KeyCode::Right
                    } else {
                        KeyCode::Left
                    }
                } else if dy > 0 {
                    KeyCode::Down
                } else {
                    KeyCode::Up
                },
                ctx,
            ),
            FocusZone::PlayerList => {
                self.player_list.handle_input(dx, dy);
                PageAction::None
            }
            FocusZone::Chat => {
                self.chat.handle_input(dy);
                PageAction::None
            }
        }
    }
}
