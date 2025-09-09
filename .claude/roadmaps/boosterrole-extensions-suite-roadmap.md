# Boosterrole Command Extensions Suite Implementation Roadmap

## Feature Overview

**Implementation of 9 missing boosterrole subcommands that extend the existing boosterrole functionality with enhanced compliance to development guidelines:**

### Role Customization Commands (Booster Only)
- `boosterrole icon` - Set custom role icons via URL
- `boosterrole random` - Generate random hex color roles
- `boosterrole remove` - Remove custom booster roles

### Administrative Commands (Manage Guild Required)
- `boosterrole base` - Set hierarchical positioning for booster roles

### Sharing System Commands
- `boosterrole share` - Share booster roles with other members (Public)
- `boosterrole share remove` - Remove yourself from shared roles (Public)
- `boosterrole share max` - Set max members per shared role (Admin)
- `boosterrole share list` - View all members in booster roles (Admin)
- `boosterrole share limit` - Set max shared roles per member (Admin)

## Discord Bot Architecture Analysis

**Current Infrastructure:**
- **Framework:** Poise v0.6 (built on Serenity v0.12)
- **Database:** SQLite with SQLx connection pooling
- **Module Structure:** Modular boosterrole subcommands in `/src/commands/boosterrole/`
- **Existing Tables:** `booster_roles`, `booster_role_links`, `role_name_blacklist`, `guild_booster_limits`, `guild_booster_awards`, `booster_rename_history`

**Discord Interaction Flow:**
1. **Slash Command Reception** → Poise framework routing → Subcommand handler
2. **Permission Validation** → Role/permission checks → Database queries
3. **Discord API Calls** → Role creation/modification → User assignment
4. **Database Persistence** → SQLite updates → Logging with tracing
5. **Response Generation** → Embed creation → Discord webhook response

## Required Database Schema Extensions

### New Tables Required

#### 1. Enhanced Role Sharing System
```sql
CREATE TABLE IF NOT EXISTS booster_role_shares (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,           -- The shared booster role
    owner_id BIGINT NOT NULL,          -- Original role creator
    shared_with_id BIGINT NOT NULL,    -- User who has access
    shared_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NULL,         -- Optional expiration
    is_active BOOLEAN DEFAULT TRUE,
    
    CONSTRAINT unique_role_share UNIQUE(guild_id, role_id, shared_with_id),
    CONSTRAINT fk_guild_limits FOREIGN KEY(guild_id) 
        REFERENCES guild_sharing_limits(guild_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS guild_sharing_limits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id BIGINT NOT NULL UNIQUE,
    max_members_per_role INTEGER DEFAULT 5,     -- Max users per shared role
    max_shared_roles_per_member INTEGER DEFAULT 3, -- Max shared roles per user
    set_by BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Performance indices
CREATE INDEX IF NOT EXISTS idx_role_shares_guild_id ON booster_role_shares(guild_id);
CREATE INDEX IF NOT EXISTS idx_role_shares_owner_id ON booster_role_shares(owner_id);
CREATE INDEX IF NOT EXISTS idx_role_shares_shared_with ON booster_role_shares(shared_with_id);
CREATE INDEX IF NOT EXISTS idx_role_shares_active ON booster_role_shares(is_active) WHERE is_active = TRUE;
```

#### 2. Role Base Hierarchy
```sql
CREATE TABLE IF NOT EXISTS guild_booster_base_roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id BIGINT NOT NULL UNIQUE,
    base_role_id BIGINT NOT NULL,      -- Role to position booster roles above
    set_by BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## Development Guidelines Compliance Enhancements

### Critical Enhancement Areas

#### 1. Rate Limiting System Implementation
```rust
use governor::{Quota, RateLimiter, DefaultKeyedStateStore, DefaultClock};
use std::num::NonZeroU32;

pub struct BoosterRoleRateLimiter {
    icon_limiter: RateLimiter<u64, DefaultKeyedStateStore<u64>, DefaultClock>,
    random_limiter: RateLimiter<u64, DefaultKeyedStateStore<u64>, DefaultClock>,
    share_limiter: RateLimiter<u64, DefaultKeyedStateStore<u64>, DefaultClock>,
}

impl BoosterRoleRateLimiter {
    pub fn new() -> Self {
        Self {
            // Icon changes: 3 per hour (resource intensive)
            icon_limiter: RateLimiter::keyed(
                Quota::per_hour(NonZeroU32::new(3).unwrap())
            ),
            // Random colors: 10 per hour
            random_limiter: RateLimiter::keyed(
                Quota::per_hour(NonZeroU32::new(10).unwrap())
            ),
            // Sharing operations: 20 per hour
            share_limiter: RateLimiter::keyed(
                Quota::per_hour(NonZeroU32::new(20).unwrap())
            ),
        }
    }
    
