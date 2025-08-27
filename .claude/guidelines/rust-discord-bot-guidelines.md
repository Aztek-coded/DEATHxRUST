# Rust Discord Bot Development Guidelines

## Table of Contents
1. [Framework Architecture](#framework-architecture)
2. [Command Development](#command-development)
3. [Error Handling](#error-handling)
4. [Async/Await Patterns](#asyncawait-patterns)
5. [Data Management](#data-management)
6. [Event Handling](#event-handling)
7. [Testing Strategy](#testing-strategy)
8. [Performance Optimization](#performance-optimization)
9. [Security Best Practices](#security-best-practices)
10. [Deployment Guidelines](#deployment-guidelines)

## Framework Architecture

### Core Dependencies
```toml
# Essential dependencies for Discord bot
poise = "0.6"          # Command framework (built on Serenity)
serenity = "0.12"      # Discord API wrapper
tokio = { version = "1", features = ["full"] }  # Async runtime
```

### Module Organization
```
src/
├── bot/           # Bot initialization and framework setup
│   ├── mod.rs     # Module exports and types
│   ├── framework.rs  # Poise framework configuration
│   ├── data.rs    # Shared application state
│   └── poise_client.rs  # Client builder
├── commands/      # Command implementations
│   ├── mod.rs     # Command exports
│   └── *.rs       # Individual command files
├── handlers/      # Event and interaction handlers
├── config/        # Configuration management
└── utils/         # Utilities and helpers
```

### Type Aliases
Always define type aliases in `bot/mod.rs` for consistency:
```rust
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Framework = poise::Framework<Data, Error>;
```

## Command Development

### Basic Command Structure
```rust
use crate::bot::{Context, Error};

/// Command description (shown in Discord)
#[poise::command(
    slash_command,           // Enable as slash command
    prefix_command,          // Enable as prefix command
    category = "Category",   // Command category
    aliases("alias1"),       // Alternative names
    required_permissions = "SEND_MESSAGES",  // Required permissions
)]
pub async fn command_name(
    ctx: Context<'_>,
    #[description = "Parameter description"] param: String,
) -> Result<(), Error> {
    // Implementation
    ctx.say("Response").await?;
    Ok(())
}
```

### Command Parameters
```rust
// Required parameter
#[description = "User to greet"] user: serenity::model::user::User,

// Optional parameter
#[description = "Custom message"] message: Option<String>,

// Default values
#[description = "Page number"] 
#[min = 1] 
#[max = 100] 
page: Option<u32>,

// Autocomplete
#[autocomplete = "autocomplete_function"] item: String,
```

### Response Types
```rust
// Simple text response
ctx.say("Hello!").await?;

// Ephemeral response (only visible to user)
ctx.send(poise::CreateReply::default()
    .content("Private message")
    .ephemeral(true)
).await?;

// Embed response
ctx.send(poise::CreateReply::default()
    .embed(serenity::all::CreateEmbed::default()
        .title("Title")
        .description("Description")
        .color(0x00ff00)
    )
).await?;

// Deferred response for long operations
ctx.defer().await?;
// ... long operation ...
ctx.say("Done!").await?;
```

### Subcommands
```rust
#[poise::command(slash_command, subcommands("add", "remove", "list"))]
pub async fn config(_: Context<'_>) -> Result<(), Error> {
    Ok(())  // Parent command doesn't execute
}

#[poise::command(slash_command)]
pub async fn add(ctx: Context<'_>, key: String, value: String) -> Result<(), Error> {
    // Implementation
    Ok(())
}
```

## Error Handling

### Custom Error Types
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Command error: {0}")]
    Command(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Discord API error: {0}")]
    Serenity(#[from] serenity::Error),
}
```

### Error Handler
```rust
on_error: |error| {
    Box::pin(async move {
        match error {
            poise::FrameworkError::Command { error, ctx, .. } => {
                // Log error
                tracing::error!("Command failed: {:?}", error);
                
                // User-friendly message
                let _ = ctx.say("❌ An error occurred").await;
            }
            poise::FrameworkError::ArgumentParse { error, ctx, .. } => {
                let _ = ctx.say(format!("❌ Invalid arguments: {}", error)).await;
            }
            _ => {
                poise::builtins::on_error(error).await.ok();
            }
        }
    })
}
```

### Result Pattern
```rust
// Early return on error
let data = fetch_data().await?;

// Handle specific errors
match operation().await {
    Ok(result) => process(result),
    Err(e) if e.is_timeout() => handle_timeout(),
    Err(e) => return Err(e.into()),
}

// Provide context
let user = fetch_user(id)
    .await
    .map_err(|e| BotError::Command(format!("Failed to fetch user {}: {}", id, e)))?;
```

## Async/Await Patterns

### Concurrent Operations
```rust
use tokio::join;

// Run multiple operations concurrently
let (result1, result2) = join!(
    fetch_data1(),
    fetch_data2()
);

// With error handling
let results = tokio::try_join!(
    operation1(),
    operation2()
)?;
```

### Timeouts
```rust
use tokio::time::{timeout, Duration};

match timeout(Duration::from_secs(30), long_operation()).await {
    Ok(result) => result?,
    Err(_) => return Err("Operation timed out".into()),
}
```

### Background Tasks
```rust
// Spawn background task
tokio::spawn(async move {
    // Long-running operation
    process_in_background().await;
});

// With error handling
tokio::spawn(async move {
    if let Err(e) = background_task().await {
        tracing::error!("Background task failed: {}", e);
    }
});
```

## Data Management

### Shared State
```rust
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Data {
    settings: Settings,
    cache: Arc<RwLock<HashMap<String, String>>>,
    db_pool: sqlx::PgPool,
}

impl Data {
    pub async fn get_cached(&self, key: &str) -> Option<String> {
        self.cache.read().await.get(key).cloned()
    }
    
    pub async fn set_cached(&self, key: String, value: String) {
        self.cache.write().await.insert(key, value);
    }
}
```

### Database Integration
```rust
// Using SQLx
pub async fn get_user_data(
    pool: &sqlx::PgPool,
    user_id: i64,
) -> Result<UserData, Error> {
    sqlx::query_as!(
        UserData,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}
```

### Caching Strategy
```rust
use moka::future::Cache;

pub struct Data {
    user_cache: Cache<u64, User>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            user_cache: Cache::builder()
                .max_capacity(1000)
                .time_to_live(Duration::from_secs(300))
                .build(),
        }
    }
}
```

## Event Handling

### Event Handler Setup
```rust
async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            tracing::info!("Bot ready: {}", data_about_bot.user.name);
        }
        FullEvent::Message { new_message } => {
            handle_message(ctx, new_message, data).await?;
        }
        FullEvent::GuildMemberAddition { new_member } => {
            handle_new_member(ctx, new_member, data).await?;
        }
        _ => {}
    }
    Ok(())
}
```

### Rate Limiting
```rust
use std::collections::HashMap;
use tokio::time::Instant;

struct RateLimiter {
    limits: Arc<RwLock<HashMap<u64, Instant>>>,
    duration: Duration,
}

impl RateLimiter {
    async fn check_limit(&self, user_id: u64) -> bool {
        let mut limits = self.limits.write().await;
        
        match limits.get(&user_id) {
            Some(last_use) if last_use.elapsed() < self.duration => false,
            _ => {
                limits.insert(user_id, Instant::now());
                true
            }
        }
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_command_logic() {
        let result = process_data("input").await;
        assert_eq!(result, expected_output);
    }
    
    #[test]
    fn test_parsing() {
        let parsed = parse_input("test");
        assert!(parsed.is_ok());
    }
}
```

### Integration Tests
```rust
// tests/integration_test.rs
use your_bot::{create_framework, Settings};

#[tokio::test]
async fn test_bot_startup() {
    let settings = Settings::from_env().unwrap();
    let framework = create_framework(settings).await;
    assert!(framework.is_ok());
}
```

### Mock Testing
```rust
use mockall::automock;

#[automock]
trait DataStore {
    async fn get(&self, key: &str) -> Option<String>;
    async fn set(&self, key: &str, value: String);
}

#[tokio::test]
async fn test_with_mock() {
    let mut mock = MockDataStore::new();
    mock.expect_get()
        .with(eq("key"))
        .return_const(Some("value".to_string()));
    
    let result = function_using_store(&mock, "key").await;
    assert_eq!(result, "value");
}
```

## Performance Optimization

### Memory Management
```rust
// Use Arc for shared immutable data
let shared_data = Arc::new(large_data);

// Use Cow for potentially borrowed data
use std::borrow::Cow;
fn process(data: Cow<'_, str>) -> String {
    // Avoids allocation if not modified
    data.into_owned()
}

// Pre-allocate collections
let mut vec = Vec::with_capacity(expected_size);
```

### Async Optimization
```rust
// Buffer operations
use futures::stream::{self, StreamExt};

let results: Vec<_> = stream::iter(items)
    .map(|item| async move { process(item).await })
    .buffer_unordered(10)  // Process 10 concurrently
    .collect()
    .await;

// Use select! for multiple futures
tokio::select! {
    result = operation1() => handle_result1(result),
    result = operation2() => handle_result2(result),
    _ = tokio::time::sleep(Duration::from_secs(30)) => handle_timeout(),
}
```

### Database Optimization
```rust
// Use prepared statements
let stmt = client.prepare("SELECT * FROM users WHERE id = $1").await?;
let row = client.query_one(&stmt, &[&user_id]).await?;

// Batch operations
let values: Vec<_> = users.iter()
    .map(|u| format!("({}, '{}')", u.id, u.name))
    .collect();
let query = format!("INSERT INTO users VALUES {}", values.join(","));

// Connection pooling
let pool = sqlx::postgres::PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?;
```

## Security Best Practices

### Input Validation
```rust
// Validate user input
fn validate_input(input: &str) -> Result<String, Error> {
    if input.len() > 100 {
        return Err("Input too long".into());
    }
    
    if input.contains(['<', '>', '@']) {
        return Err("Invalid characters".into());
    }
    
    Ok(input.to_string())
}

// Sanitize for Discord
fn sanitize_for_discord(text: &str) -> String {
    text.replace("@everyone", "@\u{200B}everyone")
        .replace("@here", "@\u{200B}here")
}
```

### Permission Checks
```rust
#[poise::command(
    slash_command,
    required_permissions = "MANAGE_GUILD",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    guild_only
)]
pub async fn admin_command(ctx: Context<'_>) -> Result<(), Error> {
    // Additional runtime checks
    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let member = ctx.author_member().await.ok_or("Not a member")?;
    
    if !member.permissions()?.manage_guild() {
        return Err("Insufficient permissions".into());
    }
    
    // Command logic
    Ok(())
}
```

### Secret Management
```rust
// Never hardcode secrets
use std::env;

pub struct Secrets {
    discord_token: String,
    database_url: String,
    api_key: Option<String>,
}

impl Secrets {
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            discord_token: env::var("DISCORD_TOKEN")
                .map_err(|_| "DISCORD_TOKEN not set")?,
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite::memory:".to_string()),
            api_key: env::var("API_KEY").ok(),
        })
    }
}

// Use secrets manager in production
#[cfg(feature = "production")]
async fn get_secrets() -> Result<Secrets, Error> {
    // AWS Secrets Manager, Vault, etc.
    aws_secrets_manager::get_secrets("bot-secrets").await
}
```

### Rate Limiting
```rust
use governor::{Quota, RateLimiter};

pub struct CommandRateLimiter {
    limiter: RateLimiter<u64, DefaultKeyedStateStore<u64>, DefaultClock>,
}

impl CommandRateLimiter {
    pub fn new(per_minute: u32) -> Self {
        let quota = Quota::per_minute(NonZeroU32::new(per_minute).unwrap());
        Self {
            limiter: RateLimiter::keyed(quota),
        }
    }
    
    pub async fn check_rate_limit(&self, user_id: u64) -> Result<(), Error> {
        self.limiter
            .check_key(&user_id)
            .map_err(|_| "Rate limit exceeded".into())
    }
}
```

## Deployment Guidelines

### Environment Configuration
```bash
# .env.example
DISCORD_TOKEN=your_token_here
COMMAND_PREFIX=!
DEVELOPMENT_GUILD_ID=123456789
AUTO_SYNC_COMMANDS=false
SLASH_COMMANDS_GLOBAL=false
RUST_LOG=info
DATABASE_URL=postgres://user:pass@localhost/dbname
```

### Docker Deployment
```dockerfile
# Dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/bot /usr/local/bin/bot
CMD ["bot"]
```

### Health Checks
```rust
// Implement health endpoint
pub async fn health_check(data: &Data) -> Result<HealthStatus, Error> {
    // Check Discord connection
    let discord_ok = data.is_connected().await;
    
    // Check database
    let db_ok = sqlx::query("SELECT 1")
        .fetch_one(&data.db_pool)
        .await
        .is_ok();
    
    Ok(HealthStatus {
        discord: discord_ok,
        database: db_ok,
        uptime: data.start_time.elapsed(),
    })
}
```

### Logging Configuration
```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bot=info,poise=info,serenity=warn".into())
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

// Structured logging
tracing::info!(
    user_id = %ctx.author().id,
    command = %ctx.command().qualified_name,
    "Command executed"
);
```

### Graceful Shutdown
```rust
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let bot = create_bot().await?;
    
    tokio::select! {
        result = bot.start() => {
            if let Err(e) = result {
                tracing::error!("Bot error: {}", e);
            }
        }
        _ = signal::ctrl_c() => {
            tracing::info!("Shutting down gracefully...");
            bot.shutdown().await;
        }
    }
    
    Ok(())
}
```

### Monitoring
```rust
// Metrics collection
use prometheus::{Encoder, TextEncoder, Counter, register_counter};

lazy_static! {
    static ref COMMAND_COUNTER: Counter = register_counter!(
        "bot_commands_total",
        "Total number of commands executed"
    ).unwrap();
}

// In command handler
COMMAND_COUNTER.inc();

// Expose metrics endpoint
async fn metrics_handler() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
```

### Command Registration Strategy
```rust
// Development: Register to specific guild (instant)
if cfg!(debug_assertions) {
    poise::builtins::register_in_guild(
        &ctx.http,
        &framework.options().commands,
        guild_id,
    ).await?;
}

// Production: Register globally (takes up to 1 hour)
else {
    poise::builtins::register_globally(
        &ctx.http,
        &framework.options().commands,
    ).await?;
}

// Hybrid approach
match settings.command_registration {
    Registration::Guild(id) => register_in_guild(ctx, commands, id).await?,
    Registration::Global => register_globally(ctx, commands).await?,
    Registration::Both(id) => {
        register_in_guild(ctx, commands, id).await?;
        register_globally(ctx, commands).await?;
    }
}
```

## Best Practices Summary

### DO's
- ✅ Use type aliases for consistency
- ✅ Handle all errors explicitly
- ✅ Use async/await effectively
- ✅ Implement proper logging
- ✅ Write tests for critical functionality
- ✅ Use environment variables for configuration
- ✅ Implement rate limiting for commands
- ✅ Validate and sanitize all user input
- ✅ Use connection pooling for databases
- ✅ Implement graceful shutdown

### DON'Ts
- ❌ Don't hardcode secrets or tokens
- ❌ Don't block the async runtime
- ❌ Don't ignore error handling
- ❌ Don't use `.unwrap()` in production code
- ❌ Don't store sensitive data in plain text
- ❌ Don't trust user input without validation
- ❌ Don't use global mutable state without synchronization
- ❌ Don't ignore Discord API rate limits
- ❌ Don't deploy without proper logging
- ❌ Don't skip testing critical paths

## Resources

### Documentation
- [Poise Documentation](https://docs.rs/poise)
- [Serenity Documentation](https://docs.rs/serenity)
- [Discord API Documentation](https://discord.com/developers/docs)
- [Rust Async Book](https://rust-lang.github.io/async-book/)

### Examples
- [Poise Examples](https://github.com/serenity-rs/poise/tree/main/examples)
- [Serenity Examples](https://github.com/serenity-rs/serenity/tree/current/examples)

### Community
- [Serenity Discord Server](https://discord.gg/serenity-rs)
- [Rust Discord Server](https://discord.gg/rust-lang)