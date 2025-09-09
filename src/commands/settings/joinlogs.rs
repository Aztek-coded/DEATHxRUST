use crate::bot::{Context, Error};
use crate::data::models::{GuildJoinLogChannel, SettingsAuditLog};
use crate::utils::{EmbedColor, ResponseHelper, SettingsError};
use serenity::all::{Channel, ChannelId, CreateEmbed, CreateMessage, Permissions};
use serenity::model::mention::Mentionable;

#[poise::command(slash_command, prefix_command, subcommands("set", "disable", "test"))]
pub async fn joinlogs(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "Channel for join/leave logs"] channel: Channel,
) -> Result<(), Error> {
    super::validate_permissions(&ctx).await?;

    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;

    // Extract channel ID and validate it's a text channel
    let channel_id = match channel {
        Channel::Guild(ref gc) => {
            if !gc.is_text_based() {
                return Err(SettingsError::ChannelPermissionDenied(
                    "Channel must be a text channel".to_string(),
                )
                .into());
            }
            gc.id
        }
        _ => {
            return Err(SettingsError::ChannelPermissionDenied(
                "Invalid channel type".to_string(),
            )
            .into())
        }
    };

    // Check bot permissions in the channel
    let bot_id = ctx.framework().bot_id;

    if let Channel::Guild(gc) = &channel {
        // Get the bot member and guild to check permissions
        let bot_member = guild_id.member(&ctx.serenity_context().http, bot_id).await?;
        let guild = guild_id.to_partial_guild(&ctx.serenity_context().http).await?;
        let perms = guild.user_permissions_in(gc, &bot_member);
        if !perms.contains(Permissions::SEND_MESSAGES | Permissions::EMBED_LINKS) {
            return Err(SettingsError::ChannelPermissionDenied(
                "I need Send Messages and Embed Links permissions in that channel".to_string(),
            )
            .into());
        }
    }

    GuildJoinLogChannel::set(pool, guild_id, channel_id, ctx.author().id).await?;

    SettingsAuditLog::log(
        pool,
        guild_id,
        ctx.author().id,
        "join_log_channel_set",
        Some(&format!("Channel: <#{}>", channel_id)),
    )
    .await?;

    // Send test message
    let test_embed = CreateEmbed::new()
        .title("✅ Join Logs Configured")
        .description("This channel will now receive member join/leave notifications")
        .color(EmbedColor::Success.value())
        .footer(serenity::all::CreateEmbedFooter::new(format!(
            "Configured by {}",
            ctx.author().tag()
        )));

    channel_id
        .send_message(
            &ctx.serenity_context().http,
            CreateMessage::new().embed(test_embed),
        )
        .await?;

    ResponseHelper::send_success(
        ctx,
        "✅ Join Logs Configured",
        &format!("Join/leave logs will be sent to <#{}>", channel_id),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn disable(ctx: Context<'_>) -> Result<(), Error> {
    super::validate_permissions(&ctx).await?;

    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;

    let removed = GuildJoinLogChannel::remove(pool, guild_id).await?;

    if removed {
        SettingsAuditLog::log(
            pool,
            guild_id,
            ctx.author().id,
            "join_log_channel_disabled",
            None,
        )
        .await?;

        ResponseHelper::send_success(
            ctx,
            "✅ Join Logs Disabled",
            "Join/leave logging has been disabled",
        )
        .await?;
    } else {
        ResponseHelper::send_info(ctx, "ℹ️ No Join Logs", "Join logs were not configured")
            .await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;

    let join_log = GuildJoinLogChannel::get(pool, guild_id).await?;

    if let Some(log_config) = join_log {
        let channel_id = ChannelId::new(log_config.channel_id as u64);

        let test_embed = CreateEmbed::new()
            .title("🧪 Test Join Log")
            .description("This is a test message for join/leave logs")
            .color(EmbedColor::Primary.value())
            .field("Member", ctx.author().mention().to_string(), true)
            .field("Type", "Test Event", true)
            .timestamp(serenity::model::Timestamp::now());

        channel_id
            .send_message(
                &ctx.serenity_context().http,
                CreateMessage::new().embed(test_embed),
            )
            .await?;

        ResponseHelper::send_success(
            ctx,
            "✅ Test Sent",
            &format!("Test message sent to <#{}>", channel_id),
        )
        .await?;
    } else {
        ResponseHelper::send_info(
            ctx,
            "ℹ️ No Join Logs",
            "Join logs are not configured. Use `/settings joinlogs set` to configure.",
        )
        .await?;
    }
    Ok(())
}