    pub fn check_icon_limit(&self, user_id: u64) -> Result<(), BoosterRoleError> {
        self.icon_limiter
            .check_key(&user_id)
            .map_err(|_| BoosterRoleError::RateLimitExceeded {
                command: "icon".to_string(),
                reset_time: "1 hour".to_string(),
            })
    }
}
```

#### 2. Response System Standardization
All commands must use the established ResponseHelper utilities and standardized color system for consistent user experience across the bot.

#### 3. Enhanced Error Types with thiserror
Custom error types provide better error context and user-friendly messages while maintaining developer debugging information.

#### 4. Comprehensive Testing Strategy
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use tokio_test;
    
    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        init_database(&pool).await.unwrap();
        pool
    }
    
    #[tokio::test]
    async fn test_sharing_limits() {
        let pool = setup_test_db().await;
        
        // Test limit enforcement
        let guild_id = 12345;
        let owner_id = 67890;
        
        // Set sharing limit to 2
        GuildSharingLimit::set(&pool, guild_id, 2, 3, owner_id).await.unwrap();
        
        // Should allow sharing up to limit
        for i in 0..2 {
            assert!(check_sharing_limit(&pool, guild_id, owner_id).await.is_ok());
        }
        
        // Should reject when limit exceeded
        assert!(check_sharing_limit(&pool, guild_id, owner_id).await.is_err());
    }
    
    #[tokio::test]
    async fn test_rate_limiting() {
        let limiter = BoosterRoleRateLimiter::new();
        let user_id = 12345;
        
        // Should allow within limit
        assert!(limiter.check_icon_limit(user_id).is_ok());
        
        // Should block when limit exceeded
        for _ in 0..3 {
            let _ = limiter.check_icon_limit(user_id);
        }
        assert!(limiter.check_icon_limit(user_id).is_err());
    }
}
```

## Implementation Strategy

### Phase 1: Core Infrastructure Setup
**Branch:** `feature/boosterrole-extensions-suite`

#### 1.1 Database Schema Implementation
- **Files:** `src/data/database.rs`, `src/data/models.rs`
- **Models:** `BoosterRoleShare`, `GuildSharingLimit`, `GuildBoosterBaseRole`
- **Operations:** CRUD methods with proper error handling and tracing

#### 1.2 Utility Extensions  
- **Files:** `src/utils/role_manager.rs`, `src/utils/color_generator.rs` (new), `src/utils/rate_limiter.rs` (new)
- **Features:** Role hierarchy management, random color generation, role icon validation, rate limiting system

### Phase 2: Individual Command Implementation

#### 2.1 Role Customization Commands
**Files:** 
- `src/commands/boosterrole/icon.rs` (new)
- `src/commands/boosterrole/random.rs` (new) 
- `src/commands/boosterrole/remove.rs` (new)

**Implementation Details:**

##### `boosterrole icon` - Enhanced Implementation
```rust
use crate::utils::{ResponseHelper, EmbedColor, BoosterRoleRateLimiter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BoosterRoleError {
    #[error("Invalid icon URL: {reason}")]
    InvalidIconUrl { reason: String },
    #[error("Rate limit exceeded for {command}: try again in {reset_time}")]
    RateLimitExceeded { command: String, reset_time: String },
    #[error("Database operation failed: {operation}")]
    DatabaseError { operation: String, #[source] source: sqlx::Error },
}

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Booster Roles",
    description = "Set a custom icon for your booster role using a URL",
    /// Set a custom icon for your booster role using a URL
    /// 
    /// This command allows server boosters to customize their booster role
    /// with a custom icon. The icon URL must be a direct link to a PNG, JPG, or GIF image.
    /// 
    /// Requirements:
    /// - Must be a server booster
    /// - URL must be valid and accessible
    /// - Image must be PNG, JPG, or GIF format
    /// - Rate limited to 3 uses per hour
    aliases("ico")
)]
pub async fn icon(
    ctx: Context<'_>,
    #[description = "Direct URL to image file (PNG, JPG, or GIF)"] 
    #[min_length = 10]
    #[max_length = 2048]
    url: String,
) -> Result<(), Error>
```

**Enhanced Discord API Flow:**
1. **Rate Limit Check** → Validate user is booster → Check existing booster role
2. **Input Validation** → Enhanced URL validation with security checks → Image format verification
3. **Concurrent Operations** → Download validation and Discord API preparation
4. **Response System** → Standardized ResponseHelper success/error messages
5. **Structured Logging** → Tracing with context fields → Database persistence

