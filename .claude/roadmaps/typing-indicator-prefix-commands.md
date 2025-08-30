# Typing Indicator for Prefix Commands - Implementation Roadmap

## Feature Summary
Implement framework-level typing indicator functionality that automatically shows Discord's typing status when prefix commands are being executed. This feature will provide immediate visual feedback to users that their command has been received and is being processed.

## Discord Interaction Flow Analysis

### Event Sequence
1. **User sends prefix command** → Discord API receives message
2. **Poise framework detects prefix** → Command parsing begins
3. **Pre-command hook triggered** → Start typing indicator via `ctx.channel_id().start_typing()`
4. **Command execution begins** → Typing indicator remains active
5. **Command processing** → Bot executes command logic
6. **Command completes/fails** → Typing stops automatically when message is sent or after 10 seconds
7. **Response sent** → Typing indicator disappears

### Discord API Behavior
- Typing indicators automatically disappear after:
  - 10 seconds of inactivity
  - When the bot sends a message to the channel
  - When explicitly stopped
- Multiple typing requests extend the duration
- No explicit "stop typing" call needed if sending a message

## Module Analysis

### Primary Modification Points

#### 1. `/src/bot/framework.rs` (Main implementation location)
- **Current State**: Contains framework options, command registration, error handling
- **Required Changes**: 
  - Add `pre_command` hook to start typing indicator
  - Optionally add `post_command` hook for explicit cleanup
  - Ensure hooks only apply to prefix commands, not slash commands

#### 2. `/src/bot/mod.rs` (Type definitions)
- **Current State**: Defines type aliases for Context, Error, Framework
- **Required Changes**: None expected, but review for any needed helper types

#### 3. `/src/utils/response.rs` (Response handling)
- **Current State**: ResponseHelper manages all bot responses as embeds
- **Required Changes**: None - typing automatically stops when message is sent

## Hypothesized Implementation Approaches

### Approach 1: Pre-Command Hook (Recommended)
```rust
// In framework.rs - FrameworkOptions
pre_command: |ctx| {
    Box::pin(async move {
        // Only trigger for prefix commands
        if ctx.prefix_command {
            // Start typing indicator
            let _ = ctx.channel_id().start_typing(&ctx.http());
        }
    })
},
```

**Pros:**
- Clean, framework-level implementation
- Automatic for all prefix commands
- No modification to individual commands
- Typing stops automatically when response is sent

**Cons:**
- No fine-grained control per command
- Cannot disable for specific commands without additional logic

### Approach 2: Typing Handle Management
```rust
// Store typing handle for explicit management
pre_command: |ctx| {
    Box::pin(async move {
        if ctx.prefix_command {
            let typing = ctx.channel_id().start_typing(&ctx.http());
            // Store handle if needed for explicit stop
            ctx.data().typing_handles.insert(ctx.id(), typing);
        }
    })
},
```

**Pros:**
- Explicit control over typing duration
- Can stop typing early if needed

**Cons:**
- More complex state management
- Requires cleanup logic
- Unnecessary given Discord's auto-stop behavior

### Approach 3: Middleware Pattern
Create a middleware wrapper that handles typing for all commands.

**Pros:**
- Highly flexible
- Can add additional pre/post processing

**Cons:**
- Over-engineered for this use case
- Adds complexity without clear benefit

## Step-by-Step Implementation Roadmap

### Phase 1: Setup and Research
1. **Create feature branch**
   ```bash
   git checkout -b feature/typing-indicator-prefix
   ```

2. **Add tracing for debugging**
   - Location: `/src/bot/framework.rs`
   - Add: `tracing::debug!("Starting typing indicator for prefix command: {}", ctx.command().name);`
   - Add: `tracing::debug!("Command execution completed: {}", ctx.command().name);`

### Phase 2: Core Implementation

3. **Implement pre_command hook**
   - File: `/src/bot/framework.rs`
   - Modify: `poise::FrameworkOptions` struct
   - Add `pre_command` field with typing indicator logic
   - Ensure check for `ctx.prefix()` to only apply to prefix commands

