# Rust Discord Bot Patterns

Apply when editing Rust sources (`**/*.rs`), especially bot/framework code.

## Type aliases (`src/bot/mod.rs`)

```rust
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
```

## Errors

- Prefer typed errors (`thiserror`) and `?` with context via `map_err`
- Avoid `.unwrap()` in production paths
- Surface user-friendly messages; log details with `tracing`

## Async

- Use `tokio::join!` / `try_join!` for concurrent work
- Wrap long ops with `timeout`
- Spawn background tasks with explicit error logging
- Never block the async runtime

## Security

- Validate/sanitize all user input (length, chars, `@everyone`/`@here`)
- Use `required_permissions` / `required_bot_permissions` / `guild_only` on commands
- Secrets only from env — never hardcode tokens

## Command registration

- Dev: guild registration (fast)
- Prod: global (up to ~1h) or hybrid per settings

## Logging

- Structured `tracing` with user/command fields on command paths
- Init via env filter; default toward `bot=info,poise=info,serenity=warn`

## Do / Don't

- DO: type aliases, explicit errors, env config, rate limits, tests for critical paths
- DON'T: hardcode secrets, ignore Discord rate limits, skip input validation, use unsync global mutability

Longer reference: `.grok/guidelines/rust-discord-bot.md`
