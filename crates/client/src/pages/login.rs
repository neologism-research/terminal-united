use crossterm::event::KeyCode;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use terminal_united_shared::MAX_USERNAME_LENGTH;

use super::helpers::centered_rect;
use super::hints::{hint_key, hints_line};
use super::{PageAction, RenderContext};

pub struct LoginPage {
    username: String,
    pub status: Option<String>,
}

impl LoginPage {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            status: None,
        }
    }

    pub fn render(&self, frame: &mut Frame, ctx: &RenderContext) {
        let area = frame.area();
        let inner_area = centered_rect(50, 30, area);

        let block = Block::default()
            .title(" Welcome to Terminal United ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let input_text = if self.username.is_empty() {
            "Type username..."
        } else {
            &self.username
        };

        let mut content = format!("\n\n   > {}_", input_text);

        if let Some(ref status) = self.status {
            content.push_str(&format!("\n\n   {}", status));
        }

        if let Some(version) = ctx.update_available {
            content.push_str(&format!("\n\n   [!] Update available: v{}", version));
        }

        let text = Paragraph::new(content)
            .block(block)
            .style(Style::default().fg(Color::White));

        frame.render_widget(text, inner_area);

        let hint_line = hints_line(&[hint_key("Enter", "connect"), hint_key("Esc", "quit")]);
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
            KeyCode::Char(c) => {
                if self.username.len() < MAX_USERNAME_LENGTH {
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
                    PageAction::JoinWorld {
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
