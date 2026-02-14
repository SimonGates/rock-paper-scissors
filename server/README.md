# Rock Paper Scissors Server

A WebSocket-based Rock-Paper-Scissors game server built with Rust, featuring asynchronous I/O and real-time bidirectional communication.

## Overview

This project implements a TCP/WebSocket server that allows clients to play Rock-Paper-Scissors against a computer opponent. Each game session maintains a score based on wins and losses, with the server handling multiple concurrent connections through Rust's async runtime.

## Features

- **Asynchronous I/O**: Built on Tokio for high-performance async operations
- **WebSocket Protocol**: Real-time bidirectional communication using `tokio-tungstenite`
- **Concurrent Connections**: Multiple clients can connect and play simultaneously
- **Persistent Score Tracking**: Score maintained throughout each game session
- **JSON Communication**: Structured request/response protocol
- **Structured Logging**: Configurable logging using `env_logger`

## Game Rules

- **Win**: +3 points (Rock beats Scissors, Paper beats Rock, Scissors beats Paper)
- **Lose**: -1 point
- **Draw**: No score change
- Score uses saturation arithmetic to prevent overflow/underflow

## Getting Started

### Prerequisites

- Rust 2024 edition or later
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/SimonGates/rust-socket.git
cd rust-sockets
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run
```

The server will start on `localhost:6767` by default.

### Configuration

Set the `RUST_LOG` environment variable to control logging level:

```bash
RUST_LOG=info cargo run
```

Available log levels: `error`, `warn`, `info`, `debug`, `trace`

## Protocol

### Connection

Connect to the WebSocket server at `ws://localhost:6767`.

### Message Format

All messages use JSON format.

#### Client Request

Send a JSON message with your move:

```json
{"Payload": "Rock"}
```

Valid options: `"Rock"`, `"Paper"`, `"Scissors"`

#### Server Response (Success)

```json
{
  "Result": {
    "turn_result": "Win",
    "game": {
      "score": 3
    }
  }
}
```

- `turn_result`: `"Win"`, `"Lose"`, or `"Draw"`
- `game.score`: Current score for the session

#### Server Response (Error)

```json
{
  "Error": "InvalidRequest"
}
```

Error codes:
- `InvalidRequest` (400): Malformed request or invalid move
- `InternalServerError` (500): Server-side error

## Example Client

Here's a simple JavaScript WebSocket client:

```javascript
const ws = new WebSocket('ws://localhost:6767');

ws.onopen = () => {
  console.log('Connected to server');
  // Send a move
  ws.send(JSON.stringify({Payload: "Rock"}));
};

ws.onmessage = (event) => {
  console.log('Received:', event.data);
  const data = JSON.parse(event.data);
  
  if (data.Result) {
    console.log(`Result: ${data.Result.turn_result}`);
    console.log(`Score: ${data.Result.game.score}`);
  } else if (data.Error) {
    console.error(`Error: ${data.Error}`);
  }
};

ws.onerror = (error) => {
  console.error('WebSocket error:', error);
};

ws.onclose = () => {
  console.log('Disconnected from server');
};
```

## Project Structure

```
rust-sockets/
├── src/
│   ├── main.rs                    # Application entry point
│   ├── server.rs                  # WebSocket server implementation
│   └── rock_paper_scissors.rs     # Game logic and rules
├── Cargo.toml                     # Project manifest
├── Cargo.lock                     # Dependency lock file
└── README.md                      # This file
```

## Architecture

The project follows a modular design with clear separation of concerns:

- **main.rs**: Initializes the logger and starts the server
- **server.rs**: Handles WebSocket connections, message parsing, and server lifecycle
- **rock_paper_scissors.rs**: Contains pure game logic, score tracking, and random move generation

Each WebSocket connection spawns a separate async task, allowing the server to handle multiple clients concurrently without blocking.

## Dependencies

- **tokio** (v1.49.0): Async runtime and TCP networking
- **tokio-tungstenite** (v0.28.0): WebSocket protocol implementation
- **futures-util** (v0.3.31): Stream and Sink traits for async I/O
- **serde** (v1.0.228): Serialization framework
- **serde_json** (v1.0.149): JSON support
- **rand** (v0.10.0): Random number generation for computer moves
- **log** (v0.4.29): Logging facade
- **env_logger** (v0.11.8): Logger implementation

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy
```

### Formatting

```bash
cargo fmt
```

## License

This project is available for use under an open source license.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Authors

- Simon Gates

## Acknowledgments

Built with Rust's excellent async ecosystem, including Tokio and tungstenite.
