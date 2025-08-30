# Dynamic Prefix Configuration System - Implementation Roadmap

## Feature Summary

**Name:** Dynamic Prefix Configuration System  
**Type:** New Feature Implementation  
**Complexity:** Medium (Database integration + Dynamic prefix resolution)  
**Estimated Timeline:** 3-4 development sessions

### Core Functionality
Implementation of a comprehensive prefix configuration command group that allows server administrators to customize the bot's command prefix on a per-guild basis, with persistent storage and real-time prefix resolution.

**Commands:**
- `prefix` / `prefix view` - Display current guild prefix configuration
- `prefix set <new_prefix>` - Update guild's command prefix with validation
- `prefix remove` / `prefix reset` - Revert to default environment prefix

## Discord Interaction Flow Analysis

### 1. View Current Prefix Flow
```
User: !prefix
↓
Bot: Parse command → Check guild_id → Query database → Format response
↓
Discord: "📋 **Current Configuration**\n🔧 Guild Prefix: `!`\n🌐 Default Prefix: `!`"
```

### 2. Set Custom Prefix Flow
```
User: !prefix set >>
↓
Bot: Parse command → Validate prefix → Check permissions → Update database → Update runtime cache
↓
Discord: "✅ **Prefix Updated**\nNew prefix: `>>`\nYou can now use `>>help` to test!"
```

### 3. Remove Custom Prefix Flow
```
User: >>prefix remove
↓
Bot: Parse command → Check permissions → Remove from database → Update runtime cache
↓
Discord: "🔄 **Prefix Reset**\nReverted to default prefix: `!`"
```

### 4. Dynamic Prefix Resolution Flow
```
User: >>help (with custom prefix)
↓
Poise Framework: Check dynamic prefix resolver → Query database by guild_id → Match prefix
↓
Bot: Execute help command normally
```

## Required Reference Files Analysis

### Core Files to Modify/Create

#### 1. **New Command Module** - `src/commands/prefix.rs`
- Implementation pattern following `src/commands/ping.rs:4-26`
- Subcommand structure using Poise `#[poise::command(subcommands(...))]` pattern
- Permission validation using `required_permissions = "MANAGE_GUILD"`
- Error handling integration with `bot::Error` types

#### 2. **Database Layer** - `src/data/` (new module)
- `src/data/mod.rs` - Module exports and connection management
- `src/data/database.rs` - SQLite connection, migrations, connection pooling
- `src/data/models.rs` - `GuildPrefix` struct and database operations
- SQLite schema: `guild_prefixes(guild_id BIGINT PRIMARY KEY, prefix TEXT NOT NULL)`

#### 3. **Bot Data Extension** - `src/bot/data.rs:4-7`
- Add `db_pool: sqlx::SqlitePool` field to `Data` struct
- Update constructor to initialize database connection
- Add cache layer using `Arc<RwLock<HashMap<u64, String>>>` for prefix caching

#### 4. **Framework Configuration** - `src/bot/framework.rs:14-24`
- Replace static `prefix: Some(settings.command_prefix.clone())` 
- Implement dynamic prefix resolver using closure:
  ```rust
  prefix: Some(|ctx| Box::pin(resolve_dynamic_prefix(ctx))),
  ```
- Add prefix resolution function that queries database with guild_id fallback

#### 5. **Cargo Dependencies** - `Cargo.toml:23-30`
- Add `sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }`
- Add `tracing = "0.1"` for structured logging

#### 6. **Command Registration** - `src/bot/framework.rs:9-13`
- Add `prefix::prefix()` to commands vector
- Update `src/commands/mod.rs:1-3` to export prefix module

#### 7. **Error Handling Extension** - `src/bot/data.rs:17-21`
- Add `Database(sqlx::Error)` variant to `Error` enum
- Implement `From<sqlx::Error>` conversion

## Implementation Strategy with Poise Framework Patterns

### Phase 1: Database Infrastructure Setup
**Branch:** `git checkout -b feature/prefix-database-setup`

1. **Database Schema & Migrations**
   - Create SQLite database with `guild_prefixes` table
   - Implement migration system for schema versioning
   - Add connection pooling configuration

2. **Data Layer Architecture**
   - Follow async/await patterns from guidelines
   - Implement repository pattern for database operations
   - Add connection health checks and error handling

3. **Bot Integration**
   - Update `Data` struct to include database pool
   - Modify bot initialization to establish database connection
   - Add graceful shutdown for database connections

### Phase 2: Dynamic Prefix Resolution
**Branch:** `git checkout -b feature/dynamic-prefix-resolution`

1. **Prefix Resolver Implementation**
   - Create async function for guild-specific prefix lookup
   - Implement caching layer for performance optimization
   - Add fallback to default prefix when database unavailable

2. **Framework Integration**
   - Replace static prefix configuration with dynamic resolver
   - Update Poise framework options to use closure-based prefix
   - Ensure backward compatibility with existing commands

3. **Performance Optimization**
   - Implement in-memory cache with TTL for frequent lookups
   - Add database query optimization with prepared statements
   - Use connection pooling for concurrent guild operations

### Phase 3: Prefix Command Implementation
**Branch:** `git checkout -b feature/prefix-commands`

1. **Command Structure**
   - Implement parent `prefix` command with subcommands
   - Add parameter validation for prefix format restrictions
   - Implement permission checking (MANAGE_GUILD required)

2. **Business Logic**
   - Prefix validation: length (1-5 chars), allowed characters, no Discord mentions
   - Guild-specific operations with proper error handling
   - Real-time cache invalidation on prefix updates

3. **Response Formatting**
   - Create rich embed responses for all command variants
   - Add success/error feedback with clear messaging
   - Include usage examples in responses

