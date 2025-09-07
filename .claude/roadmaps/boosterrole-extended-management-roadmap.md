# Boosterrole Extended Management Commands - Implementation Roadmap

## Feature Summary

Comprehensive suite of additional boosterrole management commands extending the existing functionality with administrative controls and enhanced user customization options. This includes automated cleanup, role limits, renaming capabilities, and an award system for new boosters.

## Discord Interaction Flow Analysis

### Command Flow Patterns

1. **Cleanup Command Flow** (`/boosterrole cleanup`)
   - User invokes slash command → Check Manage Guild permission
   - Query database for all booster_roles in guild
   - Verify each role against Discord API (member boost status)
   - Collect orphaned roles (no boost or member left)
   - Optional: dry-run preview
   - Delete orphaned roles from Discord and database
   - Send embed with cleanup statistics

2. **Limit Command Flow** (`/boosterrole limit`)
   - User invokes slash command → Check Manage Guild permission
   - Parse limit parameter (optional, 0 = unlimited)
   - Store/update limit in guild_booster_limits table
   - Send confirmation embed with new limit

3. **Rename Command Flow** (`/boosterrole rename`)
   - User invokes slash command → Verify user is booster
   - Check if user has existing booster role
   - Validate new name against filters
   - Check rename rate limit (prevent spam)
   - Update role name in Discord and database
   - Send success embed

4. **Award System Flow** (`/boosterrole award`)
   - Set: Admin sets reward role → Store in guild_booster_awards table
   - View: Display current award role configuration
   - Unset: Remove award role configuration
   - Event: Member boosts → Check award config → Auto-assign role

### Event Handler Integration

- **GuildMemberUpdate Event**: Detect boost status changes
- **Ready Event**: Initialize cleanup schedules
- **RoleDelete Event**: Clean orphaned database entries

## Module and Command Structure

### Primary Modules to Modify/Add

1. **src/commands/boosterrole/cleanup.rs** (NEW)
   - Implement cleanup subcommand
   - Admin permission checks
   - Orphaned role detection logic
   - Dry-run option support

2. **src/commands/boosterrole/limit.rs** (NEW)
   - Implement limit subcommand
   - Guild-specific limit storage
   - Limit enforcement checks

3. **src/commands/boosterrole/rename.rs** (NEW)
   - Implement rename subcommand
   - Name validation against filters
   - Rate limiting implementation

4. **src/commands/boosterrole/award.rs** (NEW)
   - Implement award subcommand with set/unset/view
   - Role validation and storage
   - Integration with boost events

5. **src/commands/boosterrole/mod.rs** (MODIFY)
   - Add new subcommand exports
   - Update help text with new commands
   - Register subcommands in parent command

6. **src/handlers/boost_handler.rs** (MODIFY)
   - Add award role assignment on boost
   - Integrate limit checking
   - Enhanced cleanup scheduling

7. **src/data/models.rs** (MODIFY)
   - Add `GuildBoosterLimit` model
   - Add `GuildBoosterAward` model
   - Add rename tracking model

8. **src/data/database.rs** (MODIFY)
   - Add migration for guild_booster_limits table
   - Add migration for guild_booster_awards table
   - Add migration for booster_rename_history table

9. **src/utils/role_manager.rs** (MODIFY)
   - Add limit enforcement utilities
   - Add batch cleanup utilities
   - Add award assignment helpers

## Database Schema Extensions

### New Tables Required

```sql
-- Guild-specific booster role limits
CREATE TABLE IF NOT EXISTS guild_booster_limits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id BIGINT NOT NULL UNIQUE,
    max_roles INTEGER NOT NULL DEFAULT 0, -- 0 = unlimited
    set_by BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Guild award role configuration
CREATE TABLE IF NOT EXISTS guild_booster_awards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id BIGINT NOT NULL UNIQUE,
    award_role_id BIGINT NOT NULL,
    set_by BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Rename history for rate limiting
CREATE TABLE IF NOT EXISTS booster_rename_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    old_name TEXT NOT NULL,
    new_name TEXT NOT NULL,
    renamed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_rename_user (guild_id, user_id, renamed_at)
);
```

