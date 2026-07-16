# DEATHxRUST — Project Overview

Rust Discord bot using **Poise** (on Serenity) with Tokio async runtime.

## Dev commands

- Run: `cargo run` / `make dev` / `make watch`
- Release: `cargo build --release` / `make run`
- Quality: `cargo test` / `cargo fmt` / `cargo clippy` (or `make test|fmt|lint`)
- Deploy slash cmds: `make deploy` / `make deploy-global` / `make deploy-guild GUILD_ID=...` / `make deploy-dry`

## Module layout

- `src/bot/` — client, Poise framework, shared `Data`
- `src/commands/` — Poise command implementations
- `src/config/` — env-based settings
- `src/handlers/` — Discord event handlers
- `src/utils/` — errors, ResponseHelper, EmbedColor
- `src/data/` — persistence / data access

## Config (env via `Settings::from_env()`)

- `DISCORD_TOKEN` (required)
- `COMMAND_PREFIX` (default `!`)
- `DEVELOPMENT_GUILD_ID`
- `AUTO_SYNC_COMMANDS` (default false)
- `SLASH_COMMANDS_GLOBAL` (default false)

## Adding a command

1. New file under `src/commands/`
2. `#[poise::command(...)]` async fn returning `Result<(), Error>`
3. Register in `src/bot/framework.rs` `commands![]`
4. Export from `src/commands/mod.rs`

## Agent guidance

- Grok primary surface: `.grok/rules/`, `.grok/skills/`, root `AGENTS.md`
- Cursor: `.cursor/rules/`, `.cursor/skills/`
- Claude Code only: `.claude/` (do not treat as Grok source of truth)
- Prefer repo-relative paths in docs/skills
- Do not commit, push, or merge unless the user explicitly asks