### Phase 4: Integration & Testing
**Branch:** `git checkout -b feature/prefix-system-integration`

1. **System Integration**
   - Update command registration to include prefix commands
   - Test prefix resolution with all existing commands
   - Verify error handling across all failure scenarios

2. **Logging Implementation**
   - Add structured logging using `tracing` crate
   - Log prefix changes, database operations, and command executions
   - Include guild_id, user_id, and operation context in logs

3. **Documentation & Deployment**
   - Update environment configuration examples
   - Add database setup instructions to deployment docs
   - Create migration scripts for production deployment

## Step-by-Step Roadmap with Poise/Serenity Considerations

### Step 1: Project Setup & Dependencies
```bash
# Branch creation
git checkout -b feature/dynamic-prefix-system

# Update Cargo.toml dependencies
cargo add sqlx --features runtime-tokio-rustls,sqlite
cargo add tracing

# Create database directory structure
mkdir -p src/data
touch src/data/mod.rs src/data/database.rs src/data/models.rs
```

### Step 2: Database Layer Implementation
**Files to Create/Modify:**
- `src/data/mod.rs` - Module exports and public API
- `src/data/database.rs` - Connection management, migrations
- `src/data/models.rs` - Data structures and CRUD operations

**Key Poise/Serenity Considerations:**
- Use `sqlx::SqlitePool` for async connection pooling
- Implement proper error handling with custom error types
- Follow async/await patterns for all database operations

### Step 3: Bot Data Integration
**Files to Modify:**
- `src/bot/data.rs:4-13` - Add database pool to Data struct
- `src/bot/poise_client.rs` - Initialize database connection during bot setup

**Logging Specification:**
```rust
tracing::info!(
    database_url = %database_path,
    "Database connection established"
);
```

### Step 4: Dynamic Prefix Resolution
**Files to Modify:**
- `src/bot/framework.rs:14-24` - Replace static prefix with dynamic resolver

**Implementation Approach:**
```rust
// Dynamic prefix resolver function
async fn resolve_dynamic_prefix(
    ctx: poise::Context<'_, Data, Error>
) -> Option<String> {
    if let Some(guild_id) = ctx.guild_id() {
        // Query database for guild-specific prefix
        ctx.data().get_guild_prefix(guild_id.get()).await.ok()
            .flatten()
            .or_else(|| Some(ctx.data().settings.command_prefix.clone()))
    } else {
        Some(ctx.data().settings.command_prefix.clone())
    }
}
```

### Step 5: Prefix Commands Implementation
**Files to Create:**
- `src/commands/prefix.rs` - Complete command implementation

**Command Structure:**
```rust
#[poise::command(
    slash_command,
    prefix_command,
    subcommands("view", "set", "remove"),
    required_permissions = "MANAGE_GUILD",
    guild_only,
    category = "Configuration"
)]
```

**Logging Specifications:**
- **Command Invocation:** Log user_id, guild_id, command variant, parameters
- **Prefix Changes:** Log old_prefix, new_prefix, user_id, guild_id, timestamp
- **Database Operations:** Log query execution time, success/failure, affected rows
- **Validation Failures:** Log invalid prefix attempts with reason

### Step 6: Command Registration & Module Integration
**Files to Modify:**
- `src/commands/mod.rs:1-3` - Add prefix module export
- `src/bot/framework.rs:9-13` - Register prefix command in commands vector

### Step 7: Error Handling Enhancement
**Files to Modify:**
- `src/bot/data.rs:17-40` - Add Database error variant
- Implement proper error propagation for all database operations

### Step 8: Testing & Validation
**Testing Strategy:**
- Unit tests for prefix validation logic
- Integration tests for database operations
- Mock testing for Discord API interactions
- Load testing for concurrent prefix lookups

**Verification Commands:**
```bash
# Test prefix functionality
cargo test
cargo clippy
cargo fmt

# Run bot and test commands
cargo run
# In Discord: !prefix, !prefix set >>, >>prefix remove
```

## Key Changes Summary

### Main Updates Required:
1. **Database Integration:** SQLite with sqlx for persistent prefix storage
2. **Dynamic Prefix Resolution:** Replace static prefix with guild-aware resolver
3. **New Command Module:** Comprehensive prefix management with validation
4. **Caching Layer:** In-memory cache for performance optimization
5. **Enhanced Error Handling:** Database error variants and proper propagation
6. **Structured Logging:** Comprehensive logging for operations and changes

### Discord API Constraints & Considerations:
- **Rate Limiting:** Prefix resolution must be fast to avoid command delays
- **Guild Permissions:** MANAGE_GUILD permission required for prefix changes
- **Slash Command Sync:** Custom prefixes affect prefix commands only, not slash commands
- **Cache Management:** Implement TTL-based caching to handle large guild counts
- **Graceful Degradation:** Default prefix fallback when database unavailable

### Branch Strategy:
```bash
git checkout main
git checkout -b feature/dynamic-prefix-system
# Implement all changes
git add .
git commit -m "feat: implement dynamic prefix configuration system

- Add SQLite database integration for guild prefix persistence
- Implement dynamic prefix resolution with caching
- Create comprehensive prefix command group (view/set/remove)
- Add structured logging for prefix operations
- Include validation and permission checking
- Ensure backward compatibility with existing commands"
```

### Database Schema:
```sql
CREATE TABLE guild_prefixes (
    guild_id BIGINT PRIMARY KEY,
    prefix TEXT NOT NULL CHECK(length(prefix) <= 5),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_guild_prefixes_guild_id ON guild_prefixes(guild_id);
```

This roadmap provides a comprehensive implementation plan that follows Discord bot best practices, integrates seamlessly with the existing Poise framework architecture, and ensures proper error handling, logging, and performance optimization throughout the development process.