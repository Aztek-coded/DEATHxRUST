# Settings Command Suite Implementation Roadmap (Enhanced)

## Feature Summary
A comprehensive guild configuration system providing server administrators with granular control over guild settings through organized command groups. The system includes configuration viewing, staff management, auto-nickname assignment, join/leave logging, and premium role designation. This roadmap follows the Rust Discord Bot Development Guidelines with proper type aliases, standardized color system, ResponseHelper usage, and comprehensive error handling.

## Discord Interaction Flow Analysis

### User Interaction Flow
1. **Administrator invokes `/settings` command**
   - Bot responds with embed showing all available subcommands and current configuration state
   - Permission check: Requires "Manage Guild" permission

2. **Configuration Subcommands**
   - `/settings config` → Query database for all guild settings → Return formatted embed
   - `/settings staff <role>` → Validate role → Store in `guild_staff_roles` → Confirm
   - `/settings autonick <nickname>` → Validate string → Store in `guild_auto_nicknames` → Confirm
   - `/settings joinlogs <channel>` → Validate channel → Store in `guild_join_log_channels` → Confirm
   - `/settings premiumrole <role>` → Validate role → Store in `guild_premium_roles` → Confirm

3. **Event-Driven Behaviors**
   - **Member Join Event** → Check `guild_auto_nicknames` → Apply nickname if configured
   - **Member Join/Leave Event** → Check `guild_join_log_channels` → Send log embed if configured
   - **Premium Role Assignment** → Check `guild_premium_roles` → Apply special behaviors

## Hypothesized Implementation Approaches

### Database-First Approach
Create database schema first to establish data structure, then build commands and handlers around it. This ensures data persistence layer is solid before implementing business logic.

### Modular Command Structure
Following the `boosterrole` pattern, create a parent `settings` module with individual subcommand files. Each subcommand handles its specific configuration domain.

### Event Handler Integration
Extend existing event handler pattern to include `GuildMemberAddition` and `GuildMemberRemoval` events, with database lookups for configured behaviors.

## Key Module and File Changes

### New Files to Create
```
src/commands/settings/
├── mod.rs                  # Parent command with type aliases and exports
├── config.rs               # View configuration subcommand with concurrent queries
├── staff.rs                # Staff role management with ResponseHelper
├── autonick.rs            # Auto-nickname configuration with placeholders
├── joinlogs.rs            # Join/leave logging setup with embeds
└── premiumrole.rs         # Premium role designation with hierarchy checks

src/handlers/
└── member_handler.rs       # Member join/leave event handling with error recovery

src/data/models/
├── guild_settings.rs       # New models with FromRow pattern for settings tables
└── (extend models.rs)     # Add new model implementations with CRUD methods

src/utils/
├── settings_error.rs       # Custom error types for settings operations
└── settings_rate_limiter.rs # Rate limiting for settings changes
```

### Files to Modify
```
src/bot/framework.rs        # Register settings command
src/commands/mod.rs         # Export settings module
src/data/database.rs        # Add new table schemas
src/handlers/mod.rs         # Export member_handler
src/bot/framework.rs        # Add member event handling in event_handler
```

## Git Branch Strategy
```bash
git checkout -b feature/settings-command-suite
```

## Logging Specifications (using `tracing`)

### Command Invocations
```rust
// In each settings subcommand
tracing::info!(
    user_id = %ctx.author().id,
    guild_id = %guild_id,
    subcommand = "settings.staff",
    role_id = %role_id,
    "Settings command executed: staff role added"
);
```

### Event Processing
```rust
// In member_handler.rs
tracing::debug!(
    guild_id = %new_member.guild_id,
    user_id = %new_member.user.id,
    "Processing member join event"
);

tracing::info!(
    guild_id = %guild_id,
    user_id = %user_id,
    nickname = %auto_nickname,
    "Applied auto-nickname to new member"
);
```

### Database Operations
```rust
// In model methods
tracing::debug!(
    "Database query: upsert_guild_staff_role for guild {} role {}",
    guild_id,
    role_id
);
```

