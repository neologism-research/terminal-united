use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use terminal_united_shared::MAX_CHAT_LENGTH;

use super::super::PageAction;

pub struct StatusBar {
    chat_mode: bool,
    chat_buffer: String,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            chat_mode: false,
            chat_buffer: String::new(),
        }
    }

    pub fn is_chat_mode(&self) -> bool {
        self.chat_mode
    }

    pub fn enter_chat_mode(&mut self) {
        self.chat_mode = true;
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        username: &str,
        online_count: usize,
        mode_name: &str,
        is_connected: bool,
    ) {
        if self.chat_mode {
            self.render_chat_input(frame, area);
        } else {
            self.render_status(frame, area, username, online_count, mode_name, is_connected);
        }
    }

    fn render_status(
        &self,
        frame: &mut Frame,
        area: Rect,
        username: &str,
        online_count: usize,
        mode_name: &str,
        is_connected: bool,
    ) {
        let online_text = if is_connected {
            format!("Online: {}", online_count)
        } else {
            "Offline".to_string()
        };
        let online_color = if is_connected {
            Color::Green
        } else {
            Color::Red
        };

        let spans = vec![
            Span::styled(
                " Terminal United ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | "),
            Span::styled(format!("{} ", username), Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::styled(online_text, Style::default().fg(online_color)),
            Span::raw(" | "),
            Span::styled(
                format!("Mode: {} ", mode_name),
                Style::default().fg(Color::Magenta),
            ),
            Span::raw(" | "),
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(":chat "),
            Span::styled(
                "Tab",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(":focus "),
            Span::styled(
                "q",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(":quit"),
        ];

        let status_line = Line::from(spans);
        let paragraph = Paragraph::new(status_line).block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        frame.render_widget(paragraph, area);
    }

    fn render_chat_input(&self, frame: &mut Frame, area: Rect) {
        let title = format!(
            " Chat [{}/{}] - Enter to send, Esc to cancel ",
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
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .wrap(Wrap { trim: false });

        frame.render_widget(input, area);
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
}
