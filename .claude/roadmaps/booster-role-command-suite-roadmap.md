# Booster Role Command Suite - Implementation Roadmap

## Feature Overview

A comprehensive command system for Discord server boosters and administrators to manage custom booster roles. The feature enhances the existing single `boosterrole` command into a full-featured suite with subcommands for color customization, role linking, content filtering, and management tools.

## Discord Interaction Flow Analysis

### User Journey Map

#### Booster Flow
1. **Command Invocation**: Booster uses `/boosterrole color #FF0000 "Epic Booster"`
2. **Permission Check**: System verifies user has boost status via `Member.premium_since`
3. **Validation**: Role name checked against blacklist, color validated
4. **Role Creation**: Discord API creates role with specified color
5. **Assignment**: Role assigned to booster member
6. **Database Update**: Role details persisted for management
7. **Response**: Embed confirmation with role mention and color preview

#### Administrator Flow
1. **Management Commands**: Admin uses `/boosterrole link @User @Role` or `/boosterrole filter badword`
2. **Permission Check**: System verifies `MANAGE_GUILD` permission
3. **Action Execution**: Link role to booster or add word to blacklist
4. **Database Update**: Changes persisted
5. **Audit Log**: Action logged with `tracing` for accountability
6. **Response**: Success embed with action details

### Discord API Interactions
- **Role Creation**: `guild.create_role()` with color, name, permissions
- **Member Updates**: `member.add_role()`, `member.remove_role()`
- **Permission Checks**: `member.permissions()?.manage_guild()`
- **Boost Verification**: `member.premium_since.is_some()`
- **Guild Events**: Listen for `GuildMemberUpdate` to handle boost expiration
- **Rate Limiting**: Respect Discord's 2 requests/second for role operations

## Implementation Analysis

### Current State
- **Existing**: Basic `boosterrole` command for color role creation
- **Database**: SQLite with `booster_roles` table
- **Utilities**: `RoleManager`, `ColorParser`, `EmbedBuilder`
- **Framework**: Poise command structure with error handling

### Required Enhancements

#### 1. Command Structure Refactoring
Transform single command into subcommand group:
```rust
#[poise::command(subcommands("color", "link", "filter", "list"))]
pub async fn boosterrole(ctx: Context<'_>) -> Result<(), Error>
```

#### 2. New Database Tables
```sql
-- Role links for administrators
CREATE TABLE booster_role_links (
    id INTEGER PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    linked_role_id BIGINT NOT NULL,
    linked_by BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(guild_id, user_id)
);

-- Word blacklist for filtering
CREATE TABLE role_name_blacklist (
    id INTEGER PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    word TEXT NOT NULL,
    added_by BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(guild_id, word)
);
```

#### 3. Event Handler for Boost Expiration
Monitor `GuildMemberUpdate` events to detect boost removal and cleanup roles automatically.

#### 4. Rate Limiting Implementation
Use `governor` crate or custom implementation to prevent role creation spam.

## Key Modules and Files

### Files to Modify
1. **`src/commands/boosterrole.rs`** - Transform to subcommand structure
2. **`src/data/models.rs`** - Add `BoosterRoleLink` and `RoleNameBlacklist` models
3. **`src/data/database.rs`** - Add new table migrations
4. **`src/utils/role_manager.rs`** - Add link and filter validation methods
5. **`src/handlers/mod.rs`** - Add boost expiration handler

### New Files to Create
1. **`src/commands/boosterrole/mod.rs`** - Main command router
2. **`src/commands/boosterrole/color.rs`** - Color subcommand
3. **`src/commands/boosterrole/link.rs`** - Link subcommand
4. **`src/commands/boosterrole/filter.rs`** - Filter subcommand
5. **`src/commands/boosterrole/list.rs`** - List subcommand
6. **`src/handlers/boost_handler.rs`** - Boost status event handler
7. **`src/utils/content_filter.rs`** - Blacklist checking utilities

## Step-by-Step Implementation Roadmap

### Phase 1: Database Schema Updates
**Branch**: `git checkout -b feature/booster-role-command`

1. **Update `src/data/database.rs`**:
   - Add migrations for `booster_role_links` table
   - Add migrations for `role_name_blacklist` table
   - Add indexes for efficient queries
   - **Logging**: Add `tracing::info!` for migration success

2. **Extend `src/data/models.rs`**:
   - Create `BoosterRoleLink` struct with CRUD methods
   - Create `RoleNameBlacklist` struct with CRUD methods
   - Add `get_all_booster_roles()` for list functionality
   - **Logging**: Add `tracing::debug!` for all database queries

### Phase 2: Refactor Command Structure

3. **Restructure `src/commands/boosterrole.rs`**:
   - Convert to module directory `src/commands/boosterrole/`
   - Create `mod.rs` with parent command using `subcommands` attribute
   - Move existing logic to `color.rs` subcommand
   - **Logging**: Add command invocation logs with user/guild IDs

4. **Implement `color` Subcommand** (`src/commands/boosterrole/color.rs`):
   - Refactor existing boosterrole logic
   - Add second_color parameter support
   - Integrate blacklist checking before role creation
   - **Logging**: Log color parsing, validation, and role creation

### Phase 3: Administrator Features

5. **Implement `link` Subcommand** (`src/commands/boosterrole/link.rs`):
   - Require `MANAGE_GUILD` permission
   - Accept member and role parameters
   - Validate role exists and isn't system role
   - Store link in database
   - **Logging**: Log link creation with admin ID

6. **Implement `filter` Subcommand** (`src/commands/boosterrole/filter.rs`):
   - Add nested subcommands: `add`, `remove`, `list`
   - Require `MANAGE_GUILD` permission
   - Case-insensitive word storage
   - **Logging**: Log filter modifications with admin ID

