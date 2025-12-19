// src/pages/world.rs
//
// World page - the main game screen with map rendering and player movement.
// Features a sidebar with world list, chat log, and proximity list.

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    widgets::{
        Block, Borders, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Tabs, Wrap,
    },
};

use crate::map::TileType;

use super::hints::{hint, hint_key, hints_line_piped, info, status};
use super::{PageAction, RenderContext, UpdateContext};

/// Proximity distance for nearby players
const PROXIMITY_DISTANCE: i32 = 20;

/// Camera behavior modes
#[derive(PartialEq, Clone, Copy)]
pub enum CameraType {
    /// Player stays in center, map moves
    Follow,
    /// Map stays static, flips when you hit edge
    Page,
}

/// Zones that can be focused
#[derive(PartialEq, Clone, Copy)]
enum SidebarZone {
    Map,
    PlayerList,
    ChatLog,
}

/// Player list tabs
#[derive(PartialEq, Clone, Copy)]
enum PlayerTab {
    World,
    Nearby,
}

/// World page state
pub struct WorldPage {
    /// The player's display name
    username: String,
    /// Current camera mode
    camera_type: CameraType,
    /// Whether we're in chat mode
    chat_mode: bool,
    /// Current chat input buffer
    chat_buffer: String,
    /// Currently focused zone
    focused_zone: SidebarZone,
    /// Current player list tab
    player_tab: PlayerTab,
    /// State for player list scrolling (shared between tabs)
    player_list_state: ListState,
    /// State for chat log scrolling
    chat_list_state: ListState,
    /// Track last chat log length for auto-scroll
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

