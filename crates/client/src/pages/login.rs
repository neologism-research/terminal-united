// src/pages/login.rs
//
// Login page - handles username input and connects to the game server.
// All login-related state, rendering, and input handling is contained here.

use crossterm::event::KeyCode;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use super::helpers::centered_rect;
use super::PageAction;

/// Login page state
pub struct LoginPage {
    /// The username being typed by the user
    username: String,
    /// Status message (for connection feedback)
    pub status: Option<String>,
}

impl LoginPage {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            status: None,
        }
    }

    /// Set a status message (for connection errors, etc.)
    pub fn set_status(&mut self, status: String) {
        self.status = Some(status);
    }

    /// Render the login screen
    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let block = Block::default()
            .title(" Welcome to Terminal United ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let inner_area = centered_rect(50, 30, area);

        let input_text = if self.username.is_empty() {
            "Type username..."
        } else {
            &self.username
        };

        // Build the content with status message if present
        let content = if let Some(ref status) = self.status {
            format!("\n\n   > {}_\n\n   {}", input_text, status)
        } else {
            format!("\n\n   > {}_", input_text)
        };

        let text = Paragraph::new(content)
            .block(block)
            .style(Style::default().fg(Color::White));

        frame.render_widget(text, inner_area);

        let hint = Paragraph::new("Press [Enter] to Connect, [Esc] to Quit")
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

    /// Handle input specific to the login page
    pub fn handle_input(&mut self, key: KeyCode) -> PageAction {
        match key {
            KeyCode::Char(c) => {
                if self.username.len() < 20 {
                    self.username.push(c);
                }
                PageAction::None
            }
            KeyCode::Backspace => {
                self.username.pop();
                PageAction::None
            }
            KeyCode::Enter => {
                if !self.username.trim().is_empty() {
                    // Go to room selection with username
                    PageAction::GoToRoomSelect {
                        username: self.username.trim().to_string(),
                    }
                } else {
                    PageAction::None
                }
            }
            KeyCode::Esc => PageAction::Quit,
            _ => PageAction::None,
        }
    }
}

impl Default for LoginPage {
    fn default() -> Self {
        Self::new()
    }
}
