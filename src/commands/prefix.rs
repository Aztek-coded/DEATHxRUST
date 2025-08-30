use crate::bot::{Context, Error};
use crate::utils::EmbedColor;
use poise::serenity_prelude as serenity;

#[poise::command(
    slash_command,
    prefix_command,
    subcommands("view", "set", "remove", "reset"),
    guild_only,
    category = "Configuration",
    description_localized("en-US", "Manage the bot's command prefix for this server"),
    aliases("pre", "pfx", "pref"),
    broadcast_typing
)]
pub async fn prefix(ctx: Context<'_>) -> Result<(), Error> {
    // Default action shows the current prefix
    view_prefix(ctx).await
}

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    rename = "view",
    description_localized("en-US", "Display the current guild's prefix configuration"),
    aliases("v", "show", "current"),
    broadcast_typing
)]

pub async fn view(ctx: Context<'_>) -> Result<(), Error> {
    view_prefix(ctx).await
}

async fn view_prefix(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| Error::Command("This command can only be used in guilds".to_string()))?;

    let current_prefix = ctx
        .data()
        .get_guild_prefix(guild_id.get())
        .await?
        .unwrap_or_else(|| ctx.data().settings.command_prefix.clone());

    let default_prefix = &ctx.data().settings.command_prefix;

    let embed = serenity::CreateEmbed::new()
        .title("📋 Prefix Configuration")
        .description(format!(
            "**Current Guild Prefix:** `{}`\n**Default Prefix:** `{}`",
            current_prefix, default_prefix
        ))
        .color(EmbedColor::Primary.value())
        .footer(serenity::CreateEmbedFooter::new(format!(
            "Use {}prefix set <new_prefix> to change",
            current_prefix
        )));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    rename = "set",
    description_localized("en-US", "Set a custom prefix for this guild"),
    aliases("s", "change", "update"),
    broadcast_typing
)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "The new prefix to use (1-5 characters)"] new_prefix: String,
) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| Error::Command("This command can only be used in guilds".to_string()))?;

    if new_prefix.is_empty() || new_prefix.len() > 5 {
        return Err(Error::Command(
            "Prefix must be 1-5 characters long".to_string(),
        ));
    }

    if new_prefix.contains('@') || new_prefix.contains('#') {
        return Err(Error::Command(
            "Prefix cannot contain @ or # characters".to_string(),
        ));
    }

    let old_prefix = ctx
        .data()
        .get_guild_prefix(guild_id.get())
        .await?
        .unwrap_or_else(|| ctx.data().settings.command_prefix.clone());

    ctx.data()
        .set_guild_prefix(guild_id.get(), &new_prefix)
        .await?;

    tracing::info!(
        guild_id = %guild_id,
        user_id = %ctx.author().id,
        old_prefix = %old_prefix,
        new_prefix = %new_prefix,
        "Guild prefix updated"
    );

    let embed = serenity::CreateEmbed::new()
        .title("✅ Prefix Updated")
        .description(format!(
            "**Old Prefix:** `{}`\n**New Prefix:** `{}`\n\nYou can now use `{}help` to test the new prefix!",
            old_prefix,
            new_prefix,
            new_prefix
        ))
        .color(EmbedColor::Success.value())
        .footer(serenity::CreateEmbedFooter::new(
            format!("Changed by {}", ctx.author().name)
        ));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    rename = "remove",
    description_localized("en-US", "Remove the custom prefix and revert to default"),
    aliases("r", "rm", "delete"),
    broadcast_typing
)]
pub async fn remove(ctx: Context<'_>) -> Result<(), Error> {
    reset_prefix(ctx).await
}

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    rename = "reset",
    description_localized("en-US", "Reset the guild prefix to default"),
    aliases("default", "clear"),
    broadcast_typing
)]
pub async fn reset(ctx: Context<'_>) -> Result<(), Error> {
    reset_prefix(ctx).await
}

async fn reset_prefix(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| Error::Command("This command can only be used in guilds".to_string()))?;

    let removed = ctx.data().remove_guild_prefix(guild_id.get()).await?;

    if !removed {
        let default_prefix = &ctx.data().settings.command_prefix;

        let embed = serenity::CreateEmbed::new()
            .title("ℹ️ No Custom Prefix")
            .description(format!(
                "This guild is already using the default prefix: `{}`",
                default_prefix
            ))
            .color(EmbedColor::Info.value());

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    } else {
        let default_prefix = &ctx.data().settings.command_prefix;

        tracing::info!(
            guild_id = %guild_id,
            user_id = %ctx.author().id,
            default_prefix = %default_prefix,
            "Guild prefix reset to default"
        );

        let embed = serenity::CreateEmbed::new()
            .title("🔄 Prefix Reset")
            .description(format!(
                "Reverted to default prefix: `{}`\n\nYou can now use `{}help` to test!",
                default_prefix, default_prefix
            ))
            .color(EmbedColor::Success.value())
            .footer(serenity::CreateEmbedFooter::new(format!(
                "Reset by {}",
                ctx.author().name
            )));

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}
