# Terminal United 🎮

A multiplayer terminal-based game where players explore a shared world, chat with nearby players, and interact in real-time — all from your terminal.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)

## Features

- 🗺️ **Explore a 400x400 tile world** - Navigate through walls, grass, water, and more
- 👥 **Real-time multiplayer** - See other players moving around in real-time via WebSocket
- 💬 **Proximity chat** - Messages from nearby players are highlighted
- 🏠 **Multiple rooms** - Join different rooms (world, arena, dungeon, tavern)
- 📷 **Camera modes** - Toggle between follow and page-based camera
- ⌨️ **Keyboard-driven UI** - Full keyboard navigation with btop-style hints

## Screenshots

TODO

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/terminal-united.git
cd terminal-united

# Build the project
cargo build --release

# Run the client
./target/release/terminal-united

# Run the server (in a separate terminal)
./target/release/terminal-united-server
```

### Pre-built Binaries

TODO

## Usage

### Client

```bash
# Connect to the default server
terminal-united

# Or specify a custom server
terminal-united --server wss://your-server.com
```

### Controls

| Key                    | Action                             |
| ---------------------- | ---------------------------------- |
| `W/A/S/D` or `↑/↓/←/→` | Move player                        |
| `Enter`                | Open chat                          |
| `Tab`                  | Cycle focus (Map → Players → Chat) |
| `C`                    | Toggle camera mode                 |
| `Q` or `Esc`           | Quit                               |

### Server

```bash
# Start the server on default port 3000
terminal-united-server

# The server exposes:
# - WebSocket endpoint: ws://localhost:3000/ws
# - Version endpoint: http://localhost:3000/version
```

## Project Structure

```
terminal-united/
├── crates/
│   ├── client/          # TUI game client
│   │   ├── src/
│   │   │   ├── app.rs       # Main application loop
│   │   │   ├── map.rs       # Map loading and tile management
│   │   │   ├── player.rs    # Player entity
│   │   │   ├── network/     # WebSocket client
│   │   │   └── pages/       # UI pages (login, room select, world)
│   │   └── assets/
│   │       └── world_map.txt
│   ├── server/          # Game server
│   │   └── src/
│   │       ├── main.rs      # Server entry point
│   │       └── room.rs      # Room management
│   └── shared/          # Shared types and constants
│       └── src/
│           └── lib.rs
└── Cargo.toml           # Workspace configuration
```

## Configuration

TODO

## Development

### Prerequisites

- Rust 1.70 or later
- A terminal with Unicode support

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test --workspace

# Check for issues
cargo clippy --workspace
```

### Running Locally

```bash
# Terminal 1: Start the server
cargo run -p terminal-united-server

# Terminal 2: Run the client
cargo run -p terminal-united
```

## Architecture

The project uses a client-server architecture:

- **Client**: Built with [Ratatui](https://github.com/ratatui-org/ratatui) for the TUI and [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite) for WebSocket communication
- **Server**: Built with [Axum](https://github.com/tokio-rs/axum) for HTTP/WebSocket handling and [DashMap](https://github.com/xacrimon/dashmap) for concurrent state management
- **Shared**: Common types and constants used by both client and server

### Protocol

The client and server communicate via JSON messages over WebSocket:

**Client → Server:**

- `Join { username, room }` - Join a room
- `Move { dx, dy }` - Move player
- `Chat { message }` - Send chat message
- `Leave` - Leave the room

**Server → Client:**

- `Init { session_id, players }` - Initial state on join
- `PlayerJoined { player }` - New player joined
- `PlayerMoved { session_id, x, y }` - Player moved
- `PlayerLeft { session_id }` - Player left
- `ChatMessage { username, message, x, y }` - Chat message
- `Error { message }` - Error message

## Roadmap

TODO

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI library
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Tokio](https://tokio.rs/) - Async runtime

---

Made with ❤️ and Rust
