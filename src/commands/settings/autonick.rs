use crate::bot::{Context, Error};
use crate::data::models::{GuildAutoNickname, SettingsAuditLog};
use crate::utils::{ResponseHelper, SettingsError};

#[poise::command(slash_command, prefix_command, subcommands("set", "disable", "view"))]
pub async fn autonick(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "Nickname template (use {username} for username)"] template: String,
) -> Result<(), Error> {
    super::validate_permissions(&ctx).await?;

    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;

    // Validate nickname
    if template.len() > 32 {
        return Err(
            SettingsError::InvalidNickname("Nickname cannot exceed 32 characters".to_string())
                .into(),
        );
    }

    if template.contains("@everyone") || template.contains("@here") {
        return Err(SettingsError::InvalidNickname(
            "Nickname cannot contain @everyone or @here".to_string(),
        )
        .into());
    }

    GuildAutoNickname::set(pool, guild_id, &template, ctx.author().id).await?;

    SettingsAuditLog::log(
        pool,
        guild_id,
        ctx.author().id,
        "auto_nickname_set",
        Some(&format!("Template: {}", template)),
    )
    .await?;

    // Show preview
    let preview = template.replace("{username}", &ctx.author().name);

    ResponseHelper::send_success(
        ctx,
        "✅ Auto-Nickname Configured",
        &format!(
            "Template set to: `{}`\nPreview: **{}**",
            template, preview
        ),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn disable(ctx: Context<'_>) -> Result<(), Error> {
    super::validate_permissions(&ctx).await?;

    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;

    let removed = GuildAutoNickname::remove(pool, guild_id).await?;

    if removed {
        SettingsAuditLog::log(
            pool,
            guild_id,
            ctx.author().id,
            "auto_nickname_disabled",
            None,
        )
        .await?;

        ResponseHelper::send_success(ctx, "✅ Auto-Nickname Disabled", "Auto-nickname has been disabled for new members").await?;
    } else {
        ResponseHelper::send_info(
            ctx,
            "ℹ️ No Auto-Nickname",
            "Auto-nickname was not configured",
        )
        .await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn view(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;

    let auto_nick = GuildAutoNickname::get(pool, guild_id).await?;

    if let Some(config) = auto_nick {
        let preview = config
            .nickname_template
            .replace("{username}", &ctx.author().name);

        ResponseHelper::send_info(
            ctx,
            "📝 Auto-Nickname Template",
            &format!(
                "Current template: `{}`\nPreview: **{}**",
                config.nickname_template, preview
            ),
        )
        .await?;
    } else {
        ResponseHelper::send_info(
            ctx,
            "📝 Auto-Nickname",
            "Auto-nickname is not configured",
        )
        .await?;
    }
    Ok(())
}