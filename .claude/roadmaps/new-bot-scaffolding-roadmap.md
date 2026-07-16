# New Discord Bot from Scratch — Scaffolding Roadmap

> **Goal:** Create a new Rust Discord bot using the exact same architecture, patterns, dependencies, and scaffolding as DEATHxRUST — but with a clean slate (no boosterrole, settings, or other bot-specific commands/handlers).

---

## Phase 0: Project Initialization

### 0.1 — Create the Project Directory & Git Repo
- `cargo init <bot-name>` (or `cargo new <bot-name>`)
- `git init` if not already initialized
- Create `.gitignore` (copy from DEATHxRUST — covers Rust artifacts, `.env`, IDE files, OS files, logs, `data/` directory)

### 0.2 — Set Up `Cargo.toml`
Copy the structure from DEATHxRUST, keeping:

```toml
[package]
name = "<bot-name>"
version = "0.1.0"
edition = "2021"

[lib]
name = "<bot_name_snake>"
path = "src/lib.rs"

[[bin]]
name = "<bot-name>"
path = "src/main.rs"

[[bin]]
name = "deploy_commands"
path = "src/bin/deploy_commands.rs"

[dependencies]
# Core async runtime
tokio = { version = "1", features = ["macros", "rt-multi-thread", "signal"] }

# Discord
serenity = { version = "0.12", features = ["cache"] }
poise = "0.6"

# Database (include if persistence needed)
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "sqlite"] }

# Config & Environment
dotenv = "0.15"
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Error handling
thiserror = "2"

# Time
chrono = { version = "0.4", features = ["serde"] }

# HTTP (include if needed for external API calls)
# reqwest = { version = "0.12", features = ["json"] }

# Random (include if needed)
# rand = "0.8"
```

**Notes:**
- Remove `image`, `palette`, `reqwest`, `rand` unless your bot needs them
- The dual-binary setup (`main.rs` + `deploy_commands.rs`) and library crate (`lib.rs`) are essential to the pattern

### 0.3 — Create `.env.example` and `.env`
```env
DISCORD_TOKEN=your_token_here
DEBUG=false
COMMAND_PREFIX=!
DEVELOPMENT_GUILD_ID=
AUTO_SYNC_COMMANDS=false
SLASH_COMMANDS_GLOBAL=false
ALWAYS_USE_EMBEDS=true
```

### 0.4 — Create `Makefile`
Copy directly from DEATHxRUST — all targets are generic:
- `build`, `run`, `dev`, `watch`, `watch-clear`
- `deploy`, `deploy-global`, `deploy-guild`, `deploy-dry`
- `clean`, `test`, `check`, `fmt`, `lint`
- `install`, `env`

Update the binary name references from `death-bot` to your bot name.

---

## Phase 1: Core Architecture (The Skeleton)

### 1.1 — `src/main.rs` (Entry Point)
```
- Load dotenv
- Initialize tracing subscriber
- Load Settings::from_env()
- Create poise client
- Start client with ctrl+c handler
```
Exact pattern from DEATHxRUST — this file is ~30 lines and fully generic.

### 1.2 — `src/lib.rs` (Library Crate)
```rust
pub mod bot;
pub mod commands;
pub mod config;
pub mod data;
pub mod handlers;
pub mod utils;
```
All modules publicly exported so the deploy binary and tests can access them.

### 1.3 — `src/config/mod.rs` + `src/config/settings.rs`
Copy the `Settings` struct and `from_env()` method. This is fully generic scaffolding:
- `discord_token: String`
- `debug_mode: bool`
- `command_prefix: String`
- `development_guild_id: Option<u64>`
- `auto_sync_commands: bool`
- `slash_commands_global: bool`
- `always_use_embeds: bool`

Add new settings fields as your bot's features require.

### 1.4 — `src/bot/mod.rs` (Bot Module Root)
Export:
- `Data` struct (shared application state)
- `Error` enum (framework error type)
- `Context<'a>` type alias
- `Framework` type alias
- `create_framework()` function
- `create_poise_client()` function

