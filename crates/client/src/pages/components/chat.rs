use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
};
use terminal_united_shared::MAX_CHAT_LENGTH;

use crate::network::ChatEntry;
use super::super::PageAction;

pub struct ChatWidget {
    list_state: ListState,
    is_focused: bool,
    chat_mode: bool,
    chat_buffer: String,
    last_chat_len: usize,
}

impl ChatWidget {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
            is_focused: false,
            chat_mode: false,
            chat_buffer: String::new(),
            last_chat_len: 0,
        }
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    pub fn is_chat_mode(&self) -> bool {
        self.chat_mode
    }

    pub fn enter_chat_mode(&mut self) {
        self.chat_mode = true;
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, chat_log: &[ChatEntry]) {
        let border_color = if self.is_focused {
            Color::Yellow
        } else {
            Color::Blue
        };

        // Auto-scroll on new messages
        let chat_len = chat_log.len();
        if chat_len > self.last_chat_len && !self.is_focused {
            if chat_len > 0 {
                self.list_state.select(Some(chat_len - 1));
            }
        }
        self.last_chat_len = chat_len;

        let inner_width = (area.width as usize).saturating_sub(2);

        let messages: Vec<ListItem> = chat_log
            .iter()
            .map(|entry| {
                let style = if entry.is_system {
                    Style::default().fg(Color::Yellow)
                } else if entry.is_proximity {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if entry.is_system {
                    "*"
                } else if entry.is_proximity {
                    ">"
                } else {
                    " "
                };

                let full_text = format!("{} {}: {}", prefix, entry.username, entry.message);
                let wrapped_lines = textwrap::wrap(&full_text, inner_width);

                let lines: Vec<Line> = wrapped_lines
                    .into_iter()
                    .map(|s| Line::from(s.to_string()))
                    .collect();

                ListItem::new(lines).style(style)
            })
            .collect();

        let list = List::new(if messages.is_empty() {
            vec![ListItem::new("  (no messages)").style(Style::default().fg(Color::DarkGray))]
        } else {
            messages
        })
        .block(
            Block::default()
                .title(" Chat ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        frame.render_stateful_widget(list, area, &mut self.list_state);

        // Scrollbar
        let scroll_pos = self.list_state.selected().unwrap_or(0);
        let mut scrollbar_state = ScrollbarState::new(chat_len.max(1)).position(scroll_pos);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼")),
            area.inner(ratatui::layout::Margin {
                horizontal: 0,
                vertical: 1,
            }),
            &mut scrollbar_state,
        );
    }

    pub fn render_input(&self, frame: &mut Frame, area: Rect) {
        if !self.chat_mode {
            return;
        }

        let input_area = Rect::new(0, area.height.saturating_sub(4), area.width, 3);

        frame.render_widget(
            Block::default().style(Style::default().bg(Color::Black)),
            input_area,
        );

        let title = format!(
            " Chat (Enter to send, Esc to cancel) [{}/{}] ",
            self.chat_buffer.len(),
            MAX_CHAT_LENGTH
        );

        let input = Paragraph::new(format!(" > {}_", self.chat_buffer))
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: false });

        frame.render_widget(input, input_area);
    }

    pub fn handle_input(&mut self, dy: i32) {
        if dy != 0 {
            Self::scroll_list(&mut self.list_state, dy);
        }
    }

    pub fn handle_chat_input(&mut self, key: KeyCode) -> PageAction {
        match key {
            KeyCode::Esc => {
                self.chat_mode = false;
                self.chat_buffer.clear();
                PageAction::None
            }
            KeyCode::Enter => {
                let action = if !self.chat_buffer.trim().is_empty() {
                    PageAction::SendChat {
                        message: self.chat_buffer.trim().to_string(),
                    }
                } else {
                    PageAction::None
                };
                self.chat_mode = false;
                self.chat_buffer.clear();
                action
            }
            KeyCode::Backspace => {
                self.chat_buffer.pop();
                PageAction::None
            }
            KeyCode::Char(c) => {
                if self.chat_buffer.len() < MAX_CHAT_LENGTH {
                    self.chat_buffer.push(c);
                }
                PageAction::None
            }
            _ => PageAction::None,
        }
    }

    fn scroll_list(state: &mut ListState, amount: i32) {
        let current = state.selected().unwrap_or(0);
        let new_pos = if amount > 0 {
            current.saturating_add(amount as usize)
        } else {
            current.saturating_sub(amount.unsigned_abs() as usize)
        };
        state.select(Some(new_pos));
    }
}