### Phase 4: Management Tools

7. **Implement `list` Subcommand** (`src/commands/boosterrole/list.rs`):
   - Require `MANAGE_GUILD` permission
   - Fetch all booster roles from database
   - Create paginated embed response
   - Show role name, user, color, creation date
   - **Logging**: Log list command usage

8. **Create Content Filter Utility** (`src/utils/content_filter.rs`):
   - Load blacklist from database with caching
   - Implement efficient word matching
   - Support partial word matching options
   - **Logging**: Log blacklist cache refreshes

### Phase 5: Event Handling

9. **Implement Boost Handler** (`src/handlers/boost_handler.rs`):
   - Listen for `GuildMemberUpdate` events
   - Detect boost removal (premium_since becomes None)
   - Automatically remove booster role
   - Clean up database entries
   - **Logging**: Log automatic role cleanup events

10. **Update Event Router** (`src/bot/framework.rs`):
    - Register boost handler in event_handler function
    - Add handler for role deletion events
    - **Logging**: Log handler registration

### Phase 6: Utilities and Polish

11. **Enhance Role Manager** (`src/utils/role_manager.rs`):
    - Add `link_role_to_booster()` method
    - Add `check_blacklist()` method
    - Add `get_all_booster_roles()` method
    - Improve role position calculation
    - **Logging**: Log role operations

12. **Add Rate Limiting**:
    - Implement per-user rate limiting for role creation
    - Use token bucket algorithm (5 operations per minute)
    - Store limits in memory with TTL
    - **Logging**: Log rate limit hits

### Phase 7: Testing and Verification

13. **Create Integration Tests** (`tests/boosterrole_tests.rs`):
    - Test subcommand routing
    - Test permission checks
    - Test database operations
    - Mock Discord API calls

14. **Manual Testing Checklist**:
    - [ ] Color creation with valid/invalid colors
    - [ ] Blacklist filtering works correctly
    - [ ] Link command assigns existing roles
    - [ ] List command shows all roles
    - [ ] Boost expiration removes roles
    - [ ] Rate limiting prevents spam

15. **Documentation Updates**:
    - Update command help text
    - Add usage examples to README
    - Document administrator features

## Logging Specifications

### Log Levels and Locations

#### Info Level (`tracing::info!`)
- Command invocations: `src/commands/boosterrole/*.rs`
- Role creation/update/deletion: `src/utils/role_manager.rs`
- Database migrations: `src/data/database.rs`
- Boost status changes: `src/handlers/boost_handler.rs`

#### Debug Level (`tracing::debug!`)
- Database queries: `src/data/models.rs`
- Color parsing: `src/utils/color_parser.rs`
- Permission checks: All command files
- Cache operations: `src/utils/content_filter.rs`

#### Warning Level (`tracing::warn!`)
- Rate limit hits: Rate limiter implementation
- Failed role position moves: `src/utils/role_manager.rs`
- Non-booster command attempts: `src/commands/boosterrole/color.rs`
- Orphaned roles detected: `src/utils/role_manager.rs`

#### Error Level (`tracing::error!`)
- Database failures: All database operations
- Discord API errors: All Discord operations
- Critical handler failures: `src/handlers/boost_handler.rs`

### Log Format Examples
```rust
tracing::info!(
    user_id = %ctx.author().id,
    guild_id = %guild_id,
    command = "boosterrole.color",
    role_name = %name,
    color = %color,
    "Booster role color command invoked"
);

tracing::debug!(
    "Database query: get_blacklist for guild {}",
    guild_id
);

tracing::warn!(
    user_id = %user_id,
    "Rate limit exceeded for role creation"
);
```

## Hypothesized Challenges and Solutions

### Challenge 1: Role Hierarchy Management
**Issue**: Bot may not have permission to create roles above certain positions.
**Solution**: Calculate safe position below bot's highest role, handle permission errors gracefully.

### Challenge 2: Concurrent Role Operations
**Issue**: Multiple users creating roles simultaneously could cause race conditions.
**Solution**: Use database transactions and Discord API's idempotent operations.

### Challenge 3: Boost Expiration Detection
**Issue**: Need reliable way to detect when boost expires.
**Solution**: Periodic task + event listener combination for redundancy.

### Challenge 4: Blacklist Performance
**Issue**: Checking every role name against large blacklist could be slow.
**Solution**: In-memory cache with efficient string matching algorithm (Aho-Corasick).

## Success Metrics

1. **Functional Requirements**:
   - All 5 subcommands operational
   - Permission checks enforced
   - Database persistence working
   - Event handlers active

2. **Performance Requirements**:
   - Role creation < 500ms
   - Blacklist check < 50ms
   - List command pagination smooth

3. **User Experience**:
   - Clear error messages
   - Consistent embed formatting
   - Helpful command descriptions
   - Rate limiting prevents spam

## Future Enhancements (Not in Current Scope)

- Gradient role colors (when Discord supports)
- Role templates/presets
- Bulk role management
- Role trading between boosters
- Custom role icons (when Discord supports)
- Analytics dashboard
- Web interface for management

## Conclusion

This roadmap provides a comprehensive plan for implementing the Booster Role Command Suite. The phased approach ensures each component is properly integrated while maintaining the bot's stability. The implementation follows Rust and Discord bot best practices, with emphasis on error handling, logging, and user experience.

**Estimated Timeline**: 2-3 days for full implementation
**Priority**: High - Direct user feature with immediate value
**Risk Level**: Medium - Involves Discord API role operations