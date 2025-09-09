use crate::bot::{Context, Error};
use crate::data::models::{GuildStaffRole, SettingsAuditLog};
use crate::utils::{EmbedColor, ResponseHelper};
use serenity::all::{CreateEmbed, CreateEmbedFooter, Role};

#[poise::command(slash_command, prefix_command, subcommands("add", "remove", "list"))]
pub async fn staff(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Role to designate as staff"] role: Role,
) -> Result<(), Error> {
    super::validate_permissions(&ctx).await?;
    
    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;

    GuildStaffRole::add(pool, guild_id, role.id, ctx.author().id).await?;

    SettingsAuditLog::log(
        pool,
        guild_id,
        ctx.author().id,
        "staff_role_added",
        Some(&format!("Role: {} ({})", role.name, role.id)),
    )
    .await?;

    ResponseHelper::send_success(
        ctx,
        "✅ Staff Role Added",
        &format!("**{}** has been designated as a staff role", role.name),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Role to remove from staff"] role: Role,
) -> Result<(), Error> {
    super::validate_permissions(&ctx).await?;
    
    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;

    let removed = GuildStaffRole::remove(pool, guild_id, role.id).await?;

    if removed {
        SettingsAuditLog::log(
            pool,
            guild_id,
            ctx.author().id,
            "staff_role_removed",
            Some(&format!("Role: {} ({})", role.name, role.id)),
        )
        .await?;

        ResponseHelper::send_success(
            ctx,
            "✅ Staff Role Removed",
            &format!("**{}** is no longer a staff role", role.name),
        )
        .await?;
    } else {
        ResponseHelper::send_error(
            ctx,
            "❌ Role Not Found",
            &format!("**{}** was not designated as a staff role", role.name),
        )
        .await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;

    let staff_roles = GuildStaffRole::list(pool, guild_id).await?;

    if staff_roles.is_empty() {
        ResponseHelper::send_info(ctx, "📋 Staff Roles", "No staff roles have been configured")
            .await?;
    } else {
        let role_list = staff_roles
            .iter()
            .map(|sr| format!("• <@&{}>", sr.role_id))
            .collect::<Vec<_>>()
            .join("\n");

        let embed = CreateEmbed::new()
            .title("📋 Staff Roles")
            .description(&role_list)
            .color(EmbedColor::Primary.value())
            .footer(CreateEmbedFooter::new(format!(
                "{} staff roles configured",
                staff_roles.len()
            )));

        ctx.send(poise::CreateReply::default().embed(embed))
            .await?;
    }
    Ok(())
}