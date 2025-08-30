# Command Development Guidelines

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

// Embed response using standardized colors
use crate::utils::EmbedColor;

ctx.send(poise::CreateReply::default()
    .embed(serenity::all::CreateEmbed::default()
        .title("Title")
        .description("Description")
        .color(EmbedColor::Primary.value())
    )
).await?;

// Using ResponseHelper for standardized responses
use crate::utils::ResponseHelper;

ResponseHelper::send_success(ctx, "Success!", "Operation completed").await?;
ResponseHelper::send_error(ctx, "Error", "Something went wrong").await?;
ResponseHelper::send_info(ctx, "Info", "Here's some information").await?;

// Deferred response for long operations
ctx.defer().await?;
// ... long operation ...
ctx.say("Done!").await?;
```

### Standardized Color System

**Always use the `EmbedColor` enum for consistent visual branding:**

```rust
use crate::utils::EmbedColor;

// Color usage guidelines:
EmbedColor::Primary     // #C6AC80 - General responses, info displays
EmbedColor::Success     // #62CB77 - Success confirmations (settings saved, etc.)
EmbedColor::Error       // #853535 - Error messages, validation failures  
EmbedColor::Warning     // #FFE209 - Warnings, help responses
EmbedColor::Info        // #FFE209 - Information messages (same as warning)
EmbedColor::Secondary   // #95A5A6 - Less important information
```

**Preferred approach using ResponseHelper:**

```rust
use crate::utils::ResponseHelper;

// For success responses (configuration saved, action completed)
ResponseHelper::send_success(ctx, "✅ Prefix Updated", 
    "Your server prefix has been changed successfully!").await?;

// For error responses (invalid input, permission issues)
ResponseHelper::send_error(ctx, "❌ Invalid Input", 
    "Please provide a valid color code or color name.").await?;

// For informational responses (help, current settings)
ResponseHelper::send_info(ctx, "ℹ️ Current Settings", 
    "Here are your current bot configurations.").await?;

// For warning responses (deprecated features, limits)
ResponseHelper::send_warning(ctx, "⚠️ Warning", 
    "This feature will be deprecated in the next version.").await?;
```

**Manual embed creation (when ResponseHelper isn't sufficient):**

```rust
use crate::utils::EmbedColor;

let embed = serenity::CreateEmbed::new()
    .title("📊 Server Statistics")
    .description("Current server information")
    .color(EmbedColor::Primary.value())  // Use enum, not hardcoded hex
    .field("Members", "150", true)
    .field("Channels", "25", true);

ctx.send(poise::CreateReply::default().embed(embed)).await?;
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

### Command Aliases Configuration

**Always provide meaningful aliases for prefix commands to improve user experience:**

```rust
// Main command with comprehensive aliases
#[poise::command(
    slash_command,
    prefix_command,
    aliases("h", "commands", "cmd"),  // Short, descriptive, common alternatives
    category = "Utility"
)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    // Implementation
    Ok(())
}
```

#### Alias Best Practices:

1. **Single-letter shortcuts**: Always include a 1-letter alias for frequently used commands
   - `help` → `h`
   - `ping` → `p` 
   - `info` → `i`

2. **Descriptive alternatives**: Include common alternative names users might expect
   - `help` → `commands`, `cmd`
   - `info` → `information`, `stats`
   - `ping` → `pong`, `latency`

3. **Abbreviated forms**: Include reasonable abbreviations
   - `prefix` → `pre`, `pfx`, `pref`
   - `boosterrole` → `br`, `booster`
   - `cache_status` → `cache`, `status`

#### Examples from Commands:

```rust
// Core utility commands - multiple intuitive aliases
#[poise::command(slash_command, prefix_command, aliases("h", "commands", "cmd"))]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> { /* ... */ }

#[poise::command(slash_command, prefix_command, aliases("i", "information", "stats"))]  
pub async fn info(ctx: Context<'_>) -> Result<(), Error> { /* ... */ }

#[poise::command(slash_command, prefix_command, aliases("p", "pong", "latency"))]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> { /* ... */ }

// Feature-specific commands
#[poise::command(slash_command, prefix_command, aliases("br", "booster", "customrole", "role"))]
pub async fn boosterrole(ctx: Context<'_>) -> Result<(), Error> { /* ... */ }

#[poise::command(slash_command, prefix_command, aliases("pre", "pfx", "pref"))]
pub async fn prefix(ctx: Context<'_>) -> Result<(), Error> { /* ... */ }
```

#### Subcommand Aliases:

```rust
// Parent command aliases
#[poise::command(slash_command, prefix_command, aliases("pre", "pfx", "pref"), subcommands("view", "set", "remove", "reset"))]
pub async fn prefix(ctx: Context<'_>) -> Result<(), Error> { /* ... */ }

// Subcommand aliases for common operations
#[poise::command(slash_command, prefix_command, aliases("v", "show", "current"))]
pub async fn view(ctx: Context<'_>) -> Result<(), Error> { /* ... */ }

#[poise::command(slash_command, prefix_command, aliases("s", "change", "update"))]
pub async fn set(ctx: Context<'_>) -> Result<(), Error> { /* ... */ }

#[poise::command(slash_command, prefix_command, aliases("r", "rm", "delete"))]
pub async fn remove(ctx: Context<'_>) -> Result<(), Error> { /* ... */ }

#[poise::command(slash_command, prefix_command, aliases("default", "clear"))]
pub async fn reset(ctx: Context<'_>) -> Result<(), Error> { /* ... */ }
```

#### Alias Guidelines:

- **DO**: Use common abbreviations (`info` → `i`, `help` → `h`)
- **DO**: Include action words for subcommands (`set` → `change`, `update`)
- **DO**: Think about what users would naturally type
- **DON'T**: Create conflicting single-letter aliases between commands
- **DON'T**: Use overly obscure abbreviations
- **DON'T**: Forget to include aliases for frequently used commands

#### Testing Aliases:

Always test that your aliases work correctly:
```bash
# Test all variants work the same way
!help
!h
!commands
!cmd
```

## 