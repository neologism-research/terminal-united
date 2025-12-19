use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    widgets::{
        Block, Borders, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Tabs, Wrap,
    },
};
use terminal_united_shared::{MAX_CHAT_LENGTH, PROXIMITY_DISTANCE};

use crate::map::TileType;

use super::hints::{hint, hint_key, hints_line_piped, info, status};
use super::{PageAction, RenderContext, UpdateContext};

fn tile_display(tile: TileType) -> (&'static str, Color, Color) {
    match tile {
        TileType::Wall => ("#", Color::DarkGray, Color::Reset),
        TileType::Floor => (".", Color::Gray, Color::Reset),
        TileType::Grass => (",", Color::Green, Color::Reset),
        TileType::Water => ("~", Color::Blue, Color::DarkGray),
        TileType::Desk => ("D", Color::Yellow, Color::Reset),
        TileType::CoffeeMachine => ("C", Color::Magenta, Color::Reset),
        TileType::Void => (" ", Color::Reset, Color::Reset),
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum CameraType {
    Follow,
    Page,
}

#[derive(PartialEq, Clone, Copy)]
enum SidebarZone {
    Map,
    PlayerList,
    ChatLog,
}

#[derive(PartialEq, Clone, Copy)]
enum PlayerTab {
    World,
    Nearby,
}

pub struct WorldPage {
    username: String,
    camera_type: CameraType,
    chat_mode: bool,
    chat_buffer: String,
    focused_zone: SidebarZone,
    player_tab: PlayerTab,
    player_list_state: ListState,
    chat_list_state: ListState,
    last_chat_len: usize,
}

impl WorldPage {
    pub fn new(username: String) -> Self {
        Self {
            username,
            camera_type: CameraType::Follow,
            chat_mode: false,
            chat_buffer: String::new(),
            focused_zone: SidebarZone::Map,
            player_tab: PlayerTab::World,
            player_list_state: ListState::default(),
            chat_list_state: ListState::default(),
            last_chat_len: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, ctx: &RenderContext) {
        let screen_area = frame.area();

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(40), Constraint::Length(30)])
            .split(screen_area);

        self.render_map(frame, ctx, main_layout[0]);
        self.render_sidebar(frame, ctx, main_layout[1]);

        if self.chat_mode {
            self.render_chat_input(frame, screen_area);
        }
    }

    fn render_map(&self, frame: &mut Frame, ctx: &RenderContext, area: Rect) {
        let map_border = if self.focused_zone == SidebarZone::Map {
            Color::Yellow
        } else {
            Color::Cyan
        };

        let map_block = Block::default()
            .title(" Map ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(map_border));

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
                frame.render_widget(
                    Paragraph::new("@").style(
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Rect::new(
                        inner_area.x + screen_x as u16,
                        inner_area.y + screen_y as u16,
                        1,
                        1,
                    ),
                );

                if screen_y > 0 {
                    let name = if remote.username.len() > 10 {
                        &remote.username[..10]
                    } else {
                        &remote.username
                    };
                    frame.render_widget(
                        Paragraph::new(name).style(Style::default().fg(Color::Magenta)),
                        Rect::new(
                            inner_area.x + screen_x as u16,
                            inner_area.y + (screen_y - 1) as u16,
                            name.len() as u16,
                            1,
                        ),
                    );
                }
            }
        }

        // Render local player
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
                Rect::new(
                    inner_area.x + screen_player_x as u16,
                    inner_area.y + screen_player_y as u16,
                    1,
                    1,
                ),
            );
        }

        // Render HUD
        self.render_hud(frame, ctx, inner_area);
    }

    fn render_hud(&self, frame: &mut Frame, ctx: &RenderContext, inner_area: Rect) {
        let online_text = if ctx.is_connected {
            format!("Online ({} others)", ctx.remote_players.len())
        } else {
            "Offline".to_string()
        };
        let online_color = if ctx.is_connected {
            Color::Green
        } else {
            Color::Red
        };

        let focus_text = match self.focused_zone {
            SidebarZone::Map => "Map",
            SidebarZone::PlayerList => "Players",
            SidebarZone::ChatLog => "Chat",
        };

        let cam_text = match self.camera_type {
            CameraType::Follow => "FOLLOW",
            CameraType::Page => "PAGE",
        };

        let mut hints: Vec<Vec<Span<'static>>> = vec![
            status(&format!(" {} ", self.username)),
            info(&online_text, online_color),
        ];

        if self.chat_mode {
            hints.push(info("[CHAT MODE]", Color::Cyan));
        } else {
            hints.push(hint_key("Enter", "chat"));
        }

        hints.push(hint_key("Tab", focus_text));
        hints.push(hint_key("c", cam_text));
        hints.push(hint("quit"));
        hints.push(info(
            &format!("({},{})", ctx.player.x, ctx.player.y),
            Color::DarkGray,
        ));

        let hud_line = hints_line_piped(&hints);
        let hud_paragraph = Paragraph::new(hud_line).alignment(Alignment::Center);

        frame.render_widget(
            hud_paragraph,
            Rect::new(
                inner_area.x,
                inner_area.y + inner_area.height,
                inner_area.width,
                1,
            ),
        );
    }

    fn render_sidebar(&mut self, frame: &mut Frame, ctx: &RenderContext, area: Rect) {
        let sidebar_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);

        self.render_player_list(frame, ctx, sidebar_layout[0]);
        self.render_chat(frame, ctx, sidebar_layout[1]);
    }

    fn render_player_list(&mut self, frame: &mut Frame, ctx: &RenderContext, area: Rect) {
        let border_color = if self.focused_zone == SidebarZone::PlayerList {
            Color::Yellow
        } else {
            Color::DarkGray
        };

        let my_x = ctx.player.x as i32;
        let my_y = ctx.player.y as i32;

        let world_count = ctx.remote_players.len();
        let nearby_count = ctx
            .remote_players
            .values()
            .filter(|p| (p.x - my_x).abs() + (p.y - my_y).abs() <= PROXIMITY_DISTANCE)
            .count();

        let player_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(3)])
            .split(area);

        // Tabs header
        let tab_titles = vec![
            format!(" World ({}) ", world_count),
            format!(" Nearby ({}) ", nearby_count),
        ];
        let selected_tab = if self.player_tab == PlayerTab::World {
            0
        } else {
            1
        };

        let tabs = Tabs::new(tab_titles)
            .select(selected_tab)
            .style(Style::default().fg(Color::DarkGray))
            .highlight_style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED),
            )
            .divider("|");

        let tabs_block = Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
            .border_style(Style::default().fg(border_color));
        frame.render_widget(tabs_block, player_layout[0]);
        frame.render_widget(
            tabs,
            player_layout[0].inner(ratatui::layout::Margin {
                horizontal: 1,
                vertical: 0,
            }),
        );

        // Player list content
        let (items, item_count): (Vec<ListItem>, usize) = match self.player_tab {
            PlayerTab::World => {
                let items: Vec<ListItem> = ctx
                    .remote_players
                    .values()
                    .map(|p| {
                        ListItem::new(format!("  {} ", p.username))
                            .style(Style::default().fg(Color::Gray))
                    })
                    .collect();
                let count = items.len();
                if items.is_empty() {
                    (
                        vec![
                            ListItem::new("  (no players)")
                                .style(Style::default().fg(Color::DarkGray)),
                        ],
                        1,
                    )
                } else {
                    (items, count)
                }
            }
            PlayerTab::Nearby => {
                let items: Vec<ListItem> = ctx
                    .remote_players
                    .values()
                    .filter(|p| (p.x - my_x).abs() + (p.y - my_y).abs() <= PROXIMITY_DISTANCE)
                    .map(|p| {
                        let dist = (p.x - my_x).abs() + (p.y - my_y).abs();
                        ListItem::new(format!("  {} ({}m)", p.username, dist)).style(
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        )
                    })
                    .collect();
                let count = items.len();
                if items.is_empty() {
                    (
                        vec![
                            ListItem::new("  (nobody nearby)")
                                .style(Style::default().fg(Color::DarkGray)),
                        ],
                        1,
                    )
                } else {
                    (items, count)
                }
            }
        };

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                    .border_style(Style::default().fg(border_color)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        frame.render_stateful_widget(list, player_layout[1], &mut self.player_list_state);

        // Scrollbar
        let scroll_pos = self.player_list_state.selected().unwrap_or(0);
        let mut scrollbar_state = ScrollbarState::new(item_count.max(1)).position(scroll_pos);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼")),
            player_layout[1].inner(ratatui::layout::Margin {
                horizontal: 0,
                vertical: 1,
            }),
            &mut scrollbar_state,
        );
    }

    fn render_chat(&mut self, frame: &mut Frame, ctx: &RenderContext, area: Rect) {
        let border_color = if self.focused_zone == SidebarZone::ChatLog {
            Color::Yellow
        } else {
            Color::Blue
        };

        // Auto-scroll on new messages
        let chat_len = ctx.chat_log.len();
        if chat_len > self.last_chat_len && self.focused_zone != SidebarZone::ChatLog {
            if chat_len > 0 {
                self.chat_list_state.select(Some(chat_len - 1));
            }
        }
        self.last_chat_len = chat_len;

        let messages: Vec<ListItem> = ctx
            .chat_log
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

                ListItem::new(format!("{} {}: {}", prefix, entry.username, entry.message))
                    .style(style)
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

        frame.render_stateful_widget(list, area, &mut self.chat_list_state);

        // Scrollbar
        let scroll_pos = self.chat_list_state.selected().unwrap_or(0);
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

    fn render_chat_input(&self, frame: &mut Frame, area: Rect) {
        let input_area = Rect::new(0, area.height.saturating_sub(4), area.width, 3);

        frame.render_widget(
            Block::default().style(Style::default().bg(Color::Black)),
            input_area,
        );

        let input = Paragraph::new(format!(" > {}_", self.chat_buffer))
            .block(
                Block::default()
                    .title(" Chat (Enter to send, Esc to cancel) ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: false });

        frame.render_widget(input, input_area);
    }

    pub fn handle_input(&mut self, key: KeyCode, ctx: &mut UpdateContext) -> PageAction {
        if self.chat_mode {
            return self.handle_chat_input(key);
        }

        match key {
            KeyCode::Char('q') | KeyCode::Esc => PageAction::Quit,

            KeyCode::Enter => {
                self.chat_mode = true;
                PageAction::None
            }

            KeyCode::Tab => {
                self.focused_zone = self.focused_zone.next();
                PageAction::None
            }

            KeyCode::Char('c') => {
                self.camera_type = self.camera_type.toggle();
                PageAction::None
            }

            KeyCode::Up | KeyCode::Char('w') => {
                self.handle_direction(0, -1, ctx);
                PageAction::None
            }
            KeyCode::Down | KeyCode::Char('s') => {
                self.handle_direction(0, 1, ctx);
                PageAction::None
            }
            KeyCode::Left | KeyCode::Char('a') => {
                self.handle_direction(-1, 0, ctx);
                PageAction::None
            }
            KeyCode::Right | KeyCode::Char('d') => {
                self.handle_direction(1, 0, ctx);
                PageAction::None
            }

            _ => PageAction::None,
        }
    }

    fn handle_direction(&mut self, dx: i32, dy: i32, ctx: &mut UpdateContext) {
        match self.focused_zone {
            SidebarZone::Map => {
                if ctx.player.try_move(dx, dy, ctx.map) {
                    if let Some(send) = ctx.send_move {
                        send(dx, dy);
                    }
                }
            }
            SidebarZone::PlayerList => {
                if dx != 0 {
                    self.player_tab = self.player_tab.toggle();
                    self.player_list_state.select(Some(0));
                } else {
                    Self::scroll_list(&mut self.player_list_state, dy);
                }
            }
            SidebarZone::ChatLog => {
                if dy != 0 {
                    Self::scroll_list(&mut self.chat_list_state, dy);
                }
            }
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

    fn handle_chat_input(&mut self, key: KeyCode) -> PageAction {
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

impl SidebarZone {
    fn next(self) -> Self {
        match self {
            SidebarZone::Map => SidebarZone::PlayerList,
            SidebarZone::PlayerList => SidebarZone::ChatLog,
            SidebarZone::ChatLog => SidebarZone::Map,
        }
    }
}

impl CameraType {
    fn toggle(self) -> Self {
        match self {
            CameraType::Follow => CameraType::Page,
            CameraType::Page => CameraType::Follow,
        }
    }
}

impl PlayerTab {
    fn toggle(self) -> Self {
        match self {
            PlayerTab::World => PlayerTab::Nearby,
            PlayerTab::Nearby => PlayerTab::World,
        }
    }
}
