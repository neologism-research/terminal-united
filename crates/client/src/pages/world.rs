// src/pages/world.rs
//
// World page - the main game screen with map rendering and player movement.
// All world-related state, rendering, and input handling is contained here.

use crossterm::event::KeyCode;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::map::TileType;

use super::{PageAction, RenderContext, UpdateContext};

/// Camera behavior modes
#[derive(PartialEq, Clone, Copy)]
pub enum CameraType {
    /// Player stays in center, map moves
    Follow,
    /// Map stays static, flips when you hit edge
    Page,
}

/// World page state
pub struct WorldPage {
    /// The player's display name
    username: String,
    /// Current camera mode
    camera_type: CameraType,
}

impl WorldPage {
    pub fn new(username: String) -> Self {
        Self {
            username,
            camera_type: CameraType::Follow,
        }
    }

    /// Render the game world
    pub fn render(&self, frame: &mut Frame, ctx: &RenderContext) {
        let screen_area = frame.area();
        let view_width = screen_area.width as usize;
        let view_height = (screen_area.height as usize).saturating_sub(1);

        // --- CAMERA LOGIC ---
        let (offset_x, offset_y) = match self.camera_type {
            CameraType::Follow => {
                let center_x = view_width / 2;
                let center_y = view_height / 2;
                (
                    ctx.player.x.saturating_sub(center_x),
                    ctx.player.y.saturating_sub(center_y),
                )
            }
            CameraType::Page => {
                let page_x = ctx.player.x / view_width;
                let page_y = ctx.player.y / view_height;
                (page_x * view_width, page_y * view_height)
            }
        };

        // --- RENDER MAP ---
        for y in 0..view_height {
            for x in 0..view_width {
                let map_x = offset_x + x;
                let map_y = offset_y + y;

                if map_y < ctx.map.height && map_x < ctx.map.width {
                    let tile = ctx.map.tiles[map_y][map_x];
                    let (symbol, color, bg_color) = match tile {
                        TileType::Wall => ("#", Color::DarkGray, Color::Reset),
                        TileType::Floor => (".", Color::Gray, Color::Reset),
                        TileType::Grass => ("\"", Color::Green, Color::Reset),
                        TileType::Water => ("~", Color::Blue, Color::Reset),
                        TileType::Desk => ("D", Color::Yellow, Color::Reset),
                        TileType::CoffeeMachine => ("C", Color::Red, Color::Reset),
                        TileType::Void => (" ", Color::Reset, Color::Reset),
                    };

                    frame.render_widget(
                        Paragraph::new(symbol).style(Style::default().fg(color).bg(bg_color)),
                        Rect::new(x as u16, y as u16, 1, 1),
                    );
                }
            }
        }

        // --- RENDER REMOTE PLAYERS ---
        for (_id, remote) in ctx.remote_players.iter() {
            let screen_x = remote.x as i32 - offset_x as i32;
            let screen_y = remote.y as i32 - offset_y as i32;

            if screen_x >= 0
                && screen_x < view_width as i32
                && screen_y >= 0
                && screen_y < view_height as i32
            {
                // Render remote player with different color
                frame.render_widget(
                    Paragraph::new("@").style(
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Rect::new(screen_x as u16, screen_y as u16, 1, 1),
                );

                // Render username above player if space allows
                if screen_y > 0 {
                    let name_display = if remote.username.len() > 10 {
                        &remote.username[..10]
                    } else {
                        &remote.username
                    };
                    frame.render_widget(
                        Paragraph::new(name_display)
                            .style(Style::default().fg(Color::Magenta)),
                        Rect::new(screen_x as u16, (screen_y - 1) as u16, name_display.len() as u16, 1),
                    );
                }
            }
        }

        // --- RENDER LOCAL PLAYER ---
        let screen_player_x = ctx.player.x as i32 - offset_x as i32;
        let screen_player_y = ctx.player.y as i32 - offset_y as i32;

        if screen_player_x >= 0
            && screen_player_x < view_width as i32
            && screen_player_y >= 0
            && screen_player_y < view_height as i32
        {
            frame.render_widget(
                Paragraph::new(ctx.player.char.to_string()).style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Rect::new(screen_player_x as u16, screen_player_y as u16, 1, 1),
            );
        }

        // --- RENDER HUD ---
        let cam_status = match self.camera_type {
            CameraType::Follow => "FOLLOW",
            CameraType::Page => "PAGE",
        };

        let online_status = if ctx.is_connected {
            format!("Online ({} others)", ctx.remote_players.len())
        } else {
            "Offline".to_string()
        };

        let status_text = format!(
            " {} | {} | CAM: {} | Pos: ({}, {}) ",
            self.username, online_status, cam_status, ctx.player.x, ctx.player.y
        );
        let status_bar = Block::default()
            .borders(Borders::TOP)
            .title(status_text)
            .title_alignment(Alignment::Center);

        let hud_area = Rect::new(0, screen_area.height - 1, screen_area.width, 1);
        frame.render_widget(status_bar, hud_area);
    }

    /// Handle input specific to the world page
    pub fn handle_input(&mut self, key: KeyCode, ctx: &mut UpdateContext) -> PageAction {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => PageAction::Quit,

            // Toggle Camera
            KeyCode::Char('c') | KeyCode::Tab => {
                self.camera_type = match self.camera_type {
                    CameraType::Follow => CameraType::Page,
                    CameraType::Page => CameraType::Follow,
                };
                PageAction::None
            }

            // Movement - also send to server if connected
            KeyCode::Char('w') | KeyCode::Up => {
                if ctx.player.try_move(0, -1, ctx.map) {
                    if let Some(send) = ctx.send_move {
                        send(0, -1);
                    }
                }
                PageAction::None
            }
            KeyCode::Char('s') | KeyCode::Down => {
                if ctx.player.try_move(0, 1, ctx.map) {
                    if let Some(send) = ctx.send_move {
                        send(0, 1);
                    }
                }
                PageAction::None
            }
            KeyCode::Char('a') | KeyCode::Left => {
                if ctx.player.try_move(-1, 0, ctx.map) {
                    if let Some(send) = ctx.send_move {
                        send(-1, 0);
                    }
                }
                PageAction::None
            }
            KeyCode::Char('d') | KeyCode::Right => {
                if ctx.player.try_move(1, 0, ctx.map) {
                    if let Some(send) = ctx.send_move {
                        send(1, 0);
                    }
                }
                PageAction::None
            }

            _ => PageAction::None,
        }
    }
}