## Implementation Hypotheses

### Approach 1: Event-Driven Cleanup (Recommended)
- Leverage existing `BoostHandler::handle_boost_change`
- Schedule periodic cleanup via tokio intervals
- React to member leave/role delete events
- Pros: Automated, efficient, real-time
- Cons: Requires careful event handling

## Branch Strategy

```bash
git checkout -b feature/boosterrole-extended-management
```

## Logging Specifications

### Using `tracing` Crate

1. **Command Execution Logging**
```rust
// In cleanup.rs
tracing::info!(
    guild_id = %guild_id,
    admin_id = %ctx.author().id,
    dry_run = dry_run,
    "Boosterrole cleanup initiated"
);

tracing::debug!(
    orphaned_count = orphaned_roles.len(),
    "Found orphaned roles for cleanup"
);
```

2. **Database Operation Logging**
```rust
// In models.rs
tracing::debug!(
    "Database query: set_guild_booster_limit for guild {}",
    guild_id
);

tracing::error!(
    guild_id = %guild_id,
    error = ?e,
    "Failed to set booster limit"
);
```

3. **Event Handler Logging**
```rust
// In boost_handler.rs
tracing::info!(
    user_id = %user_id,
    guild_id = %guild_id,
    award_role_id = %role_id,
    "Assigning award role to new booster"
);
```

4. **Rate Limit Logging**
```rust
// In rename.rs
tracing::warn!(
    user_id = %user_id,
    cooldown_remaining = ?remaining,
    "Rename rate limit hit"
);
```

## Step-by-Step Implementation Roadmap

### Phase 1: Database Schema Setup
1. Create migration for new tables in `src/data/database.rs`
2. Implement model structs in `src/data/models.rs`:
   - `GuildBoosterLimit` with CRUD operations
   - `GuildBoosterAward` with CRUD operations
   - `BoosterRenameHistory` for tracking
3. Add database initialization in `init_database()`
4. Test database operations with unit tests

### Phase 2: Cleanup System Implementation
1. Create `src/commands/boosterrole/cleanup.rs`:
   - Permission checks (Manage Guild)
   - Orphan detection algorithm
   - Dry-run mode with preview embed
   - Actual cleanup with transaction support
   - Statistics collection and reporting
2. Integrate with `boost_handler.rs`:
   - Enhance `cleanup_orphaned_roles()` method
   - Add scheduled cleanup task
3. Add cleanup subcommand to mod.rs

### Phase 3: Limit System Implementation
1. Create `src/commands/boosterrole/limit.rs`:
   - Set limit command with validation
   - View current limit
   - Permission checks (Manage Guild)
2. Modify `src/commands/boosterrole/color.rs`:
   - Check limit before role creation
   - Return error embed if limit exceeded
3. Add limit enforcement to role creation flow
4. Create helper in `role_manager.rs` for limit checking

### Phase 4: Rename System Implementation
1. Create `src/commands/boosterrole/rename.rs`:
   - Booster status verification
   - Name validation against filters
   - Rate limiting (1 rename per hour)
   - Update Discord role name
   - Update database record
2. Add rename history tracking
3. Implement rate limit check using rename history
4. Add rename subcommand to mod.rs

### Phase 5: Award System Implementation
1. Create `src/commands/boosterrole/award.rs`:
   - Three subcommands: set, unset, view
   - Role validation (exists, not @everyone)
   - Permission checks (Manage Guild + Manage Roles)
2. Modify `boost_handler.rs`:
   - Check for award configuration on boost
   - Auto-assign award role
   - Handle role assignment errors
3. Add award removal on boost expiration
4. Create award management helpers in `role_manager.rs`

### Phase 6: Integration and Testing
1. Update parent command in `src/commands/boosterrole/mod.rs`:
   - Add all new subcommands
   - Update help text
