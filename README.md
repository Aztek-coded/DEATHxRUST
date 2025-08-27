# Death Bot 🤖

A Discord bot built with Rust using the Serenity framework, featuring a modular architecture for maintainability and scalability.

## Features

- **Modular Architecture**: Clean separation of concerns with dedicated modules
- **Command System**: Extensible command handling with ping and help commands
- **Error Handling**: Comprehensive error types and handling
- **Configuration Management**: Environment-based configuration
- **Type Safety**: Full Rust type safety and memory safety

## Quick Start

### Prerequisites

- Rust 1.70+ 
- A Discord bot token

### Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd DEATHxRUST
   ```

2. Copy the environment template:
   ```bash
   cp .env.example .env
   ```

3. Add your Discord bot token to `.env`:
   ```
   DISCORD_TOKEN=your_actual_token_here
   ```

4. Run the bot:
   ```bash
   cargo run
   ```

## Commands

- `!ping` - Test bot responsiveness
- `!help` - Show available commands

## Project Structure

```
src/
├── main.rs              # Application entry point
├── bot/                 # Bot client and configuration
│   ├── client.rs        # Discord client setup
│   └── intents.rs       # Gateway intents configuration
├── handlers/            # Event handling
│   ├── event_handler.rs # Main event handler
│   ├── message.rs       # Message event handling
│   └── ready.rs         # Bot ready event
├── commands/            # Command implementations
│   ├── ping.rs          # Ping command
│   └── help.rs          # Help command
├── config/              # Configuration management
│   └── settings.rs      # Environment settings
└── utils/               # Utilities and error handling
    └── error.rs         # Custom error types
```

## Adding New Commands

1. Create a new file in `src/commands/` (e.g., `mycommand.rs`)
2. Implement the `execute` function with the signature:
   ```rust
   pub async fn execute(ctx: &Context, msg: &Message) {
       // Command logic here
   }
   ```
3. Add the module to `src/commands/mod.rs`
4. Add the command trigger in `src/handlers/message.rs`

## Development

### Building
```bash
cargo build
```

### Running tests
```bash
cargo test
```

### Formatting code
```bash
cargo fmt
```

### Checking code
```bash
cargo clippy
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run `cargo fmt` and `cargo clippy`
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.