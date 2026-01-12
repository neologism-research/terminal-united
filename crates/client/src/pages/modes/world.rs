use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use super::super::{PageAction, RenderContext, UpdateContext};
use crate::map::TileType;

fn tile_display(tile: TileType) -> (&'static str, Color, Color) {
    match tile {
        TileType::Wall => ("#", Color::DarkGray, Color::Reset),
        TileType::Floor => (".", Color::Gray, Color::Reset),
        TileType::Grass => (",", Color::Green, Color::Reset),
        TileType::Water => ("~", Color::Blue, Color::DarkGray),
        TileType::Desk => ("D", Color::Yellow, Color::Reset),
        TileType::CoffeeMachine => ("🪑", Color::Magenta, Color::Reset),
        TileType::Void => (" ", Color::Reset, Color::Reset),
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum CameraType {
    Follow,
    Page,
}

impl CameraType {
    fn toggle(self) -> Self {
        match self {
            CameraType::Follow => CameraType::Page,
            CameraType::Page => CameraType::Follow,
        }
    }
}

pub struct WorldMode {
    camera_type: CameraType,
}

impl WorldMode {
    pub fn new() -> Self {
        Self {
            camera_type: CameraType::Follow,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, ctx: &RenderContext) {
        let map_block = Block::default()
            .title(" Map ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let inner_area = map_block.inner(area);
        frame.render_widget(map_block, area);

        let view_width = inner_area.width as usize;
        let view_height = (inner_area.height as usize).saturating_sub(1);

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

        // Render tiles
        for y in 0..view_height {
            for x in 0..view_width {
                let map_x = offset_x + x;
                let map_y = offset_y + y;

                if map_y < ctx.map.height && map_x < ctx.map.width {
                    let tile = ctx.map.tiles[map_y][map_x];
                    let (symbol, color, bg_color) = tile_display(tile);

                    frame.render_widget(
                        Paragraph::new(symbol).style(Style::default().fg(color).bg(bg_color)),
                        Rect::new(inner_area.x + x as u16, inner_area.y + y as u16, 1, 1),
                    );
                }
            }
        }

        // Render remote players
        for remote in ctx.remote_players.values() {
            let screen_x = remote.x as i32 - offset_x as i32;
            let screen_y = remote.y as i32 - offset_y as i32;

            if screen_x >= 0
                && screen_x < view_width as i32
                && screen_y >= 0
                && screen_y < view_height as i32
            {
                let color = if remote.username == ctx.username {
                    Color::Yellow
                } else {
                    Color::Cyan
                };

                frame.render_widget(
                    Paragraph::new("@")
                        .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
                    Rect::new(
                        inner_area.x + screen_x as u16,
                        inner_area.y + screen_y as u16,
                        1,
                        1,
                    ),
                );
            }
        }

        // Render player
        let player_screen_x = ctx.player.x as i32 - offset_x as i32;
        let player_screen_y = ctx.player.y as i32 - offset_y as i32;

        if player_screen_x >= 0
            && player_screen_x < view_width as i32
            && player_screen_y >= 0
            && player_screen_y < view_height as i32
        {
            frame.render_widget(
                Paragraph::new("@").style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Rect::new(
                    inner_area.x + player_screen_x as u16,
                    inner_area.y + player_screen_y as u16,
                    1,
                    1,
                ),
            );
        }
    }

    pub fn handle_input(&mut self, key: KeyCode, ctx: &mut UpdateContext) -> PageAction {
        match key {
            KeyCode::Char('c') => {
                self.camera_type = self.camera_type.toggle();
                PageAction::None
            }

            KeyCode::Up | KeyCode::Char('w') => {
                if ctx.player.try_move(0, -1, ctx.map) {
                    if let Some(send) = ctx.send_move {
                        send(0, -1);
                    }
                }
                PageAction::None
            }
            KeyCode::Down | KeyCode::Char('s') => {
                if ctx.player.try_move(0, 1, ctx.map) {
                    if let Some(send) = ctx.send_move {
                        send(0, 1);
                    }
                }
                PageAction::None
            }
            KeyCode::Left | KeyCode::Char('a') => {
                if ctx.player.try_move(-1, 0, ctx.map) {
                    if let Some(send) = ctx.send_move {
                        send(-1, 0);
                    }
                }
                PageAction::None
            }
            KeyCode::Right | KeyCode::Char('d') => {
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