### Error Conditions
```rust
tracing::error!(
    guild_id = %guild_id,
    channel_id = %channel_id,
    error = ?e,
    "Failed to send join log message"
);
```

## Code Implementation Examples

### Type Aliases and Imports
```rust
// src/commands/settings/mod.rs
use crate::bot::{Context, Error};
use crate::utils::{ResponseHelper, EmbedColor};
use serenity::all::{CreateEmbed, GuildId, RoleId, ChannelId};

// Type alias for consistency
pub type SettingsContext<'a> = Context<'a>;
```

### Parent Command Structure
```rust
// src/commands/settings/mod.rs
#[poise::command(
    slash_command,
    prefix_command,
    category = "Administration",
    aliases("config", "cfg", "set"),
    required_permissions = "MANAGE_GUILD",
    guild_only,
    subcommands("config", "staff", "autonick", "joinlogs", "premiumrole"),
    broadcast_typing
)]
pub async fn settings(ctx: Context<'_>) -> Result<(), Error> {
    // Show overview embed using ResponseHelper
    ResponseHelper::send_info(ctx, "⚙️ Guild Settings", 
        "Use subcommands to configure your server:\n\
        • `/settings config` - View all settings\n\
        • `/settings staff` - Manage staff roles\n\
        • `/settings autonick` - Auto-nickname setup\n\
        • `/settings joinlogs` - Join/leave logging\n\
        • `/settings premiumrole` - Premium role setup"
    ).await?;
    Ok(())
}
```

### Custom Error Types
```rust
// src/utils/settings_error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Invalid nickname format: {0}")]
    InvalidNickname(String),
    
    #[error("Channel not accessible: {0}")]
    ChannelPermissionDenied(String),
    
    #[error("Role hierarchy conflict: {0}")]
    RoleHierarchyError(String),
    
    #[error("Settings limit exceeded: {0}")]
    LimitExceeded(String),
    
    #[error("Insufficient permissions")]
    InsufficientPermissions,
}

impl From<SettingsError> for Error {
    fn from(err: SettingsError) -> Self {
        Error::Command(err.to_string())
    }
}
```

### Database Model Implementation
```rust
// src/data/models/guild_settings.rs
use serenity::all::{GuildId, RoleId, ChannelId, UserId};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, FromRow)]
pub struct GuildStaffRole {
    pub id: i64,
    pub guild_id: i64,
    pub role_id: i64,
    pub added_by: i64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl GuildStaffRole {
    pub async fn add(
        pool: &SqlitePool,
        guild_id: GuildId,
        role_id: RoleId,
        added_by: UserId,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!(
            "Database query: add_staff_role for guild {} role {}",
            guild_id, role_id
        );
        
        sqlx::query(
            r#"
            INSERT INTO guild_staff_roles (guild_id, role_id, added_by)
            VALUES (?, ?, ?)
            ON CONFLICT (guild_id, role_id)
            DO UPDATE SET updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(guild_id.get() as i64)
        .bind(role_id.get() as i64)
        .bind(added_by.get() as i64)
        .execute(pool)
        .await?;
        
        tracing::info!(
            guild_id = %guild_id,
            role_id = %role_id,
            added_by = %added_by,
            "Staff role added to database"
        );
        
        Ok(())
    }
    
    pub async fn remove(
        pool: &SqlitePool,
        guild_id: GuildId,
        role_id: RoleId,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM guild_staff_roles WHERE guild_id = ? AND role_id = ?"
        )
        .bind(guild_id.get() as i64)
        .bind(role_id.get() as i64)
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected() > 0)
    }
    
    pub async fn list(
        pool: &SqlitePool,
        guild_id: GuildId,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM guild_staff_roles WHERE guild_id = ? ORDER BY created_at DESC"
        )
        .bind(guild_id.get() as i64)
        .fetch_all(pool)
        .await
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct GuildAutoNickname {
    pub guild_id: i64,
    pub nickname_template: String,
    pub set_by: i64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl GuildAutoNickname {
    pub async fn set(
        pool: &SqlitePool,
        guild_id: GuildId,
        template: &str,
        set_by: UserId,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO guild_auto_nicknames (guild_id, nickname_template, set_by)
            VALUES (?, ?, ?)
            ON CONFLICT (guild_id)
            DO UPDATE SET 
                nickname_template = excluded.nickname_template,
                set_by = excluded.set_by,
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(guild_id.get() as i64)
        .bind(template)
        .bind(set_by.get() as i64)
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn get(
        pool: &SqlitePool,
        guild_id: GuildId,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM guild_auto_nicknames WHERE guild_id = ?"
        )
        .bind(guild_id.get() as i64)
        .fetch_optional(pool)
        .await
    }
}
```