### 1.5 — `src/bot/data.rs` (Shared State & Error Types)
```rust
pub struct Data {
    pub settings: Settings,
    pub db_pool: SqlitePool,  // Remove if no database needed
}

pub enum Error {
    Serenity(serenity::Error),
    Config(String),
    Command(String),
    Database(sqlx::Error),  // Remove if no database needed
}

pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Framework = poise::Framework<Data, Error>;
```
Include `From` implementations for: `serenity::Error`, `sqlx::Error`, `BotError`, `&str`, `String`.

### 1.6 — `src/bot/intents.rs`
```rust
pub fn get_bot_intents() -> GatewayIntents {
    GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
}
```
Adjust intents based on what your bot actually needs. Only request what you use.

### 1.7 — `src/bot/poise_client.rs`
```rust
pub async fn create_poise_client(settings: &Settings) -> Result<Client, ...> {
    let framework = create_framework(settings.clone()).await;
    let intents = get_bot_intents();
    let mut cache_settings = CacheSettings::default();
    cache_settings.max_messages = 100;
    Client::builder(&settings.discord_token, intents)
        .cache_settings(cache_settings)
        .framework(framework)
        .await
}
```

### 1.8 — `src/bot/framework.rs` (Framework Setup)
This is the central wiring file. Copy the structure and strip bot-specific commands:

```
create_framework() -> Framework:
  1. FrameworkOptions:
     - commands: vec![ping::ping(), help::help()]  // Start with just these
     - pre_command: log command start (tracing::info!)
     - post_command: log command completion
     - prefix_options:
         - dynamic_prefix: from database/cache (or just use static default)
         - edit_tracking: 3600 seconds
         - mention_as_prefix: true
     - on_error: embed-wrapped error handler (copy from DEATHxRUST)
     - event_handler: skeleton that matches on FullEvent variants

  2. Setup closure:
     - Log bot startup info
     - Register commands (guild or global based on settings)
     - Initialize database
     - Return Data { settings, db_pool }
```

**Key convention:** The error handler must ALWAYS respond with embeds, never plain text. Copy the full `on_error` implementation.

---

## Phase 2: Utility Layer

### 2.1 — `src/utils/mod.rs`
Export all utility modules.

### 2.2 — `src/utils/error.rs` (Generic Error Types)
```rust
#[derive(Error, Debug)]
pub enum BotError {
    Config(String),
    Discord(serenity::Error),
    Io(std::io::Error),
    Command(String),
    Other(String),
}
pub type BotResult<T> = Result<T, BotError>;
```
Add domain-specific variants as needed (e.g., `InvalidColor(String)` was added for DEATHxRUST).

### 2.3 — `src/utils/embed_builder.rs` (Standardized Embeds)
Copy the `EmbedColor` enum and `EmbedBuilder` struct:
```rust
pub enum EmbedColor {
    Success,    // 0x62CB77 (green)
    Error,      // 0x853535 (dark red)
    Warning,    // 0xFFE209 (yellow)
    Info,       // 0xFFE209 (yellow)
    Primary,    // 0xC6AC80 (beige/tan) — customize to your bot's brand
    Secondary,  // 0x95A5A6 (gray)
    Custom(u32),
}
```
Methods: `new()`, `title()`, `description()`, `field()`, `color()`, `footer()`, `timestamp()`, `thumbnail()`, `build()`.

**Customize:** Change the `Primary` color to match your bot's brand.

### 2.4 — `src/utils/response.rs` (Response Helper)
Copy `ResponseHelper` with methods:
- `send_success(ctx, title, description)`
- `send_error(ctx, title, description)`
- `send_warning(ctx, title, description)`
- `send_info(ctx, title, description)`
- `send_primary(ctx, title, description)`
- `send_embed(ctx, embed)`

This enforces the embed-only response pattern. Every command response goes through this.

---

## Phase 3: Database Layer (Optional — Include if Bot Needs Persistence)

### 3.1 — `src/data/mod.rs`
Export `init_database` function and models submodule.