2. Register commands in `src/bot/framework.rs`
3. Create integration tests:
   - Test cleanup with various scenarios
   - Test limit enforcement
   - Test rename with rate limiting
   - Test award assignment
4. Manual testing in development guild

### Phase 7: Error Handling and Edge Cases
1. Handle Discord API rate limits
2. Handle missing permissions gracefully
3. Handle deleted roles/members
4. Transaction rollback on failures
5. Proper error embeds for user feedback

### Phase 8: Documentation and Deployment
1. Update command help text
2. Add usage examples in responses
3. Document admin commands
4. Test in production-like environment
5. Deploy with feature flag if needed

## Poise/Serenity Specific Considerations

### Permission Handling
```rust
#[poise::command(
    slash_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    default_member_permissions = "MANAGE_GUILD"
)]
```

### Subcommand Structure
```rust
#[poise::command(
    slash_command,
    subcommands("set", "unset", "view"),
    subcommand_required
)]
pub async fn award(ctx: Context<'_>) -> Result<(), Error> {
    // Parent command - show help
}
```

### Embed Responses
```rust
use crate::utils::EmbedBuilder;

let embed = EmbedBuilder::success(
    "✅ Cleanup Complete",
    &format!("Removed {} orphaned roles", count)
)
.field("Affected Members", &member_list, false);

ctx.send(poise::CreateReply::default().embed(embed)).await?;
```

### Event Handler Registration
```rust
// In event_handler function
FullEvent::GuildMemberUpdate { old_if_available, new, .. } => {
    if let Some(data) = framework.user_data() {
        data.boost_handler.handle_boost_change(ctx, new).await;
        data.boost_handler.check_award_assignment(ctx, old_if_available, new).await;
    }
}
```

## Key Implementation Details

### Rate Limiting Implementation
- Use tokio RwLock HashMap for in-memory rate limits
- Store last action timestamp per user
- Check elapsed time before allowing action
- Clear old entries periodically

### Orphan Detection Algorithm
1. Get all booster_roles from database for guild
2. For each role:
   - Check if member exists in guild
   - Check if member has premium_since (is booster)
   - Check if role exists in Discord
3. Mark as orphaned if any check fails
4. Batch delete orphaned roles

### Transaction Safety
- Use database transactions for multi-step operations
- Rollback on any Discord API failure
- Log all transaction states
- Implement retry logic for transient failures

### Performance Considerations
- Batch Discord API calls where possible
- Use database indexes on frequently queried columns
- Implement caching for guild settings
- Use Arc<RwLock> for shared state

## Verification Strategy

1. **Unit Tests**: Test individual functions and database operations
2. **Integration Tests**: Test command flows end-to-end
3. **Manual Testing**: Test in development guild with various scenarios
4. **Logging Verification**: Ensure all operations are properly logged
5. **Permission Testing**: Verify all permission checks work correctly
6. **Rate Limit Testing**: Confirm rate limits prevent spam
7. **Edge Case Testing**: Test with deleted roles, left members, etc.

## Success Metrics

- All commands respond within 3 seconds
- Cleanup removes 100% of orphaned roles
- Limits properly enforced with clear error messages
- Rename rate limiting prevents abuse
- Award system automatically assigns roles
- No database inconsistencies
- Comprehensive logging for debugging
- User-friendly error messages

## Risk Mitigation

- **Discord API Downtime**: Cache critical data, queue operations
- **Database Corruption**: Regular backups, transaction safety
- **Rate Limiting**: Implement exponential backoff
- **Permission Changes**: Re-verify permissions at runtime
- **Mass Cleanup**: Implement batch size limits
- **Concurrent Modifications**: Use database locks appropriately

## Future Enhancements (Not in Current Scope)

- Bulk operations for multiple users
- Role templates for common configurations
- Automatic role color gradients
- Role hierarchy management
- Boost duration tracking
- Custom award messages
- Web dashboard for management
- Role backup and restore
- Analytics and usage statistics