### Staff Subcommand Implementation
```rust
// src/commands/settings/staff.rs
use crate::bot::{Context, Error};
use crate::data::models::GuildStaffRole;
use crate::utils::{ResponseHelper, EmbedColor, SettingsError};
use serenity::all::{Role, CreateEmbed};

#[poise::command(slash_command, subcommands("add", "remove", "list"))]
pub async fn staff(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Role to designate as staff"] role: Role,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;
    
    // Add to database
    GuildStaffRole::add(pool, guild_id, role.id, ctx.author().id).await?;
    
    // Use ResponseHelper for consistent success message
    ResponseHelper::send_success(
        ctx, 
        "✅ Staff Role Added",
        &format!("**{}** has been designated as a staff role", role.name)
    ).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Role to remove from staff"] role: Role,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;
    
    let removed = GuildStaffRole::remove(pool, guild_id, role.id).await?;
    
    if removed {
        ResponseHelper::send_success(
            ctx,
            "✅ Staff Role Removed",
            &format!("**{}** is no longer a staff role", role.name)
        ).await?;
    } else {
        ResponseHelper::send_error(
            ctx,
            "❌ Role Not Found",
            &format!("**{}** was not designated as a staff role", role.name)
        ).await?;
    }
    Ok(())
}

#[poise::command(slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;
    
    let staff_roles = GuildStaffRole::list(pool, guild_id).await?;
    
    if staff_roles.is_empty() {
        ResponseHelper::send_info(
            ctx,
            "📋 Staff Roles",
            "No staff roles have been configured"
        ).await?;
    } else {
        let role_list = staff_roles
            .iter()
            .map(|sr| format!("• <@&{}>", sr.role_id))
            .collect::<Vec<_>>()
            .join("\n");
        
        let embed = CreateEmbed::new()
            .title("📋 Staff Roles")
            .description(&role_list)
            .color(EmbedColor::Primary.value())
            .footer(serenity::all::CreateEmbedFooter::new(
                format!("{} staff roles configured", staff_roles.len())
            ));
        
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    }
    Ok(())
}
```

### Config Subcommand with Concurrent Queries
```rust
// src/commands/settings/config.rs
use crate::bot::{Context, Error};
use crate::data::models::{GuildStaffRole, GuildAutoNickname, GuildJoinLogChannel, GuildPremiumRole};
use crate::utils::EmbedColor;
use serenity::all::CreateEmbed;
use tokio::join;

#[poise::command(slash_command)]
pub async fn config(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;
    
    // Fetch all settings concurrently for performance
    let (staff_roles, auto_nick, join_log, premium_role) = join!(
        GuildStaffRole::list(pool, guild_id),
        GuildAutoNickname::get(pool, guild_id),
        GuildJoinLogChannel::get(pool, guild_id),
        GuildPremiumRole::get(pool, guild_id)
    );
    
    // Format staff roles
    let staff_display = match staff_roles {
        Ok(roles) if !roles.is_empty() => {
            roles.iter()
                .map(|r| format!("<@&{}>", r.role_id))
                .collect::<Vec<_>>()
                .join(", ")
        }
        _ => "None configured".to_string()
    };
    
    // Format auto-nickname
    let auto_nick_display = match auto_nick {
        Ok(Some(an)) => format!("`{}`", an.nickname_template),
        _ => "Disabled".to_string()
    };
    
    // Format join log channel
    let join_log_display = match join_log {
        Ok(Some(jl)) => format!("<#{}>", jl.channel_id),
        _ => "Disabled".to_string()
    };
    
    // Format premium role
    let premium_role_display = match premium_role {
        Ok(Some(pr)) => format!("<@&{}>", pr.role_id),
        _ => "None configured".to_string()
    };
    
    // Build response embed with all settings
    let embed = CreateEmbed::new()
        .title("⚙️ Current Guild Settings")
        .color(EmbedColor::Primary.value())
        .field("Staff Roles", staff_display, false)
        .field("Auto-Nickname Template", auto_nick_display, false)
        .field("Join/Leave Logs", join_log_display, false)
        .field("Premium Role", premium_role_display, false)
        .timestamp(serenity::model::Timestamp::now());
    
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
```

