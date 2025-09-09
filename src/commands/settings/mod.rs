use crate::bot::{Context, Error};
use crate::utils::{ResponseHelper, SettingsError};
use serenity::all::Permissions;

pub type SettingsContext<'a> = Context<'a>;

pub mod autonick;
pub mod config;
pub mod joinlogs;
pub mod premiumrole;
pub mod staff;

#[poise::command(
    slash_command,
    prefix_command,
    category = "Administration",
    aliases("config", "cfg", "set"),
    required_permissions = "MANAGE_GUILD",
    guild_only,
    subcommands(
        "config::config",
        "staff::staff",
        "autonick::autonick",
        "joinlogs::joinlogs",
        "premiumrole::premiumrole"
    ),
    broadcast_typing
)]
pub async fn settings(ctx: Context<'_>) -> Result<(), Error> {
    ResponseHelper::send_info(
        ctx,
        "⚙️ Guild Settings",
        "Use subcommands to configure your server:\n\
        • `/settings config` - View all settings\n\
        • `/settings staff` - Manage staff roles\n\
        • `/settings autonick` - Auto-nickname setup\n\
        • `/settings joinlogs` - Join/leave logging\n\
        • `/settings premiumrole` - Premium role setup",
    )
    .await?;
    Ok(())
}

pub async fn validate_permissions(ctx: &Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let member = ctx.author_member().await.ok_or("Cannot fetch member")?;

    // Get the guild to check permissions properly
    let guild = guild_id.to_partial_guild(&ctx.serenity_context().http).await?;
    
    // Check member permissions - using everyone permissions as base and adding role permissions
    let member_roles = &member.roles;
    let everyone_role_id = guild.id.everyone_role();
    let mut member_permissions = guild.roles.get(&everyone_role_id)
        .map(|r| r.permissions)
        .unwrap_or(Permissions::empty());
    for role_id in member_roles {
        if let Some(role) = guild.roles.get(role_id) {
            member_permissions |= role.permissions;
        }
    }
    
    if !member_permissions.contains(Permissions::MANAGE_GUILD) {
        return Err(SettingsError::InsufficientPermissions.into());
    }

    let bot_member = guild_id
        .member(&ctx.serenity_context().http, ctx.framework().bot_id)
        .await?;

    // Check bot permissions
    let bot_roles = &bot_member.roles;
    let mut bot_permissions = guild.roles.get(&everyone_role_id)
        .map(|r| r.permissions)
        .unwrap_or(Permissions::empty());
    for role_id in bot_roles {
        if let Some(role) = guild.roles.get(role_id) {
            bot_permissions |= role.permissions;
        }
    }
    
    if !bot_permissions.contains(Permissions::MANAGE_ROLES) {
        ResponseHelper::send_error(
            *ctx,
            "❌ Bot Permissions",
            "I need **Manage Roles** permission to configure settings",
        )
        .await?;
        return Err("Missing bot permissions".into());
    }

    Ok(())
}