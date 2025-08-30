# Booster Role Creation Command Implementation Roadmap

## Feature Summary
A Discord slash command that allows server boosters to create their own custom-styled role with personalized name and color. The command enforces booster-only permissions and manages role creation/assignment automatically.

## Discord Interaction Flow Analysis

### User Journey
1. **Command Invocation**: User types `/boosterrole` in Discord
2. **Argument Input**: Discord shows autocomplete for:
   - `color` (required): Hex color code or color name
   - `second_color` (optional): For future gradient features  
   - `name` (required): Custom role name
3. **Permission Check**: Bot validates user's booster status
4. **Role Processing**: 
   - Check for existing booster role for user
   - Create new role or update existing
   - Position role appropriately in hierarchy
5. **Role Assignment**: Assign role to user
6. **Response**: Send success/error embed

### Discord API Interactions
- `InteractionCreate` event handling
- `Member::premium_since` check for booster status
- `Guild::create_role()` for role creation
- `Member::add_role()` for role assignment
- `Role::edit()` for updates
- Embed response via Poise reply

## Hypothesized Implementation Approaches

### Approach 1: Simple Per-User Role
- One booster role per user
- Store role ID in database linked to user ID
- Update existing role if user runs command again
- **Pros**: Simple, clean role list
- **Cons**: Limited customization history

### Approach 2: Role History with Limits
- Track multiple roles per user (with limit)
- Allow switching between created roles
- Archive old roles
- **Pros**: More flexibility for users
- **Cons**: More complex management

### Approach 3: Guild-Managed Pool (Recommended)
- One active booster role per user
- Store in database with guild_id, user_id, role_id
- Clean up roles when boost expires
- **Pros**: Clean, manageable, follows Discord patterns
- **Cons**: Requires boost expiry handling

## Module Structure Analysis

### New Modules Required
```
src/commands/boosterrole.rs  # Main command implementation
src/data/models/booster_role.rs  # Database model
src/utils/color_parser.rs  # Color parsing utilities
src/utils/role_manager.rs  # Role management helpers
```

### Existing Modules to Modify
```
src/commands/mod.rs  # Export new command
src/bot/framework.rs  # Register command (line 11-17)
src/data/models.rs  # Export new model
src/data/database.rs  # Add migration for booster_roles table
```

## Database Schema Design

### New Table: `booster_roles`
```sql
CREATE TABLE IF NOT EXISTS booster_roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,
    role_name TEXT NOT NULL,
    primary_color TEXT NOT NULL,
    secondary_color TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(guild_id, user_id)  -- One role per user per guild
);

CREATE INDEX idx_booster_roles_guild_user 
ON booster_roles(guild_id, user_id);
```

## Step-by-Step Implementation Roadmap

### Phase 1: Foundation Setup
1. **Create Git Branch**
   ```bash
   git checkout -b feature/booster-role-command
   ```

2. **Database Migration**
   - Edit `src/data/database.rs:41-52`
   - Add booster_roles table creation query
   - Add logging: `tracing::info!("Creating booster_roles table")`

3. **Create Database Model**
   - Create `src/data/models/booster_role.rs`
   - Implement `BoosterRole` struct with SQLx FromRow
   - Methods: `get()`, `create()`, `update()`, `delete()`
   - Add logging for all database operations

### Phase 2: Utility Functions

