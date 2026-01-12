use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{Terminal, backend::Backend};
use std::collections::HashMap;
use std::io;
use terminal_united_shared::constants::VERSION;

use crate::map::Map;
use crate::network::{ChatEntry, NetworkClient, RemotePlayer};
use crate::pages::modes::WorldMode;
use crate::pages::{GameLayout, GameMode, PageAction, PageState, RenderContext, UpdateContext};
use crate::player::Player;

// const SERVER_URL: &str = "wss://j62zf3m1-3000.asse.devtunnels.ms";
const SERVER_URL: &str = "ws://localhost:8080";
// const SERVER_URL: &str = "wss://terminal-united.neologism.cc";
// const SERVER_URL: &str = "wss://shark-app-oh6d4.ondigitalocean.app";

pub struct App {
    should_quit: bool,
    map: Map,
    player: Player,
    page: PageState,
    network: Option<NetworkClient>,
    runtime: tokio::runtime::Runtime,
    remote_players: HashMap<String, RemotePlayer>,
    chat_log: Vec<ChatEntry>,
    update_available: Option<String>,
    username: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            map: Map::default(),
            player: Player::default(),
            page: PageState::initial(),
            network: None,
            runtime: tokio::runtime::Runtime::new().expect("Failed to create tokio runtime"),
            remote_players: HashMap::new(),
            chat_log: Vec::new(),
            update_available: None,
            username: String::new(),
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
                    username: &self.username,
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

    fn sync_network_state(&mut self) {
        if let Some(network) = &self.network {
            network.update_local_pos(self.player.x as i32, self.player.y as i32, &self.runtime);
            self.remote_players = self
                .runtime
                .block_on(async { network.players.lock().await.clone() });
            self.chat_log = self
                .runtime
                .block_on(async { network.chat_log.lock().await.clone() });
        }
    }

    fn handle_page_action(&mut self, action: PageAction) {
        match action {
            PageAction::None => {}
            PageAction::Quit => self.should_quit = true,
            PageAction::JoinWorld { username } => {
                self.handle_join_world(username);
            }
            PageAction::SendChat { message } => {
                if let Some(network) = &self.network {
                    network.send_chat(message);
                }
            }
        }
    }

    fn handle_join_world(&mut self, username: String) {
        if let PageState::Login(ref mut login_page) = self.page {
            login_page.status = Some("Joining world...".to_string());
        }

        let result = self
            .runtime
            .block_on(async { NetworkClient::connect(SERVER_URL, &username).await });

        match result {
            Ok(client) => {
                self.username = username.clone();
                self.network = Some(client);
                let mode = GameMode::World(WorldMode::new());
                self.page = PageState::InGame(GameLayout::new(mode));
            }
            Err(e) => {
                if let PageState::Login(ref mut login_page) = self.page {
                    login_page.status = Some(format!("Failed: {}", e));
                }
            }
        }
    }

    fn check_version(&mut self) {
        let url = SERVER_URL
            .replace("wss://", "https://")
            .replace("ws://", "http://")
            + "/version";

        let result: Result<serde_json::Value, _> = self.runtime.block_on(async {
            reqwest::Client::new()
                .get(&url)
                .send()
                .await?
                .json::<serde_json::Value>()
                .await
        });

        if let Ok(json) = result {
            if let Some(server_version) = json.get("version").and_then(|v| v.as_str()) {
                if server_version != VERSION {
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