### Event Handler Implementation
```rust
// src/handlers/member_handler.rs
use crate::data::models::{GuildAutoNickname, GuildJoinLogChannel};
use crate::utils::EmbedColor;
use serenity::all::{Context, Member, GuildId, CreateEmbed, EditMember};
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct MemberHandler {
    pub db_pool: Arc<SqlitePool>,
}

impl MemberHandler {
    pub fn new(db_pool: Arc<SqlitePool>) -> Self {
        Self { db_pool }
    }
    
    pub async fn handle_member_join(&self, ctx: &Context, new_member: &Member) {
        let guild_id = new_member.guild_id;
        let user_id = new_member.user.id;
        
        // Apply auto-nickname if configured
        if let Err(e) = self.apply_auto_nickname(ctx, new_member).await {
            tracing::error!(
                guild_id = %guild_id,
                user_id = %user_id,
                error = ?e,
                "Failed to apply auto-nickname"
            );
        }
        
        // Send join log if configured
        if let Err(e) = self.send_join_log(ctx, new_member).await {
            tracing::error!(
                guild_id = %guild_id,
                user_id = %user_id,
                error = ?e,
                "Failed to send join log"
            );
        }
    }
    
    async fn apply_auto_nickname(
        &self,
        ctx: &Context,
        member: &Member,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let auto_nick = GuildAutoNickname::get(&self.db_pool, member.guild_id).await?;
        
        if let Some(nick_config) = auto_nick {
            let nickname = self.parse_nickname_template(
                &nick_config.nickname_template,
                &member.user.name,
                member.user.discriminator
            );
            
            // Apply nickname with graceful degradation
            match member.edit(&ctx.http, EditMember::new().nickname(&nickname)).await {
                Ok(_) => {
                    tracing::info!(
                        guild_id = %member.guild_id,
                        user_id = %member.user.id,
                        nickname = %nickname,
                        "Applied auto-nickname to new member"
                    );
                }
                Err(serenity::Error::Http(e)) if e.status_code() == Some(403) => {
                    tracing::warn!(
                        guild_id = %member.guild_id,
                        user_id = %member.user.id,
                        "Cannot apply auto-nickname: missing permissions"
                    );
                }
                Err(e) => return Err(e.into()),
            }
        }
        
        Ok(())
    }
    
    async fn send_join_log(
        &self,
        ctx: &Context,
        member: &Member,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let join_log = GuildJoinLogChannel::get(&self.db_pool, member.guild_id).await?;
        
        if let Some(log_config) = join_log {
            let channel_id = serenity::all::ChannelId::new(log_config.channel_id as u64);
            
            let embed = CreateEmbed::new()
                .title("📥 Member Joined")
                .color(EmbedColor::Success.value())
                .thumbnail(member.user.avatar_url().unwrap_or_default())
                .field("User", format!("{} ({})", member.user.mention(), member.user.tag()), false)
                .field("Account Created", format!("<t:{}:R>", member.user.created_at().unix_timestamp()), true)
                .field("Member Count", member.guild_id.to_guild_cached(&ctx.cache)
                    .map(|g| g.member_count.to_string())
                    .unwrap_or_else(|| "Unknown".to_string()), true)
                .timestamp(serenity::model::Timestamp::now());
            
            channel_id.send_message(&ctx.http, serenity::all::CreateMessage::new()
                .embed(embed)
            ).await?;
        }
        
        Ok(())
    }
    
    fn parse_nickname_template(&self, template: &str, username: &str, discriminator: Option<u16>) -> String {
        let mut result = template.to_string();
        result = result.replace("{username}", username);
        if let Some(disc) = discriminator {
            result = result.replace("{discriminator}", &disc.to_string());
        }
        result
    }
}
```

