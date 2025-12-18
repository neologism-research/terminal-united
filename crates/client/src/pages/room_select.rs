// src/pages/room_select.rs
//
// Room selection page - allows users to browse and join available rooms.

use crossterm::event::KeyCode;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use super::helpers::centered_rect;
use super::PageAction;

/// Available room info
#[derive(Clone)]
pub struct RoomInfo {
    pub name: String,
    pub players: usize,
    pub max_players: usize,
}

/// Room selection page state
pub struct RoomSelectPage {
    /// The player's username (passed from login)
    pub username: String,
    /// Available rooms
    rooms: Vec<RoomInfo>,
    /// Currently selected room index
    selected: usize,
    /// List state for rendering
    list_state: ListState,
    /// Status message
    pub status: Option<String>,
}

impl RoomSelectPage {
    pub fn new(username: String) -> Self {
        // Default rooms - in a real app, you'd fetch these from the server
        let rooms = vec![
            RoomInfo {
                name: "world".to_string(),
                players: 0,
                max_players: 50,
            },
            RoomInfo {
                name: "lobby".to_string(),
                players: 0,
                max_players: 20,
            },
            RoomInfo {
                name: "arena".to_string(),
                players: 0,
                max_players: 10,
            },
        ];

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            username,
            rooms,
            selected: 0,
            list_state,
            status: None,
        }
    }

    /// Set a status message
    pub fn set_status(&mut self, status: String) {
        self.status = Some(status);
    }

    /// Get the currently selected room name
    pub fn selected_room(&self) -> &str {
        &self.rooms[self.selected].name
    }

    /// Render the room selection screen
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let inner_area = centered_rect(60, 60, area);

        // Title block
        let block = Block::default()
            .title(format!(" Select a Room - {} ", self.username))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        frame.render_widget(block.clone(), inner_area);

        // Create list items
        let items: Vec<ListItem> = self
            .rooms
            .iter()
            .enumerate()
            .map(|(i, room)| {
                let prefix = if i == self.selected { "► " } else { "  " };
                let content = format!(
                    "{}{} ({}/{})",
                    prefix, room.name, room.players, room.max_players
                );
                let style = if i == self.selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().padding(ratatui::widgets::Padding::new(2, 2, 2, 1)));

        let list_area = Rect::new(
            inner_area.x + 1,
            inner_area.y + 1,
            inner_area.width - 2,
            inner_area.height - 4,
        );

        frame.render_stateful_widget(list, list_area, &mut self.list_state);

        // Status message
        if let Some(ref status) = self.status {
            let status_area = Rect::new(
                inner_area.x + 2,
                inner_area.y + inner_area.height - 3,
                inner_area.width - 4,
                1,
            );
            let status_text =
                Paragraph::new(status.as_str()).style(Style::default().fg(Color::Yellow));
            frame.render_widget(status_text, status_area);
        }

        // Instructions
        let hint = Paragraph::new("[↑/↓] Navigate  [Enter] Join  [Esc] Back")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));

        let hint_area = Rect::new(
            inner_area.x,
            inner_area.y + inner_area.height + 1,
            inner_area.width,
            1,
        );
        frame.render_widget(hint, hint_area);
    }

    /// Handle input specific to the room selection page
    pub fn handle_input(&mut self, key: KeyCode) -> PageAction {
        match key {
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => {
                if self.selected > 0 {
                    self.selected -= 1;
                    self.list_state.select(Some(self.selected));
                }
                PageAction::None
            }
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => {
                if self.selected < self.rooms.len() - 1 {
                    self.selected += 1;
                    self.list_state.select(Some(self.selected));
                }
                PageAction::None
            }
            KeyCode::Enter => {
                let room_name = self.rooms[self.selected].name.clone();
                PageAction::JoinRoom {
                    username: self.username.clone(),
                    room: room_name,
                }
            }
            KeyCode::Esc => PageAction::BackToLogin,
            _ => PageAction::None,
        }
    }
}