### 3.2 — `src/data/database.rs`
```rust
pub async fn init_database() -> Result<SqlitePool, sqlx::Error> {
    std::fs::create_dir_all("data")?;
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            SqliteConnectOptions::new()
                .filename("data/bot.db")
                .create_if_missing(true)
        )
        .await?;

    // Run migrations / CREATE TABLE IF NOT EXISTS statements
    sqlx::query("CREATE TABLE IF NOT EXISTS guild_prefixes (
        guild_id INTEGER PRIMARY KEY,
        prefix TEXT NOT NULL,
        set_at TEXT NOT NULL DEFAULT (datetime('now'))
    )").execute(&pool).await?;

    // Add your tables here...

    Ok(pool)
}
```

### 3.3 — `src/data/models/mod.rs`
Start with just `GuildPrefix` model if using dynamic prefixes. Add models as you add features.

**Model convention:**
```rust
#[derive(Debug, Clone, FromRow)]
pub struct ModelName { /* fields */ }

impl ModelName {
    pub async fn get(pool: &SqlitePool, ...) -> Result<Option<Self>, sqlx::Error> { ... }
    pub async fn set(pool: &SqlitePool, ...) -> Result<(), sqlx::Error> { ... }
    pub async fn delete(pool: &SqlitePool, ...) -> Result<bool, sqlx::Error> { ... }
}
```
- Use `i64` for Discord IDs (SQLite constraint)
- Use `UPSERT` patterns (`INSERT ... ON CONFLICT DO UPDATE`)
- Use `tracing::info!` / `tracing::debug!` for logging

---

## Phase 4: Starter Commands

### 4.1 — `src/commands/mod.rs`
```rust
pub mod ping;
pub mod help;
```

### 4.2 — `src/commands/ping.rs`
```rust
use crate::bot::{Context, Error};
use crate::utils::{EmbedBuilder, EmbedColor, ResponseHelper};

/// Check bot latency
#[poise::command(slash_command, prefix_command, category = "General")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = std::time::Instant::now();
    let msg = ResponseHelper::send_primary(ctx, "Pong!", "Measuring latency...").await;
    let elapsed = start.elapsed().as_millis();
    // Optionally edit with actual latency
    Ok(())
}
```

### 4.3 — `src/commands/help.rs`
Copy the help command pattern — it dynamically lists all registered commands with descriptions and categories.

### 4.4 — Register in `framework.rs`
```rust
let commands = vec![
    ping::ping(),
    help::help(),
];
```

---

## Phase 5: Event Handler Skeleton

### 5.1 — `src/handlers/mod.rs`
Start empty — add handler modules as your bot grows.

### 5.2 — Event Handler in `framework.rs`
```rust
async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            tracing::info!("{} is connected!", data_about_bot.user.name);
        }
        _ => {}
    }
    Ok(())
}
```

Add event arms as you build features that need them.

---

## Phase 6: Deploy System

### 6.1 — `src/bin/deploy_commands.rs`
Copy from DEATHxRUST. Update the command list to match your bot's commands:
```rust
let commands = vec![
    ping::ping(),
    help::help(),
    // Add your commands here as you build them
];
```

### 6.2 — `scripts/deploy.sh`
Copy directly — fully generic. Just update the binary name reference.

---

## Phase 7: Development Tooling

### 7.1 — `CLAUDE.md`
Create a new CLAUDE.md documenting:
- Build/run/test commands
- Architecture overview (same pattern)
- How to add new commands
- Configuration system
- Environment setup

### 7.2 — Agent surface (prefer `.grok/`)
Copy from DEATHxRUST (adjust as needed):
- `.grok/skills/` — commit, commit-and-merge, implement-roadmap, new-feature-report, new-feature-roadmap, analyze-roadmap, troubleshoot-issue
- `.grok/rules/` — project-overview, rust-discord-bot, command-development, data-management
- `.grok/guidelines/` — full Rust/Poise, command, and data guidelines
- Optional Claude Code only: `.claude/guidelines/` if you still use Claude (no slash commands; Grok skills cover those workflows)

### 7.3 — `README.md`
Create a new README with:
- Bot description and features
- Setup instructions (clone, `.env`, `cargo run`)
- Available commands
- Project structure
- Contributing guide

### 7.4 — `DEPLOYMENT.md`
Copy from DEATHxRUST — the deployment patterns are fully generic.

---

## Phase 8: Testing Infrastructure

