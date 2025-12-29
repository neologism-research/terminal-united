use crossterm::event::KeyCode;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use super::PageAction;
use super::helpers::centered_rect;
use super::hints::{arrows, hint_key, hints_line};

#[derive(Clone)]
pub struct RoomInfo {
    pub name: String,
    pub players: usize,
    pub max_players: usize,
}

pub struct RoomSelectPage {
    pub username: String,
    rooms: Vec<RoomInfo>,
    selected: usize,
    list_state: ListState,
    pub status: Option<String>,
}

impl RoomSelectPage {
    pub fn new(username: String) -> Self {
        let rooms = vec![
            RoomInfo {
                name: "world".to_string(),
                players: 0,
                max_players: 50,
            },
            // RoomInfo {
            //     name: "lobby".to_string(),
            //     players: 0,
            //     max_players: 20,
            // },
            // RoomInfo {
            //     name: "arena".to_string(),
            //     players: 0,
            //     max_players: 10,
            // },
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

    pub fn set_status(&mut self, status: String) {
        self.status = Some(status);
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let inner_area = centered_rect(60, 60, area);

        let block = Block::default()
            .title(format!(" Select a Room - {} ", self.username))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        frame.render_widget(block.clone(), inner_area);

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

        let hint_line = hints_line(&[
            arrows("navigate"),
            hint_key("Enter", "join"),
            hint_key("Esc", "back"),
        ]);
        let hint = Paragraph::new(hint_line).alignment(Alignment::Center);

        let hint_area = Rect::new(
            inner_area.x,
            inner_area.y + inner_area.height + 1,
            inner_area.width,
            1,
        );
        frame.render_widget(hint, hint_area);
    }

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
            KeyCode::Enter => PageAction::JoinRoom {
                username: self.username.clone(),
                room: self.rooms[self.selected].name.clone(),
            },
            KeyCode::Esc => PageAction::BackToLogin,
            _ => PageAction::None,
        }
    }
}
