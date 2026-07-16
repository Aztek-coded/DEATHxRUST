# Command Development

Apply when editing `src/commands/**/*.rs`.

## Structure

```rust
use crate::bot::{Context, Error};
use crate::utils::{ResponseHelper, EmbedColor};

/// Shown in Discord
#[poise::command(
    slash_command,
    prefix_command,
    aliases("short", "alt"),
    category = "Utility"
)]
pub async fn command_name(
    ctx: Context<'_>,
    #[description = "Param"] param: String,
) -> Result<(), Error> {
    ResponseHelper::send_success(ctx, "✅ Success", "Done.").await?;
    Ok(())
}
```

## Responses

- Prefer `ResponseHelper::{send_success,send_error,send_info,send_warning}`
- Ephemeral: `CreateReply::default().content(...).ephemeral(true)`
- Long work: `ctx.defer().await?` then reply
- Manual embeds: always `EmbedColor::*.value()`, never raw hex

## EmbedColor

- `Primary` `#C6AC80` — general/info displays
- `Success` `#62CB77` — confirmations
- `Error` `#853535` — failures
- `Warning` / `Info` `#FFE209`
- `Secondary` `#95A5A6`

## Aliases

- Include a 1-letter shortcut for frequent commands (`h`, `p`, `i`)
- Add common alternatives and short abbreviations
- Avoid conflicting single-letter aliases across commands
- Mirror aliases on common subcommands (`v`/`show`, `s`/`set`, `r`/`rm`)

## Subcommands

Parent is a no-op shell with `subcommands("add", "remove", ...)`; implement each child as its own `#[poise::command]`.

## Registration checklist

1. Implement under `src/commands/`
2. Export in `src/commands/mod.rs`
3. Add to `commands![]` in `src/bot/framework.rs`

Longer reference: `.grok/guidelines/command-development.md`