    /// Render the game world with sidebar
    pub fn render(&mut self, frame: &mut Frame, ctx: &RenderContext) {
        let screen_area = frame.area();

        // Split into main area (map) and sidebar
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(40),    // Map area (flexible, minimum 40)
                Constraint::Length(30), // Sidebar (fixed 30 chars)
            ])
            .split(screen_area);

        let map_area = main_layout[0];
        let sidebar_area = main_layout[1];

        // Render the map
        self.render_map(frame, ctx, map_area);

        // Render the sidebar
        self.render_sidebar(frame, ctx, sidebar_area);

        // Render chat input if in chat mode
        if self.chat_mode {
            self.render_chat_input(frame, screen_area);
        }
    }

    /// Render the game map
    fn render_map(&self, frame: &mut Frame, ctx: &RenderContext, area: Rect) {
        // Determine border color based on focus
        let map_border = if self.focused_zone == SidebarZone::Map {
            Color::Yellow
        } else {
            Color::Cyan
        };

        // Render the map container
        let map_block = Block::default()
            .title(" Map ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(map_border));

        let inner_area = map_block.inner(area);
        frame.render_widget(map_block, area);

        let view_width = inner_area.width as usize;
        let view_height = (inner_area.height as usize).saturating_sub(1); // Leave room for HUD

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
                        Rect::new(inner_area.x + x as u16, inner_area.y + y as u16, 1, 1),
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

                // Render username above player if space allows
                if screen_y > 0 {
                    let name_display = if remote.username.len() > 10 {
                        &remote.username[..10]
                    } else {
                        &remote.username
                    };
                    frame.render_widget(
                        Paragraph::new(name_display).style(Style::default().fg(Color::Magenta)),
                        Rect::new(
                            inner_area.x + screen_x as u16,
                            inner_area.y + (screen_y - 1) as u16,
                            name_display.len() as u16,
                            1,
                        ),
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
                Rect::new(
                    inner_area.x + screen_player_x as u16,
                    inner_area.y + screen_player_y as u16,
                    1,
                    1,
                ),
            );
        }

        // --- RENDER HUD ---
        // Build status info spans
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

        // Build hints based on current state (show only relevant hints)
        let mut hints: Vec<Vec<Span<'static>>> = vec![
            status(&format!(" {} ", self.username)),
            info(&online_text, online_color),
        ];

        // Context-sensitive hints
        if self.chat_mode {
            hints.push(info("[CHAT MODE]", Color::Cyan));
        } else {
            hints.push(hint_key("Enter", "chat"));
        }

        // Always show these hints
        hints.push(hint_key("Tab", focus_text));
        hints.push(hint_key("c", cam_text));
        hints.push(hint("quit"));
        hints.push(info(
            &format!("({},{})", ctx.player.x, ctx.player.y),
            Color::DarkGray,
        ));

        let hud_line = hints_line_piped(&hints);
        let hud_paragraph = Paragraph::new(hud_line).alignment(Alignment::Center);

        let hud_area = Rect::new(
            inner_area.x,
            inner_area.y + inner_area.height,
            inner_area.width,
            1,
        );
        frame.render_widget(hud_paragraph, hud_area);
    }

    /// Render the sidebar with tabbed player list and chat
    fn render_sidebar(&mut self, frame: &mut Frame, ctx: &RenderContext, area: Rect) {
        // Split sidebar into 2 zones: Player List (with tabs) and Chat
        let sidebar_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40), // Zone A: Player List with tabs
                Constraint::Percentage(60), // Zone B: Chat Log
            ])
            .split(area);

        let player_list_area = sidebar_layout[0];
        let chat_area = sidebar_layout[1];

        // Determine border colors based on focus
        let player_border = if self.focused_zone == SidebarZone::PlayerList {
            Color::Yellow
        } else {
            Color::DarkGray
        };
        let chat_border = if self.focused_zone == SidebarZone::ChatLog {
            Color::Yellow
        } else {
            Color::Blue
        };

        // --- ZONE A: Player List with Tabs ---
        // Calculate counts for tab titles
        let world_count = ctx.remote_players.len();
        let my_x = ctx.player.x as i32;
        let my_y = ctx.player.y as i32;
        let nearby_count = ctx
            .remote_players
            .values()
            .filter(|p| {
                let dist = (p.x - my_x).abs() + (p.y - my_y).abs();
                dist <= PROXIMITY_DISTANCE
            })
            .count();

        // Split player list area into tabs header and list content
        let player_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2), // Tabs header
                Constraint::Min(3),    // List content
            ])
            .split(player_list_area);

        let tabs_area = player_layout[0];
        let list_area = player_layout[1];

        // Render tabs
        let tab_titles = vec![
            format!(" World ({}) ", world_count),
            format!(" Nearby ({}) ", nearby_count),
        ];
        let selected_tab = match self.player_tab {
            PlayerTab::World => 0,
            PlayerTab::Nearby => 1,
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

        // Render tabs with border
        let tabs_block = Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
            .border_style(Style::default().fg(player_border));
        frame.render_widget(tabs_block, tabs_area);
        frame.render_widget(
            tabs,
            tabs_area.inner(ratatui::layout::Margin {
                horizontal: 1,
                vertical: 0,
            }),
        );

        // Build player list based on current tab
        let (players, list_count): (Vec<ListItem>, usize) = match self.player_tab {
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
                    .filter(|p| {
                        let dist = (p.x - my_x).abs() + (p.y - my_y).abs();
                        dist <= PROXIMITY_DISTANCE
                    })
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

        let player_list = List::new(players)
            .block(
                Block::default()
                    .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                    .border_style(Style::default().fg(player_border)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        frame.render_stateful_widget(player_list, list_area, &mut self.player_list_state);

        // Scrollbar for player list
        let player_scroll_pos = self.player_list_state.selected().unwrap_or(0);
        let mut player_scrollbar_state =
            ScrollbarState::new(list_count.max(1)).position(player_scroll_pos);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼")),
            list_area.inner(ratatui::layout::Margin {
                horizontal: 0,
                vertical: 1,
            }),
            &mut player_scrollbar_state,
        );

        // --- ZONE B: Chat Log ---
        // Auto-scroll to bottom when new messages arrive
        let chat_len = ctx.chat_log.len();
        if chat_len > self.last_chat_len && self.focused_zone != SidebarZone::ChatLog {
            // New message arrived, scroll to bottom
            if chat_len > 0 {
                self.chat_list_state.select(Some(chat_len - 1));
            }
        }
        self.last_chat_len = chat_len;

        let chat_messages: Vec<ListItem> = ctx
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
                    "*".to_string()
                } else if entry.is_proximity {
                    ">".to_string()
                } else {
                    " ".to_string()
                };

                ListItem::new(format!("{} {}: {}", prefix, entry.username, entry.message))
                    .style(style)
            })
            .collect();

        let chat_list = List::new(if chat_messages.is_empty() {
            vec![ListItem::new("  (no messages)").style(Style::default().fg(Color::DarkGray))]
        } else {
            chat_messages
        })
        .block(
            Block::default()
                .title(" Chat ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(chat_border)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        frame.render_stateful_widget(chat_list, chat_area, &mut self.chat_list_state);

        // Scrollbar for chat
        let chat_scroll_pos = self.chat_list_state.selected().unwrap_or(0);
        let mut chat_scrollbar_state =
            ScrollbarState::new(chat_len.max(1)).position(chat_scroll_pos);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼")),
            chat_area.inner(ratatui::layout::Margin {
                horizontal: 0,
                vertical: 1,
            }),
            &mut chat_scrollbar_state,
        );
    }

    /// Render the chat input overlay
    fn render_chat_input(&self, frame: &mut Frame, area: Rect) {
        let input_height = 3;
        let input_area = Rect::new(
            0,
            area.height.saturating_sub(input_height + 1),
            area.width,
            input_height,
        );

        // Clear the area first
        frame.render_widget(
            Block::default().style(Style::default().bg(Color::Black)),
            input_area,
        );

        let input_text = format!(" > {}_", self.chat_buffer);
        let input_widget = Paragraph::new(input_text)
            .block(
                Block::default()
                    .title(" Chat (Enter to send, Esc to cancel) ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: false });

        frame.render_widget(input_widget, input_area);
    }

    /// Handle input specific to the world page
    pub fn handle_input(&mut self, key: KeyCode, ctx: &mut UpdateContext) -> PageAction {
        if self.chat_mode {
            return self.handle_chat_input(key);
        }

        match key {
            KeyCode::Char('q') => PageAction::Quit,
            KeyCode::Esc => PageAction::Quit,

            // Enter chat mode
            KeyCode::Enter => {
                self.chat_mode = true;
                PageAction::None
            }

            // Cycle Focus
            KeyCode::Tab => {
                self.focused_zone = match self.focused_zone {
                    SidebarZone::Map => SidebarZone::PlayerList,
                    SidebarZone::PlayerList => SidebarZone::ChatLog,
                    SidebarZone::ChatLog => SidebarZone::Map,
                };
                PageAction::None
            }

            // Toggle Camera
            KeyCode::Char('c') => {
                self.camera_type = match self.camera_type {
                    CameraType::Follow => CameraType::Page,
                    CameraType::Page => CameraType::Follow,
                };
                PageAction::None
            }

            // Movement / Scrolling
            KeyCode::Up => {
                match self.focused_zone {
                    SidebarZone::Map => {
                        if ctx.player.try_move(0, -1, ctx.map) {
                            if let Some(send) = ctx.send_move {
                                send(0, -1);
                            }
                        }
                    }
                    SidebarZone::PlayerList => Self::scroll_list(&mut self.player_list_state, -1),
                    SidebarZone::ChatLog => Self::scroll_list(&mut self.chat_list_state, -1),
                }
                PageAction::None
            }
            KeyCode::Down => {
                match self.focused_zone {
                    SidebarZone::Map => {
                        if ctx.player.try_move(0, 1, ctx.map) {
                            if let Some(send) = ctx.send_move {
                                send(0, 1);
                            }
                        }
                    }
                    SidebarZone::PlayerList => Self::scroll_list(&mut self.player_list_state, 1),
                    SidebarZone::ChatLog => Self::scroll_list(&mut self.chat_list_state, 1),
                }
                PageAction::None
            }
            KeyCode::Left => {
                match self.focused_zone {
                    SidebarZone::Map => {
                        if ctx.player.try_move(-1, 0, ctx.map) {
                            if let Some(send) = ctx.send_move {
                                send(-1, 0);
                            }
                        }
                    }
                    SidebarZone::PlayerList => {
                        // Switch player tab
                        self.player_tab = match self.player_tab {
                            PlayerTab::World => PlayerTab::Nearby,
                            PlayerTab::Nearby => PlayerTab::World,
                        };
                        self.player_list_state.select(Some(0)); // Reset scroll on tab switch
                    }
                    SidebarZone::ChatLog => {}
                }
                PageAction::None
            }
            KeyCode::Right => {
                match self.focused_zone {
                    SidebarZone::Map => {
                        if ctx.player.try_move(1, 0, ctx.map) {
                            if let Some(send) = ctx.send_move {
                                send(1, 0);
                            }
                        }
                    }
                    SidebarZone::PlayerList => {
                        // Switch player tab
                        self.player_tab = match self.player_tab {
                            PlayerTab::World => PlayerTab::Nearby,
                            PlayerTab::Nearby => PlayerTab::World,
                        };
                        self.player_list_state.select(Some(0)); // Reset scroll on tab switch
                    }
                    SidebarZone::ChatLog => {}
                }
                PageAction::None
            }

            // WASD always moves player
            KeyCode::Char('w') => {
                if ctx.player.try_move(0, -1, ctx.map) {
                    if let Some(send) = ctx.send_move {
                        send(0, -1);
                    }
                }
                PageAction::None
            }
            KeyCode::Char('s') => {
                if ctx.player.try_move(0, 1, ctx.map) {
                    if let Some(send) = ctx.send_move {
                        send(0, 1);
                    }
                }
                PageAction::None
            }
            KeyCode::Char('a') => {
                if ctx.player.try_move(-1, 0, ctx.map) {
                    if let Some(send) = ctx.send_move {
                        send(-1, 0);
                    }
                }
                PageAction::None
            }
            KeyCode::Char('d') => {
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

    fn scroll_list(state: &mut ListState, amount: i32) {
        let i = state.selected().unwrap_or(0);
        if amount > 0 {
            state.select(Some(i.saturating_add(amount as usize)));
        } else {
            state.select(Some(i.saturating_sub(amount.abs() as usize)));
        }
    }

    /// Handle input when in chat mode
    fn handle_chat_input(&mut self, key: KeyCode) -> PageAction {
        match key {
            KeyCode::Esc => {
                // Cancel chat
                self.chat_mode = false;
                self.chat_buffer.clear();
                PageAction::None
            }
            KeyCode::Enter => {
                // Send message if not empty
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
                if self.chat_buffer.len() < 200 {
                    self.chat_buffer.push(c);
                }
                PageAction::None
            }
            _ => PageAction::None,
        }
    }
}