4. **Color Parser Utility**
   - Create `src/utils/color_parser.rs`
   - Parse hex codes (#FF0000, FF0000, 0xFF0000)
   - Parse color names (red, blue, etc.)
   - Return `Result<u32, Error>` 
   - Log invalid color attempts: `tracing::warn!("Invalid color: {}", input)`

5. **Role Manager Utility**
   - Create `src/utils/role_manager.rs`
   - `create_booster_role()` function
   - `find_booster_role_position()` - calculate appropriate hierarchy position
   - `cleanup_old_roles()` for role management
   - Log all role operations with guild/user context

### Phase 3: Command Implementation

6. **Command Module Creation**
   - Create `src/commands/boosterrole.rs`
   - Implement Poise command with attributes:
     ```rust
     #[poise::command(slash_command, guild_only)]
     ```
   - Define parameters with descriptions
   - Log command invocation: `tracing::info!("Boosterrole command invoked by {} in guild {}", user_id, guild_id)`

7. **Permission Checking**
   - Check `member.premium_since.is_some()`
   - Return error embed for non-boosters
   - Log permission denials: `tracing::warn!("Non-booster {} attempted boosterrole command", user_id)`

8. **Role Creation Logic**
   - Check for existing role in database
   - Create or update Discord role
   - Handle role hierarchy positioning
   - Assign role to user
   - Update database record
   - Log each step with appropriate level

### Phase 4: Response Handling

9. **Success Response**
   - Use `EmbedBuilder::success()`
   - Show role preview with color
   - Include role mention
   - Add footer with timestamp
   - Log successful creation

10. **Error Handling**
    - Permission denied (non-booster)
    - Invalid color format
    - Role limit reached
    - Discord API errors
    - Database errors
    - Use `EmbedBuilder::error()` for all error responses
    - Log errors with context: `tracing::error!("Failed to create role: {:?}", error)`

### Phase 5: Integration

11. **Register Command**
    - Add to `src/commands/mod.rs` exports
    - Add to command vec in `src/bot/framework.rs:11-17`
    - Verify command appears in Discord

12. **Testing Checklist**
    - Non-booster rejection
    - Valid hex color parsing
    - Invalid color handling
    - Role creation and assignment
    - Database persistence
    - Update existing role
    - Guild-specific isolation

### Phase 6: Enhancement Features

13. **Boost Expiry Handling** (Optional)
    - Event handler for `GuildMemberUpdate`
    - Check if boost removed
    - Clean up role if configured
    - Log boost status changes

14. **Role Management Commands** (Future)
    - `/boosterrole delete` - Remove custom role
    - `/boosterrole list` - Show all booster roles
    - Admin override options

## Logging Specifications

### Command Invocation
```rust
tracing::info!(
    user_id = %ctx.author().id,
    guild_id = %ctx.guild_id().unwrap(),
    role_name = %name,
    color = %color,
    "Boosterrole command invoked"
);
```

### Success Events
```rust
tracing::info!(
    user_id = %user_id,
    guild_id = %guild_id,
    role_id = %role.id,
    role_name = %role.name,
    "Booster role created successfully"
);
```

### Error Events
```rust
tracing::error!(
    user_id = %user_id,
    guild_id = %guild_id,
    error = ?e,
    "Failed to create booster role"
);
```

### Database Operations
```rust
tracing::debug!(
    "Database query: get_booster_role for user {} in guild {}",
    user_id, guild_id
);
```

## Error Recovery Strategies

1. **Partial Failure Recovery**
   - If role created but DB fails: Delete role and show error
   - If DB updated but assignment fails: Retry assignment
   - Log all recovery attempts

2. **Rate Limit Handling**
   - Implement exponential backoff
   - Queue role creations if needed
   - Inform user of delays

3. **Cleanup Procedures**
   - Periodic task to remove orphaned roles
   - Verify database consistency
   - Log cleanup operations

## Performance Considerations

1. **Caching Strategy**
   - Cache booster status checks (5 min TTL)
   - Cache role lookups per guild
   - Use Arc<RwLock<HashMap>> pattern from prefix system

2. **Database Optimization**
   - Index on (guild_id, user_id)
   - Prepared statements for common queries
   - Connection pooling (already configured)

## Security Considerations

1. **Input Validation**
   - Sanitize role names (max length, forbidden characters)
   - Validate color inputs strictly
   - Prevent role name injection

2. **Permission Boundaries**
   - Never create roles above bot's highest role
   - Respect Discord's role limits
   - Validate bot has necessary permissions

3. **Rate Limiting**
   - Per-user cooldown (60 seconds)
   - Per-guild limits (X roles per hour)
   - Log rate limit violations

## Success Metrics

- Command executes < 2 seconds
- 0% error rate for valid inputs
- Proper cleanup of expired roles
- All operations logged appropriately
- Database consistency maintained

## Notes

- This roadmap follows the Poise framework patterns established in the codebase
- All async operations use Tokio runtime
- Error handling follows the custom Error enum pattern
- Embed responses maintain consistency with existing commands
- Logging uses tracing crate throughout