**Input Validation Function:**
```rust
fn validate_icon_url(url: &str) -> Result<String, BoosterRoleError> {
    // Length validation
    if url.len() > 2048 {
        return Err(BoosterRoleError::InvalidIconUrl {
            reason: "URL too long (max 2048 characters)".to_string()
        });
    }
    
    // Format validation
    let url = url.trim();
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(BoosterRoleError::InvalidIconUrl {
            reason: "URL must start with http:// or https://".to_string()
        });
    }
    
    // Extension validation
    let valid_extensions = [".png", ".jpg", ".jpeg", ".gif"];
    if !valid_extensions.iter().any(|ext| url.to_lowercase().ends_with(ext)) {
        return Err(BoosterRoleError::InvalidIconUrl {
            reason: "URL must point to PNG, JPG, or GIF image".to_string()
        });
    }
    
    // Sanitize Discord mentions
    let sanitized = url.replace("@everyone", "@\u{200B}everyone")
                      .replace("@here", "@\u{200B}here");
    
    Ok(sanitized)
}
```

**Enhanced Logging with Structured Context:**
```rust
use tracing::{info, warn, error, instrument};

#[instrument(
    skip(ctx),
    fields(
        user_id = %ctx.author().id,
        guild_id = ?ctx.guild_id(),
        command = "boosterrole.icon"
    )
)]
pub async fn icon(ctx: Context<'_>, url: String) -> Result<(), Error> {
    info!(icon_url = %url, "Icon command invoked");
    
    // Rate limiting check
    let rate_limiter = ctx.data().rate_limiter.clone();
    if let Err(e) = rate_limiter.check_icon_limit(ctx.author().id.0) {
        warn!(error = %e, "Rate limit exceeded");
        return ResponseHelper::send_error(ctx, "Rate Limited", &e.to_string()).await.map_err(Into::into);
    }
    
    // Enhanced validation
    let validated_url = match validate_icon_url(&url) {
        Ok(url) => {
            info!("Icon URL validation successful");
            url
        },
        Err(e) => {
            warn!(error = %e, "Icon URL validation failed");
            return ResponseHelper::send_error(ctx, "Invalid URL", &e.to_string()).await.map_err(Into::into);
        }
    };
    
    // Success response using ResponseHelper
    ResponseHelper::send_success(
        ctx, 
        "✅ Icon Updated", 
        "Your booster role icon has been successfully updated!"
    ).await.map_err(Into::into)
}
```

##### `boosterrole random`
```rust
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    description = "Generate a random color for your booster role",
    aliases("rand")
)]
pub async fn random(ctx: Context<'_>) -> Result<(), Error>
```

**Discord API Flow:**
1. Validate user is booster → Generate random hex color → Update existing role or create new
2. Database persistence → Response with color preview

**Logging:** Random color generation, role updates

##### `boosterrole remove`
```rust
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    description = "Remove your custom booster role",
    aliases("rm", "delete")
)]
pub async fn remove(ctx: Context<'_>) -> Result<(), Error>
```

**Discord API Flow:**
1. Validate user has booster role → Remove from Discord → Database cleanup → Shared role cleanup
2. Confirmation response

**Logging:** Role deletion, cleanup operations

#### 2.2 Administrative Commands
**Files:** `src/commands/boosterrole/base.rs` (new)

##### `boosterrole base`
```rust
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    description = "Set the base role for booster role hierarchy positioning"
)]
pub async fn base(
    ctx: Context<'_>,
    #[description = "Role to position booster roles above"] role: serenity::Role,
) -> Result<(), Error>
```

**Discord API Flow:**
1. Permission validation → Role hierarchy analysis → Database storage
2. Automatic repositioning of existing booster roles → Success confirmation

**Logging:** Base role changes, hierarchy updates

#### 2.3 Sharing System Commands
**Files:** 
- `src/commands/boosterrole/share.rs` (new, with subcommands)

**Subcommand Structure:**
```rust
#[poise::command(slash_command, subcommands("share_role", "share_remove", "share_max", "share_list", "share_limit"))]
pub async fn share(_: Context<'_>) -> Result<(), Error>
```

##### Individual Share Commands Implementation:

**`boosterrole share`** (Public access)
- Validate role ownership → Check sharing limits → Add to sharing table → Assign role to target user

**`boosterrole share remove`** (Public access)  
- Validate user has shared access → Remove from sharing table → Remove role from user