4. **Test with existing commands**
   - Verify typing appears for prefix commands like `!ping`, `!help`
   - Confirm typing does NOT appear for slash commands
   - Test with both quick and slow commands

5. **Handle edge cases**
   - Test with commands that defer responses
   - Test with commands that send multiple messages
   - Test with commands that fail/error

### Phase 3: Enhancement and Optimization

6. **Add configuration option (optional)**
   - File: `/src/config/mod.rs`
   - Add: `typing_indicator_enabled: bool` field to Settings
   - Default: `true`
   - Environment variable: `TYPING_INDICATOR_ENABLED`

7. **Implement selective disable (optional)**
   - Add command attribute for disabling typing on specific commands
   - Example: Commands that respond instantly might not need typing

### Phase 4: Testing and Verification

8. **Manual testing checklist**
   - [ ] Typing appears for all prefix commands
   - [ ] Typing does NOT appear for slash commands  
   - [ ] Typing stops when response is sent
   - [ ] Typing stops after 10 seconds if no response
   - [ ] Multiple rapid commands handle typing correctly
   - [ ] Error responses still stop typing

9. **Add logging for monitoring**
   ```rust
   tracing::trace!(
       command = %ctx.command().qualified_name,
       user_id = %ctx.author().id,
       channel_id = %ctx.channel_id(),
       "Typing indicator started for prefix command"
   );
   ```

### Phase 5: Documentation and Deployment

10. **Update documentation**
    - Add note to CLAUDE.md about typing indicator behavior
    - Document any new configuration options

11. **Create PR**
    ```bash
    git add -A
    git commit -m "feat: Add automatic typing indicator for prefix commands
    
    - Implements framework-level typing indicator
    - Shows visual feedback when processing prefix commands
    - Typing automatically stops when response is sent
    - Only applies to prefix commands, not slash commands"
    
    git push origin feature/typing-indicator-prefix
    ```

## Key Implementation Details

### Critical Code Location
**File**: `/src/bot/framework.rs`
**Function**: `create_framework()`
**Section**: `poise::FrameworkOptions` initialization (lines 12-137)

### Specific Implementation
```rust
// Add to FrameworkOptions (around line 50, after prefix_options)
pre_command: |ctx| {
    Box::pin(async move {
        // Check if this is a prefix command (not slash command)
        if ctx.prefix().is_some() {
            // Start typing indicator - returns a Typing handle
            // The typing will automatically stop when we send a message
            let _typing = ctx.channel_id().start_typing(&ctx.http());
            
            tracing::trace!(
                command = %ctx.command().qualified_name,
                user_id = %ctx.author().id,
                channel_id = %ctx.channel_id(),
                "Typing indicator started for prefix command"
            );
        }
    })
},
```

### Integration Points
- **Serenity API**: `ChannelId::start_typing()` method
- **Poise Context**: Access via `ctx.channel_id()` and `ctx.http()`
- **Command Detection**: Use `ctx.prefix()` to identify prefix commands

## Risk Assessment

### Low Risk
- Non-breaking change (additive only)
- Typing indicators are cosmetic
- Auto-cleanup by Discord API
- No database changes required

### Potential Issues
- Rate limiting: Typing requests count against rate limits
- Performance: Minimal overhead, single HTTP request per command
- User experience: Some users might find constant typing annoying

## Success Metrics
- Users report improved feedback when using prefix commands
- No increase in command latency
- No errors related to typing indicator in logs
- Works consistently across all prefix commands

## Alternative Considerations
If pre_command hook doesn't work as expected:
1. Investigate `event_handler` for Message events
2. Consider command wrapper pattern
3. Explore Serenity's lower-level hooks

## Conclusion
This implementation provides a clean, framework-level solution for typing indicators that requires minimal code changes and provides maximum benefit. The approach leverages Discord's automatic typing cleanup and Poise's hook system for an elegant solution.