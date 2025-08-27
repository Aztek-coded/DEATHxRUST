use crate::bot::{Context, Error};

/// Show available commands and their usage
#[poise::command(slash_command, prefix_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "\n💡 **Tip:** These commands work as both slash commands (/) and prefix commands (!)",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}