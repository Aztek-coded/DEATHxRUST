# Discord Bot Makefile
# ====================

.PHONY: help build run deploy deploy-global deploy-guild clean test

# Default target
help:
	@echo "Discord Bot Makefile"
	@echo "===================="
	@echo ""
	@echo "Available targets:"
	@echo "  build         - Build the bot in release mode"
	@echo "  run           - Run the bot"
	@echo "  dev           - Run in development mode"
	@echo "  watch         - Watch for changes and auto-restart"
	@echo "  watch-clear   - Watch for changes with clear screen"
	@echo "  deploy        - Deploy slash commands (uses env config)"
	@echo "  deploy-global - Deploy slash commands globally"
	@echo "  deploy-guild  - Deploy to development guild"
	@echo "  clean         - Clean build artifacts"
	@echo "  test          - Run tests"
	@echo ""
	@echo "Examples:"
	@echo "  make watch        - Start development with auto-restart"
	@echo "  make deploy-global"
	@echo "  make deploy-guild GUILD_ID=123456789"
	@echo "  make run"

# Build the project
build:
	@echo "🔨 Building project..."
	cargo build --release

# Run the bot
run:
	@echo "🚀 Starting bot..."
	cargo run --release

# Deploy commands using environment configuration
deploy:
	@echo "📡 Deploying slash commands..."
	./scripts/deploy.sh

# Deploy commands globally
deploy-global:
	@echo "🌍 Deploying slash commands globally..."
	./scripts/deploy.sh --global

# Deploy commands to specific guild
deploy-guild:
	@echo "🏰 Deploying slash commands to guild..."
ifdef GUILD_ID
	./scripts/deploy.sh --guild $(GUILD_ID)
else
	@echo "❌ Please specify GUILD_ID: make deploy-guild GUILD_ID=123456789"
	@exit 1
endif

# Deploy with dry run
deploy-dry:
	@echo "🔍 Dry run deployment..."
	./scripts/deploy.sh --dry-run

deploy-global-dry:
	@echo "🔍 Dry run global deployment..."
	./scripts/deploy.sh --global --dry-run

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# Run tests
test:
	@echo "🧪 Running tests..."
	cargo test

# Check code without building
check:
	@echo "✅ Checking code..."
	cargo check

# Format code
fmt:
	@echo "🎨 Formatting code..."
	cargo fmt

# Lint code
lint:
	@echo "📝 Linting code..."
	cargo clippy

# Install dependencies
install:
	@echo "📦 Installing dependencies..."
	cargo build

# Development build and run
dev:
	@echo "🛠️ Running in development mode..."
	cargo run

# Watch for changes and auto-restart
watch:
	@echo "👀 Watching for changes and auto-restarting..."
	@echo "Installing cargo-watch if not present..."
	@cargo install cargo-watch 2>/dev/null || true
	cargo watch -x run

# Watch with clear screen on restart
watch-clear:
	@echo "👀 Watching for changes (with clear screen)..."
	@echo "Installing cargo-watch if not present..."
	@cargo install cargo-watch 2>/dev/null || true
	cargo watch -c -x run

# Show environment variables
env:
	@echo "Environment variables:"
	@echo "DISCORD_TOKEN: $(if $(DISCORD_TOKEN),set,not set)"
	@echo "DEVELOPMENT_GUILD_ID: $(DEVELOPMENT_GUILD_ID)"
	@echo "COMMAND_PREFIX: $(COMMAND_PREFIX)"
	@echo "SLASH_COMMANDS_GLOBAL: $(SLASH_COMMANDS_GLOBAL)"
	@echo "AUTO_SYNC_COMMANDS: $(AUTO_SYNC_COMMANDS)"