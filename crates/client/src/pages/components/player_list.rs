use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    widgets::{
        Block, Borders, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Tabs,
    },
};
use terminal_united_shared::PROXIMITY_DISTANCE;

use crate::network::RemotePlayer;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
enum PlayerTab {
    World,
    Nearby,
}

impl PlayerTab {
    fn toggle(self) -> Self {
        match self {
            PlayerTab::World => PlayerTab::Nearby,
            PlayerTab::Nearby => PlayerTab::World,
        }
    }
}

pub struct PlayerListWidget {
    player_tab: PlayerTab,
    list_state: ListState,
    is_focused: bool,
}

impl PlayerListWidget {
    pub fn new() -> Self {
        Self {
            player_tab: PlayerTab::World,
            list_state: ListState::default(),
            is_focused: false,
        }
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    pub fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        remote_players: &HashMap<String, RemotePlayer>,
        my_x: i32,
        my_y: i32,
    ) {
        let border_color = if self.is_focused {
            Color::Yellow
        } else {
            Color::DarkGray
        };

        let world_count = remote_players.len();
        let nearby_count = remote_players
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
                let items: Vec<ListItem> = remote_players
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
                let items: Vec<ListItem> = remote_players
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

        frame.render_stateful_widget(list, player_layout[1], &mut self.list_state);

        // Scrollbar
        let scroll_pos = self.list_state.selected().unwrap_or(0);
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

    pub fn handle_input(&mut self, dx: i32, dy: i32) {
        if dx != 0 {
            self.player_tab = self.player_tab.toggle();
            self.list_state.select(Some(0));
        } else if dy != 0 {
            Self::scroll_list(&mut self.list_state, dy);
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
