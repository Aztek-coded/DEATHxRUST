use crate::bot::{Context, Error};
use crate::data::models::{GuildPremiumRole, SettingsAuditLog};
use crate::utils::{ResponseHelper, SettingsError};
use serenity::all::Role;

#[poise::command(slash_command, prefix_command, subcommands("set", "disable", "view"))]
pub async fn premiumrole(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "Role to designate as premium"] role: Role,
) -> Result<(), Error> {
    super::validate_permissions(&ctx).await?;

    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;

    // Check role hierarchy
    if role.managed {
        return Err(SettingsError::RoleHierarchyError(
            "Cannot use a managed role (bot/integration role) as premium role".to_string(),
        )
        .into());
    }

    if role.id == guild_id.everyone_role() {
        return Err(
            SettingsError::RoleHierarchyError("Cannot use @everyone as premium role".to_string())
                .into(),
        );
    }

    // Check bot can manage this role
    let bot_member = guild_id
        .member(&ctx.serenity_context().http, ctx.framework().bot_id)
        .await?;
    
    // Get bot's highest role position
    let guild = guild_id.to_partial_guild(&ctx.serenity_context().http).await?;
    let bot_highest_role = bot_member.roles
        .iter()
        .filter_map(|role_id| guild.roles.get(role_id))
        .map(|r| r.position)
        .max()
        .unwrap_or(0);

    if role.position >= bot_highest_role {
        return Err(SettingsError::RoleHierarchyError(
            "I cannot manage this role. Please move my role higher in the hierarchy.".to_string(),
        )
        .into());
    }

    GuildPremiumRole::set(pool, guild_id, role.id, ctx.author().id).await?;

    SettingsAuditLog::log(
        pool,
        guild_id,
        ctx.author().id,
        "premium_role_set",
        Some(&format!("Role: {} ({})", role.name, role.id)),
    )
    .await?;

    ResponseHelper::send_success(
        ctx,
        "✅ Premium Role Set",
        &format!(
            "**{}** has been designated as the premium role\nMembers with this role will receive special privileges",
            role.name
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

    let removed = GuildPremiumRole::remove(pool, guild_id).await?;

    if removed {
        SettingsAuditLog::log(
            pool,
            guild_id,
            ctx.author().id,
            "premium_role_disabled",
            None,
        )
        .await?;

        ResponseHelper::send_success(
            ctx,
            "✅ Premium Role Disabled",
            "Premium role has been disabled",
        )
        .await?;
    } else {
        ResponseHelper::send_info(ctx, "ℹ️ No Premium Role", "Premium role was not configured")
            .await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn view(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;

    let premium_role = GuildPremiumRole::get(pool, guild_id).await?;

    if let Some(config) = premium_role {
        ResponseHelper::send_info(
            ctx,
            "👑 Premium Role",
            &format!("Current premium role: <@&{}>", config.role_id),
        )
        .await?;
    } else {
        ResponseHelper::send_info(ctx, "👑 Premium Role", "No premium role is configured")
            .await?;
    }
    Ok(())
}