**`boosterrole share max`** (Admin only)
- Set max members per shared role → Database update → Apply to existing shares

**`boosterrole share list`** (Admin only)
- Query all shared roles → Format member lists → Paginated embed response

**`boosterrole share limit`** (Admin only)
- Set max shared roles per member → Database update → Validation for new shares

### Phase 3: Integration and Testing

#### 3.1 Module Registration
**Files:** `src/commands/boosterrole/mod.rs`, `src/bot/framework.rs`

**Updates Required:**
```rust
// Add new imports and subcommands
pub mod icon;
pub mod random;
pub mod remove;  
pub mod base;
pub mod share;

// Update main boosterrole command subcommands array
subcommands(
    "color", "dominant", "rename", "link", "filter", "list", 
    "cleanup", "limit", "award", "icon", "random", "remove", 
    "base", "share"
)
```

#### 3.2 Database Migration Integration
**Files:** `src/data/database.rs`

**Migration Strategy:**
- Add new table creation to `init_database()` function
- Implement proper indices for performance
- Add migration versioning for production deployments

## Key Implementation Considerations

### Discord API Constraints
- **Rate Limits:** Role modification limited to 250 requests per 5 minutes
- **Role Limits:** 250 roles per guild maximum
- **Permission Requirements:** Bot needs `MANAGE_ROLES` permission
- **Hierarchy Rules:** Bot role must be above target roles in hierarchy

### Enhanced Async Execution Patterns
```rust
use tokio::join;

// Replace sequential operations with concurrent ones where possible
// Enhanced concurrent approach with proper error handling:
let (role_result, limit_result) = join!(
    BoosterRole::get(&pool, guild_id, user_id),
    GuildSharingLimit::get(&pool, guild_id)
);

let role_data = role_result.map_err(|e| BoosterRoleError::DatabaseError {
    operation: "fetch booster role".to_string(),
    source: e,
})?;

let limit_data = limit_result.map_err(|e| BoosterRoleError::DatabaseError {
    operation: "fetch sharing limits".to_string(), 
    source: e,
})?;
```

### Logging Strategy with Tracing
```rust
// Command invocations
tracing::info!(
    user_id = %user_id,
    guild_id = %guild_id,
    command = "boosterrole.icon",
    icon_url = %url,
    "Icon command invoked"
);

// API responses  
tracing::warn!(
    user_id = %user_id,
    guild_id = %guild_id,
    error = ?e,
    "Failed to download icon from URL"
);

// Database operations
tracing::debug!(
    "Database query: create_role_share for role {} shared with user {}",
    role_id, target_user_id
);
```

## Step-by-Step Implementation Roadmap

### Step 1: Repository Setup
```bash
git checkout -b feature/boosterrole-extensions-suite
```

### Step 2: Database Schema Implementation (Day 1-2)
1. **Extend database.rs:** Add new table creation queries
2. **Implement models.rs:** Create data structures with proper SQLx derives
3. **Add database indices:** Optimize for sharing queries and role lookups
4. **Test database migrations:** Ensure backward compatibility

### Step 3: Utility Function Development (Day 2-3)  
1. **Color generation utility:** Random hex color generator with validation
2. **Role hierarchy management:** Functions for base role positioning
3. **Icon validation:** URL format checking and image validation
4. **Sharing limit validation:** Enforce per-guild and per-user limits

### Step 4: Core Command Implementation (Day 3-7)
**Priority Order:**
1. `remove` command (simplest, builds on existing patterns)
2. `random` command (moderate complexity, reuses existing color logic)
3. `icon` command (complex, requires URL validation and Discord API calls)
4. `base` command (moderate complexity, role hierarchy management)
5. `share` subcommands (most complex, new database relationships)

### Step 5: Integration Testing (Day 7-8)
1. **Unit tests:** Individual command logic validation
2. **Integration tests:** Full command workflow testing  
3. **Database tests:** Sharing relationship integrity
4. **Permission tests:** Ensure proper access control

### Step 6: Command Registration and Deployment (Day 8-9)
1. **Update module exports:** Add new commands to boosterrole mod.rs
2. **Framework integration:** Register subcommands in bot/framework.rs
3. **Development deployment:** Test in development guild
4. **Production deployment:** Global command registration

## Verification and Testing Strategy

### Discord Interaction Testing
```bash
# Test booster-only commands
/boosterrole icon https://example.com/icon.png
/boosterrole random  
/boosterrole remove

# Test admin commands
/boosterrole base @BaseRole
/boosterrole share max 10

# Test sharing system
/boosterrole share @User
/boosterrole share list
/boosterrole share remove @MyBoosterRole
```

