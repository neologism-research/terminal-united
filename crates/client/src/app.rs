// src/app.rs
//
// Main application struct - manages the game loop and delegates to pages.
// The App owns shared game state and the current page.

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{Terminal, backend::Backend};
use std::collections::HashMap;
use std::io;

use crate::map::Map;
use crate::network::{ChatEntry, NetworkClient, RemotePlayer};
use crate::pages::{
    LoginPage, PageAction, PageState, RenderContext, RoomSelectPage, UpdateContext, WorldPage,
};
use crate::player::Player;

/// Server URL for the game server
// const SERVER_URL: &str = "ws://localhost:3000";
const SERVER_URL: &str = "wss://j62zf3m1-3000.asse.devtunnels.ms";
const CLIENT_VERSION: &str = "0.1.0";

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
    /// Cached chat log for rendering
    chat_log: Vec<ChatEntry>,
    /// Whether a new version is available
    update_available: Option<String>,
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
            chat_log: Vec::new(),
            update_available: None,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        // Check for updates on startup
        self.check_version();

        while !self.should_quit {
            // Update remote players and chat from network
            self.sync_network_state();

            // Render current page
            terminal.draw(|frame| {
                let ctx = RenderContext {
                    map: &self.map,
                    player: &self.player,
                    remote_players: &self.remote_players,
                    chat_log: &self.chat_log,
                    is_connected: self.network.is_some(),
                    update_available: self.update_available.as_ref(),
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

    /// Sync remote players and chat log from network client
    fn sync_network_state(&mut self) {
        if let Some(network) = &self.network {
            // Update local position for proximity calculations
            network.update_local_pos(self.player.x as i32, self.player.y as i32, &self.runtime);

            // Sync remote players
            self.remote_players = self
                .runtime
                .block_on(async { network.players.lock().await.clone() });

            // Sync chat log
            self.chat_log = self
                .runtime
                .block_on(async { network.chat_log.lock().await.clone() });
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
                self.chat_log.clear();
                self.page = PageState::Login(LoginPage::new());
            }
            PageAction::SendChat { message } => {
                if let Some(network) = &self.network {
                    network.send_chat(message);
                }
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
        let result = self
            .runtime
            .block_on(async { NetworkClient::connect(SERVER_URL, &username, &room).await });

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

    /// Check for updates from the server
    fn check_version(&mut self) {
        let url = SERVER_URL
            .replace("wss://", "https://")
            .replace("ws://", "http://")
            + "/version";

        let result: Result<serde_json::Value, reqwest::Error> = self.runtime.block_on(async {
            let client = reqwest::Client::new();
            let resp = client
                .get(&url)
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;
            Ok(resp)
        });

        if let Ok(json) = result {
            if let Some(server_version) = json.get("version").and_then(|v| v.as_str()) {
                if server_version != CLIENT_VERSION {
                    self.update_available = Some(server_version.to_string());
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
