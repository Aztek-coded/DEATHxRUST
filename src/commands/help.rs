use crate::bot::{Context, Error};
use crate::utils::{EmbedColor, ResponseHelper};
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter, Timestamp};

/// Show available commands and their usage
#[poise::command(
    slash_command,
    prefix_command,
    aliases("h", "commands", "cmd"),
    broadcast_typing
)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    match command {
        Some(cmd_name) => {
            // Show help for specific command
            let commands = &ctx.framework().options().commands;

            if let Some(cmd) = commands
                .iter()
                .find(|c| c.name == cmd_name || c.qualified_name == cmd_name)
            {
                let mut embed = CreateEmbed::new()
                    .title(format!("📚 Help: /{}", cmd.name))
                    .color(EmbedColor::Info.value())
                    .field(
                        "📝 Description",
                        cmd.description
                            .as_deref()
                            .unwrap_or("No description available"),
                        false,
                    );

                // Add usage examples
                let actual_prefix = if let Some(guild_id) = ctx.guild_id() {
                    ctx.data()
                        .get_guild_prefix(guild_id.get())
                        .await?
                        .unwrap_or_else(|| ctx.data().settings.command_prefix.clone())
                } else {
                    ctx.data().settings.command_prefix.clone()
                };
                embed = embed.field(
                    "💬 Usage",
                    format!(
                        "**Slash:** `/{}`\n**Prefix:** `{}{}`",
                        cmd.name, actual_prefix, cmd.name
                    ),
                    false,
                );

                // Add parameters if any
                let params: Vec<String> = cmd
                    .parameters
                    .iter()
                    .map(|p| {
                        let required = if p.required { "" } else { " (optional)" };
                        format!(
                            "• **{}**{}: {}",
                            p.name,
                            required,
                            p.description.as_deref().unwrap_or("No description")
                        )
                    })
                    .collect();

                if !params.is_empty() {
                    embed = embed.field("⚙️ Parameters", params.join("\n"), false);
                }

                // Add subcommands if any
                if !cmd.subcommands.is_empty() {
                    let subcmds: Vec<String> = cmd
                        .subcommands
                        .iter()
                        .map(|sc| {
                            format!(
                                "• **{}**: {}",
                                sc.name,
                                sc.description.as_deref().unwrap_or("No description")
                            )
                        })
                        .collect();

                    embed = embed.field("📂 Subcommands", subcmds.join("\n"), false);
                }

                // Add permissions if required
                if !cmd.required_permissions.is_empty() {
                    embed = embed.field(
                        "🔒 Required Permissions",
                        format!("{:?}", cmd.required_permissions),
                        false,
                    );
                }

                embed = embed
                    .footer(CreateEmbedFooter::new(format!(
                        "Requested by {}",
                        ctx.author().name
                    )))
                    .timestamp(Timestamp::now());

                ResponseHelper::send_embed(ctx, embed).await?;
            } else {
                ResponseHelper::send_error(
                    ctx,
                    "Command Not Found",
                    format!("No command named '{}' was found.", cmd_name),
                )
                .await?;
            }
        }
        None => {
            // Show general help with all commands
            let commands = &ctx.framework().options().commands;
            let _prefix = ctx.data().settings.command_prefix.clone();

            // Get actual prefix for this guild
            let actual_prefix = if let Some(guild_id) = ctx.guild_id() {
                ctx.data()
                    .get_guild_prefix(guild_id.get())
                    .await?
                    .unwrap_or_else(|| ctx.data().settings.command_prefix.clone())
            } else {
                ctx.data().settings.command_prefix.clone()
            };

            let mut embed = CreateEmbed::new()
                .title("📚 Bot Commands")
                .color(EmbedColor::Primary.value())
                .description(format!(
                    "Here are all available commands. Use `{}help <command>` or `/help <command>` for more details about a specific command.",
                    actual_prefix
                ));

            // Group commands by category or show all
            let mut command_list = Vec::new();

            for cmd in commands {
                let description = cmd.description.as_deref().unwrap_or("No description");
                let subcommands = if cmd.subcommands.is_empty() {
                    String::new()
                } else {
                    let sub_names: Vec<&str> =
                        cmd.subcommands.iter().map(|s| s.name.as_str()).collect();
                    format!(" `[{}]`", sub_names.join(", "))
                };

                command_list.push(format!("**{}{}** - {}", cmd.name, subcommands, description));
            }

            embed = embed.field("📋 Available Commands", command_list.join("\n"), false);

            embed = embed.field(
                "💡 Usage Tips",
                format!(
                    "• **Slash Commands:** Type `/` and select from the menu\n\
                     • **Prefix Commands:** Use `{}` before the command\n\
                     • **Additional Prefixes:** `death`, `d!`",
                    actual_prefix
                ),
                false,
            );

            embed = embed
                .footer(CreateEmbedFooter::new(format!(
                    "Total Commands: {} | Requested by {}",
                    commands.len(),
                    ctx.author().name
                )))
                .timestamp(Timestamp::now());

            ResponseHelper::send_embed(ctx, embed).await?;
        }
    }

    Ok(())
}