### Permission Validation Helper
```rust
// src/commands/settings/mod.rs
pub async fn validate_permissions(ctx: &Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let member = ctx.author_member().await
        .ok_or("Cannot fetch member")?;
    
    if !member.permissions()?.manage_guild() {
        return Err(SettingsError::InsufficientPermissions.into());
    }
    
    // Check bot permissions for specific operations
    let bot_member = guild_id.member(&ctx.serenity_context().http, 
        ctx.framework().bot_id).await?;
    
    if !bot_member.permissions()?.manage_roles() {
        ResponseHelper::send_error(ctx, "❌ Bot Permissions",
            "I need **Manage Roles** permission to configure settings").await?;
        return Err("Missing bot permissions".into());
    }
    
    Ok(())
}
```

### Rate Limiter Implementation
```rust
// src/utils/settings_rate_limiter.rs
use std::collections::HashMap;
use tokio::time::{Duration, Instant};
use tokio::sync::RwLock;
use serenity::all::{GuildId, UserId};
use std::sync::Arc;
use crate::bot::Error;

pub struct SettingsRateLimiter {
    limits: Arc<RwLock<HashMap<(GuildId, UserId), Instant>>>,
}

impl SettingsRateLimiter {
    pub fn new() -> Self {
        Self {
            limits: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn check_limit(&self, guild_id: GuildId, user_id: UserId) -> Result<(), Error> {
        let mut limits = self.limits.write().await;
        let key = (guild_id, user_id);
        
        if let Some(last_use) = limits.get(&key) {
            if last_use.elapsed() < Duration::from_secs(60) {
                let remaining = 60 - last_use.elapsed().as_secs();
                return Err(format!("Please wait {} seconds between settings changes", remaining).into());
            }
        }
        
        limits.insert(key, Instant::now());
        Ok(())
    }
}
```

## Step-by-Step Implementation Roadmap

### Phase 1: Database Foundation (Priority: Critical)
1. **Create database migration** (`src/data/database.rs`)
   ```sql
   -- Add guild_staff_roles table
   CREATE TABLE IF NOT EXISTS guild_staff_roles (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       guild_id BIGINT NOT NULL,
       role_id BIGINT NOT NULL,
       added_by BIGINT NOT NULL,
       created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
       updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
       UNIQUE(guild_id, role_id)
   );
   CREATE INDEX idx_guild_staff_roles ON guild_staff_roles(guild_id);
   
   -- Add guild_auto_nicknames table
   CREATE TABLE IF NOT EXISTS guild_auto_nicknames (
       guild_id BIGINT PRIMARY KEY,
       nickname_template TEXT NOT NULL,
       set_by BIGINT NOT NULL,
       created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
       updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
   );
   
   -- Add guild_join_log_channels table  
   CREATE TABLE IF NOT EXISTS guild_join_log_channels (
       guild_id BIGINT PRIMARY KEY,
       channel_id BIGINT NOT NULL,
       set_by BIGINT NOT NULL,
       created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
       updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
   );
   
   -- Add guild_premium_roles table
   CREATE TABLE IF NOT EXISTS guild_premium_roles (
       guild_id BIGINT PRIMARY KEY,
       role_id BIGINT NOT NULL,
       set_by BIGINT NOT NULL,
       created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
       updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
   );
   
   -- Add settings_audit_log table
   CREATE TABLE IF NOT EXISTS settings_audit_log (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       guild_id BIGINT NOT NULL,
       user_id BIGINT NOT NULL,
       action TEXT NOT NULL,
       details TEXT,
       timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
   );
   CREATE INDEX idx_settings_audit_log ON settings_audit_log(guild_id, timestamp);
   ```