### 8.1 — `tests/config_tests.rs`
Copy the `Settings::from_env()` tests — they're generic scaffolding.

### 8.2 — Unit Test Convention
Add `#[cfg(test)] mod tests` blocks in each module. Follow the existing patterns:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // ...
    }

    #[tokio::test]
    async fn test_async_feature() {
        // ...
    }
}
```

---

## Final Directory Structure (Clean Slate)

```
<bot-name>/
  Cargo.toml
  Makefile
  .env.example
  .env                          # (gitignored)
  .gitignore
  CLAUDE.md
  README.md
  DEPLOYMENT.md
  scripts/
    deploy.sh
  src/
    main.rs                     # Entry point
    lib.rs                      # pub mod declarations
    bin/
      deploy_commands.rs        # Standalone deployment CLI
    bot/
      mod.rs                    # Exports Data, Context, Error, Framework
      data.rs                   # Shared state, error enum, type aliases
      framework.rs              # Poise framework config, command registration, event routing
      intents.rs                # Gateway intents
      poise_client.rs           # Client builder
    commands/
      mod.rs                    # Command module exports
      ping.rs                   # /ping
      help.rs                   # /help
    config/
      mod.rs                    # Exports Settings
      settings.rs               # Environment config
    data/                       # (include if using database)
      mod.rs                    # Exports init_database
      database.rs               # SQLite pool + schema init
      models/
        mod.rs                  # Model exports
    handlers/
      mod.rs                    # Handler exports (initially empty)
    utils/
      mod.rs                    # Utility exports
      error.rs                  # BotError enum, BotResult
      embed_builder.rs          # EmbedBuilder, EmbedColor
      response.rs               # ResponseHelper (embed-only responses)
  tests/
    config_tests.rs             # Settings tests
  .grok/
    rules/
      project-overview.md
      rust-discord-bot.md
      command-development.md
      data-management.md
    skills/
      commit/
      commit-and-merge/
      implement-roadmap/
      new-feature-report/
      new-feature-roadmap/
      analyze-roadmap/
      troubleshoot-issue/
    guidelines/
      rust-discord-bot.md
      command-development.md
      data-management.md
```

---

## Conventions Carried Forward

| Convention | Description |
|---|---|
| **Embed-only responses** | Never send plain text. Every response is an embed via `ResponseHelper`. |
| **Two-layer errors** | `BotError` (utils) converts into `Error` (framework). Add domain errors as needed. |
| **Dynamic prefix** | Per-guild prefix with in-memory cache + database persistence. |
| **Structured logging** | `tracing::info!` with structured fields (`guild_id`, `user_id`, `command`). |
| **Model pattern** | `FromRow` structs with `get/set/delete` async associated functions. |
| **Deferred responses** | Call `ctx.defer().await?` before any long-running operation. |
| **Cache safety** | Extract guild data from cache into locals BEFORE any `.await` point. |
| **Conditional compilation** | `#[cfg(debug_assertions)]` for dev-only commands/features. |
| **Command attributes** | Always declare: `slash_command`, `prefix_command`, `category`, permissions as needed. |
| **UPSERT database writes** | `INSERT ... ON CONFLICT DO UPDATE` for idempotent operations. |

---

## Implementation Order (Recommended)

1. **Phase 0** — Project init, Cargo.toml, .env, Makefile, .gitignore
2. **Phase 1** — Core architecture (main.rs through framework.rs) — get the bot online with zero commands
3. **Phase 2** — Utility layer (errors, embeds, response helpers)
4. **Phase 4** — Starter commands (ping, help) — verify the bot responds
5. **Phase 3** — Database layer (add when first feature needs persistence)
6. **Phase 5** — Event handler skeleton
7. **Phase 6** — Deploy system
8. **Phase 7** — Dev tooling (CLAUDE.md, .claude/ directory, README)
9. **Phase 8** — Testing infrastructure

**Milestone checkpoints:**
- After Phase 1+2+4: Bot starts, connects to Discord, responds to `/ping` and `/help`
- After Phase 3: Bot has persistent storage ready for feature data
- After Phase 6: Commands can be deployed to guilds or globally via CLI
- After Phase 7+8: Full development workflow with Claude Code integration and tests
