# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Building and Running
- `cargo run` - Run the bot in development mode
- `cargo build --release` - Build optimized release version
- `make run` - Run the bot using Makefile (release build)
- `make dev` - Run in development mode
- `make watch` - Watch for changes and auto-restart (requires cargo-watch)

### Code Quality
- `cargo test` - Run all tests
- `cargo fmt` - Format code
- `cargo clippy` - Run linting
- `make test` - Run tests via Makefile
- `make fmt` - Format code via Makefile
- `make lint` - Run linting via Makefile

### Slash Command Deployment
- `make deploy` - Deploy commands using environment config
- `make deploy-global` - Deploy commands globally
- `make deploy-guild GUILD_ID=123456789` - Deploy to specific guild
- `make deploy-dry` - Dry run deployment

## Architecture Overview

This is a Discord bot built with Rust using the **Poise** framework (built on top of Serenity). The architecture follows a modular design:

### Framework Choice
- **Poise**: Modern command framework providing both slash commands and prefix commands
- **Serenity**: Lower-level Discord API wrapper
- Uses async/await throughout with Tokio runtime

### Key Modules
- `bot/` - Bot client setup and Poise framework configuration
- `commands/` - Individual command implementations (ping, help, info)
- `config/` - Environment-based settings management
- `handlers/` - Event handling logic
- `utils/` - Error handling and common utilities

### Configuration System
Settings are loaded from environment variables via `Settings::from_env()`:
- `DISCORD_TOKEN` (required) - Bot token
- `COMMAND_PREFIX` (default: "!") - Prefix for text commands
- `DEVELOPMENT_GUILD_ID` - Guild ID for development command registration
- `AUTO_SYNC_COMMANDS` (default: false) - Auto-sync commands to guild
- `SLASH_COMMANDS_GLOBAL` (default: false) - Deploy commands globally vs guild-only

### Command Registration
The bot supports both slash commands and prefix commands. Commands are registered in `bot/framework.rs` and can be deployed globally or to specific guilds based on configuration.

### Adding New Commands
1. Create new file in `src/commands/` 
2. Implement as Poise command function with `#[poise::command(...)]` macro
3. Add to command list in `bot/framework.rs`
4. Export from `commands/mod.rs`

### Error Handling
Custom error types defined in `utils/error.rs` with comprehensive error handling in the Poise framework setup.

### Environment Setup
Requires `.env` file with `DISCORD_TOKEN`. The bot uses `dotenv` to load environment variables on startup.