2. **Implement database models** (`src/data/models/guild_settings.rs`)
   - Use `#[derive(Debug, Clone, FromRow)]` for all structs
   - Implement async CRUD methods with proper error handling
   - Use `tracing` for database query logging
   - Bind parameters with `.get() as i64` for ID conversions
   - Return appropriate types (Option for get, bool for delete)

### Phase 2: Command Structure (Priority: High)
3. **Create settings command module** (`src/commands/settings/mod.rs`)
   - Add type aliases: `pub type SettingsContext<'a> = Context<'a>;`
   - Import ResponseHelper and EmbedColor from utils
   - Define parent command with all Poise attributes
   - Use `broadcast_typing` for long operations
   - Implement permission validation helper function

4. **Implement config subcommand** (`src/commands/settings/config.rs`)
   - Use `tokio::join!` for concurrent database queries
   - Format results using EmbedColor::Primary (#C6AC80)
   - Handle None/error cases gracefully with default text
   - Add timestamp to embed using `serenity::model::Timestamp::now()`

5. **Implement staff subcommand** (`src/commands/settings/staff.rs`)
   - Create parent command with empty body
   - Implement `add`, `remove`, `list` as subcommands
   - Use ResponseHelper for all responses:
     - `ResponseHelper::send_success` for additions
     - `ResponseHelper::send_error` for failures
     - `ResponseHelper::send_info` for empty lists
   - Validate role belongs to guild before database operations

### Phase 3: Feature Implementation (Priority: High)
6. **Implement autonick subcommand** (`src/commands/settings/autonick.rs`)
   - Validate nickname: max 32 chars, no @everyone/@here
   - Support placeholders: `{username}`, `{discriminator}`
   - Use SettingsError::InvalidNickname for validation errors
   - Store with `ON CONFLICT DO UPDATE` for upsert
   - Special handling for "disable" or "off" to remove setting
   - Use ResponseHelper::send_success with preview of template

7. **Implement joinlogs subcommand** (`src/commands/settings/joinlogs.rs`)
   - Check bot has SEND_MESSAGES and EMBED_LINKS in channel
   - Validate channel is text-based (not voice/stage)
   - Store channel ID with user who configured it
   - Handle "disable" keyword to remove setting
   - Send test embed to confirm configuration
   - Consider webhook creation for better rate limits

8. **Implement premiumrole subcommand** (`src/commands/settings/premiumrole.rs`)
   - Check role hierarchy (bot's highest role > target role)
   - Validate role is not @everyone or managed
   - Store in database with configuring user ID
   - Use SettingsError::RoleHierarchyError for conflicts
   - Option to disable by passing "none" or no arguments

### Phase 4: Event Handler Integration (Priority: Medium)
9. **Create member event handler** (`src/handlers/member_handler.rs`)
   - Create struct with `Arc<SqlitePool>` for database access
   - Implement `new()` constructor taking db_pool
   - Handle `FullEvent::GuildMemberAddition` in framework
   - Handle `FullEvent::GuildMemberRemoval` for leave logs
   - Use separate error handling for each operation (don't fail all if one fails)

10. **Implement auto-nickname logic**
    - Query `GuildAutoNickname::get()` for template
    - Parse placeholders: `{username}` → member.user.name
    - Use `EditMember::new().nickname()` for application
    - Graceful degradation for 403 Forbidden (missing perms)
    - Log with `tracing::info!` on success, `tracing::warn!` on permission failure

11. **Implement join/leave logging**
    - Create embeds using EmbedColor::Success for joins, EmbedColor::Error for leaves
    - Include: user mention, tag, avatar, account age, member count
    - Use Discord timestamp formatting: `<t:{}:R>` for relative times
    - Send via `channel_id.send_message()` with embed
    - Consider caching channel to reduce API calls

### Phase 5: Integration and Testing (Priority: High)
12. **Register commands and handlers** (`src/bot/framework.rs`)
    ```rust
    // Add to commands list
    commands![
        // ... existing commands
        settings::settings(),
    ]
    
    // In event_handler match
    FullEvent::GuildMemberAddition { new_member } => {
        member_handler.handle_member_join(ctx, new_member).await;
    }
    FullEvent::GuildMemberRemoval { guild_id, user, .. } => {
        member_handler.handle_member_leave(ctx, *guild_id, user).await;
    }
    ```

13. **Add permission checks**
    - Use macro: `required_permissions = "MANAGE_GUILD"`
    - Runtime validation with `validate_permissions()` helper
    - Check bot permissions: MANAGE_ROLES, MANAGE_NICKNAMES
    - Return SettingsError::InsufficientPermissions
    - Use ResponseHelper::send_error for permission failures

14. **Implement rate limiting**
    - Create SettingsRateLimiter with 60-second cooldown
    - Store in Data struct as `Arc<SettingsRateLimiter>`
    - Check before any settings modification
    - Use `tokio::time::Instant` for tracking
    - Return remaining cooldown time in error message

### Phase 6: Polish and Optimization (Priority: Low)
15. **Add caching layer**
    - Use `DashMap<GuildId, CachedSettings>` for thread-safe cache
    - TTL of 5 minutes using `tokio::time::Instant`
    - Invalidate on any settings modification
    - Pre-fetch on bot startup for active guilds
    - Significant performance boost for event handlers

16. **Enhance error messages**
    - Create detailed error embeds with EmbedColor::Error
    - Include troubleshooting steps in descriptions
    - Add footer with support server invite
    - Suggest permission fixes: "Grant me Manage Roles permission"
    - Link to web dashboard when available

17. **Add audit logging**
    - Store in `settings_audit_log` table
    - Track: guild_id, user_id, action, details, timestamp
    - Query with `/settings audit` subcommand (admin only)
    - Optional webhook for real-time audit notifications
    - Retain for 30 days, auto-cleanup older entries

## Testing Implementation

### Unit Tests
```rust
// tests/settings_commands_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    
    async fn create_test_pool() -> SqlitePool {
        SqlitePool::connect("sqlite::memory:").await.unwrap()
    }
    
    #[tokio::test]
    async fn test_staff_role_crud() {
        let pool = create_test_pool().await;
        crate::data::database::init_database_tables(&pool).await.unwrap();
        
        let guild_id = GuildId::new(123456789);
        let role_id = RoleId::new(987654321);
        let user_id = UserId::new(111111111);
        
        // Test add
        let result = GuildStaffRole::add(&pool, guild_id, role_id, user_id).await;
        assert!(result.is_ok());
        
        // Test list
        let roles = GuildStaffRole::list(&pool, guild_id).await.unwrap();
        assert_eq!(roles.len(), 1);
        assert_eq!(roles[0].role_id, role_id.get() as i64);
        
        // Test remove
        let removed = GuildStaffRole::remove(&pool, guild_id, role_id).await.unwrap();
        assert!(removed);
        
        // Verify removed
        let roles = GuildStaffRole::list(&pool, guild_id).await.unwrap();
        assert!(roles.is_empty());
    }
    
    #[test]
    fn test_nickname_template_parsing() {
        let handler = MemberHandler::new(Arc::new(pool));
        
        // Test username placeholder
        let result = handler.parse_nickname_template("New-{username}", "TestUser", None);
        assert_eq!(result, "New-TestUser");
        
        // Test discriminator placeholder
        let result = handler.parse_nickname_template("{username}#{discriminator}", "User", Some(1234));
        assert_eq!(result, "User#1234");
        
        // Test no placeholders
        let result = handler.parse_nickname_template("StaticNick", "User", None);
        assert_eq!(result, "StaticNick");
    }
    
    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = SettingsRateLimiter::new();
        let guild_id = GuildId::new(123);
        let user_id = UserId::new(456);
        
        // First attempt should succeed
        assert!(limiter.check_limit(guild_id, user_id).await.is_ok());
        
        // Immediate second attempt should fail
        assert!(limiter.check_limit(guild_id, user_id).await.is_err());
        
        // Wait and retry
        tokio::time::sleep(Duration::from_secs(61)).await;
        assert!(limiter.check_limit(guild_id, user_id).await.is_ok());
    }
}
```

### Integration Tests
```rust
// tests/integration/settings_integration.rs
#[tokio::test]
async fn test_settings_command_flow() {
    let ctx = create_mock_context().await;
    
    // Test parent command
    let result = settings(ctx.clone()).await;
    assert!(result.is_ok());
    
    // Test config with no settings
    let result = config(ctx.clone()).await;
    assert!(result.is_ok());
    
    // Test staff add with permissions
    let role = create_mock_role();
    let result = staff::add(ctx.clone(), role).await;
    assert!(result.is_ok());
}
```

## Security Considerations
- Validate all user input for SQL injection
- Check Discord permissions before operations
- Rate limit database writes
- Validate role/channel IDs belong to guild
- Sanitize nicknames for Discord markdown
- Prevent privilege escalation via staff roles

## Performance Optimizations
- Batch database operations where possible
- Use prepared statements for queries
- Implement connection pooling (already exists)
- Consider using Redis for settings cache
- Optimize join log embeds for size
- Use webhooks for join logs (better rate limits)

## Future Enhancement Opportunities
- Settings import/export functionality
- Settings templates for common configurations
- Webhook integration for external logging
- Settings dashboard web interface
- Backup and restore functionality
- Multi-language support for messages
- Custom join/leave messages with variables
- Role-based setting permissions
- Settings sync across multiple guilds
- Analytics on setting usage patterns

## Dependencies and Requirements
- Poise 0.6+ for command framework
- Serenity 0.12+ for Discord API
- SQLx for database operations
- Tokio for async runtime
- Tracing for structured logging
- Chrono for timestamp handling

## Estimated Implementation Timeline
- Phase 1 (Database): 2-3 hours
- Phase 2 (Commands): 3-4 hours  
- Phase 3 (Features): 4-5 hours
- Phase 4 (Events): 3-4 hours
- Phase 5 (Integration): 2-3 hours
- Phase 6 (Polish): 2-3 hours
- **Total: 16-22 hours**

## Risk Mitigation
- Database migrations should be reversible
- Feature flags for gradual rollout
- Comprehensive error handling
- Fallback to defaults if settings corrupted
- Regular database backups
- Monitor Discord API rate limits
- Graceful degradation if services unavailable

## Success Metrics
- Commands respond within 200ms
- Event handlers process within 50ms
- Zero data loss on settings changes
- 99.9% uptime for core functionality
- Clear audit trail for all changes
- Positive user feedback on UX

## Development Guidelines Alignment

This enhanced roadmap strictly adheres to the Rust Discord Bot Development Guidelines:

### ✅ Type System
- Uses type aliases (`SettingsContext<'a>`)
- Proper error types with `thiserror`
- FromRow pattern for all database models

### ✅ Color System
- All embeds use `EmbedColor` enum
- Standardized colors: Primary (#C6AC80), Success (#62CB77), Error (#853535), Warning (#FFE209)
- No hardcoded color values

### ✅ Response Pattern
- Consistent use of `ResponseHelper`
- `send_success`, `send_error`, `send_info` for all user responses
- Proper embed formatting with timestamps

### ✅ Command Structure
- Poise macros with all required attributes
- Subcommand patterns following boosterrole example
- Permission checks in macros and runtime

### ✅ Async Patterns
- `tokio::join!` for concurrent operations
- Proper error handling with `?` operator
- No blocking operations in async context

### ✅ Database Patterns
- SQLx with prepared statements
- Proper parameter binding with `.get() as i64`
- Transaction support where needed

### ✅ Error Handling
- Custom error types with context
- Graceful degradation for permissions
- Comprehensive logging with `tracing`

### ✅ Testing Strategy
- Unit tests with `#[tokio::test]`
- Integration tests for command flows
- Mock contexts for isolated testing

### ✅ Security Best Practices
- Input validation and sanitization
- Permission checks (dual-layer)
- Rate limiting implementation
- No hardcoded secrets

### ✅ Performance Optimizations
- Concurrent database queries
- Caching layer for frequent reads
- Connection pooling (existing)
- Efficient event handling