### Logging Validation Points
- **Command Invocations:** All 9 new commands logged with user context
- **Permission Failures:** Unauthorized access attempts logged with details
- **API Errors:** Discord API failures logged with retry context
- **Database Operations:** All sharing table modifications logged
- **Success Confirmations:** Successful command executions logged

### Error Handling Scenarios
- **Invalid URLs:** Proper validation and user feedback for icon command
- **Sharing Limit Exceeded:** Clear error messages with current limits
- **Role Not Found:** Handle deleted roles gracefully
- **Permission Denied:** Consistent error responses across commands
- **Database Failures:** Graceful fallback with user notification

## Health Check Integration

### Boosterrole System Health Monitoring
```rust
// Add to existing health check system
pub async fn boosterrole_health_check(data: &Data) -> Result<BoosterRoleHealthStatus, Error> {
    let pool = &data.db_pool;
    
    // Test database connectivity
    let db_check = sqlx::query("SELECT COUNT(*) FROM booster_role_shares")
        .fetch_one(pool)
        .await
        .is_ok();
    
    // Test sharing system integrity
    let integrity_check = sqlx::query(r#"
        SELECT COUNT(*) 
        FROM booster_role_shares brs 
        LEFT JOIN guild_sharing_limits gsl ON brs.guild_id = gsl.guild_id
        WHERE gsl.guild_id IS NULL
    "#)
    .fetch_one(pool)
    .await
    .map(|row| {
        let count: i64 = row.get(0);
        count == 0
    })
    .unwrap_or(false);
    
    Ok(BoosterRoleHealthStatus {
        database_connectivity: db_check,
        sharing_integrity: integrity_check,
        active_shares: get_active_share_count(pool).await.unwrap_or(0),
    })
}
```

## Expected Technical Outcomes

### Enhanced Database Performance
- **Optimized queries with indices:** Proper indexing on guild_id, user_id, role_id combinations
- **Efficient sharing lookups:** Single query to get all shared roles for user with foreign key constraints
- **Fast limit validation:** Indexed checks without full table scans
- **Database integrity:** Foreign key relationships ensure data consistency

### Enhanced Discord API Efficiency  
- **Concurrent operations:** Strategic use of tokio::join! for parallel database queries
- **Rate limiting:** User-level rate limits prevent abuse and API exhaustion
- **Error handling:** Comprehensive error types with user-friendly messages
- **Role hierarchy management:** Efficient positioning updates with base role system

### Superior User Experience Improvements
- **Standardized responses:** All commands use ResponseHelper for consistent styling
- **Comprehensive validation:** Input sanitization and format validation for security
- **Clear error messages:** Detailed feedback with actionable solutions for all failure scenarios
- **Progressive permissions:** Appropriate access levels for each command with proper validation
- **Intuitive aliases:** Short command alternatives for frequent use
- **Rate limit feedback:** Clear communication of limits and reset times

## Compliance Checklist

### Development Guidelines Compliance
- [x] **ResponseHelper Integration:** All commands use standardized response system
- [x] **Enhanced Error Handling:** Custom error types with thiserror and proper context
- [x] **Input Validation:** Comprehensive validation with sanitization for all user inputs
- [x] **Rate Limiting:** User-level rate limits implemented for resource-intensive operations
- [x] **Structured Logging:** Tracing with instrument macro and contextual fields
- [x] **Performance Optimizations:** Concurrent async operations where applicable
- [x] **Command Documentation:** Comprehensive help text and parameter validation
- [x] **Database Optimization:** Proper indexing, constraints, and foreign keys
- [x] **Testing Strategy:** Unit tests, integration tests, and rate limiting tests
- [x] **Health Monitoring:** Integration with existing health check systems

### Implementation Priority
1. **High Priority:** Response system, error handling, input validation, rate limiting
2. **Medium Priority:** Database schema enhancements, structured logging, testing
3. **Lower Priority:** Health checks, documentation improvements, performance optimizations

### Security Enhancements
- **Input Sanitization:** All URLs and text inputs sanitized against Discord mention exploits
- **Rate Limiting:** Prevents abuse of resource-intensive operations like icon updates
- **Permission Validation:** Proper role and permission checks at multiple levels
- **Database Security:** Foreign key constraints prevent orphaned records and data integrity issues

This enhanced roadmap provides a comprehensive implementation strategy for the 9 missing boosterrole subcommands with full compliance to development guidelines, ensuring proper integration with existing infrastructure while maintaining security, performance, and user experience standards.