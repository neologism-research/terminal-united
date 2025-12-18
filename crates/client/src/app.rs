// src/app.rs
//
// Main application struct - manages the game loop and delegates to pages.
// The App owns shared game state and the current page.

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{Terminal, backend::Backend};
use std::collections::HashMap;
use std::io;

use crate::map::Map;
use crate::network::{NetworkClient, RemotePlayer};
use crate::pages::{PageAction, PageState, RenderContext, UpdateContext, WorldPage, RoomSelectPage, LoginPage};
use crate::player::Player;

/// Server URL for the game server
// const SERVER_URL: &str = "ws://localhost:3000";
const SERVER_URL: &str = "wss://j62zf3m1-3000.asse.devtunnels.ms";

pub struct App {
    /// Whether the application should quit
    should_quit: bool,
    /// The game map (shared state)
    map: Map,
    /// The player (shared state)
    player: Player,
    /// Current page state
    page: PageState,
    /// Network client (None if not connected)
    network: Option<NetworkClient>,
    /// Tokio runtime for async networking
    runtime: tokio::runtime::Runtime,
    /// Cached remote players for rendering
    remote_players: HashMap<String, RemotePlayer>,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            map: Map::load(),
            player: Player::new(5, 5), // Spawn point
            page: PageState::initial(),
            network: None,
            runtime: tokio::runtime::Runtime::new().expect("Failed to create tokio runtime"),
            remote_players: HashMap::new(),
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        while !self.should_quit {
            // Update remote players from network
            self.sync_remote_players();

            // Render current page
            terminal.draw(|frame| {
                let ctx = RenderContext {
                    map: &self.map,
                    player: &self.player,
                    remote_players: &self.remote_players,
                    is_connected: self.network.is_some(),
                };
                self.page.render(frame, &ctx);
            })?;

            // Handle input
            if event::poll(std::time::Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        // Create a closure for sending moves
                        let network_ref = &self.network;
                        let send_move_fn = |dx: i32, dy: i32| {
                            if let Some(net) = network_ref {
                                net.send_move(dx, dy);
                            }
                        };

                        let mut ctx = UpdateContext {
                            map: &self.map,
                            player: &mut self.player,
                            send_move: if self.network.is_some() {
                                Some(&send_move_fn)
                            } else {
                                None
                            },
                        };

                        let action = self.page.handle_input(key.code, &mut ctx);
                        self.handle_page_action(action);
                    }
                }
            }
        }
        Ok(())
    }

    /// Sync remote players from network client
    fn sync_remote_players(&mut self) {
        if let Some(network) = &self.network {
            self.remote_players = self.runtime.block_on(async {
                network.players.lock().await.clone()
            });
        }
    }

    /// Process actions returned by pages
    fn handle_page_action(&mut self, action: PageAction) {
        match action {
            PageAction::None => {}
            PageAction::Transition(new_page) => {
                self.page = new_page;
            }
            PageAction::Quit => {
                self.should_quit = true;
            }
            PageAction::GoToRoomSelect { username } => {
                self.page = PageState::RoomSelect(RoomSelectPage::new(username));
            }
            PageAction::JoinRoom { username, room } => {
                self.handle_join_room(username, room);
            }
            PageAction::BackToLogin => {
                // Disconnect if connected
                self.network = None;
                self.remote_players.clear();
                self.page = PageState::Login(LoginPage::new());
            }
        }
    }

    /// Handle join room request from room select page
    fn handle_join_room(&mut self, username: String, room: String) {
        // Update status on room select page
        if let PageState::RoomSelect(ref mut room_select) = self.page {
            room_select.set_status(format!("Joining {}...", room));
        }

        // Try to connect
        let result = self.runtime.block_on(async {
            NetworkClient::connect(SERVER_URL, &username, &room).await
        });

        match result {
            Ok(client) => {
                self.network = Some(client);
                self.page = PageState::World(WorldPage::new(username));
            }
            Err(e) => {
                // Show error on room select page
                if let PageState::RoomSelect(ref mut room_select) = self.page {
                    room_select.set_status(format!("Failed: {}", e));
                }
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
