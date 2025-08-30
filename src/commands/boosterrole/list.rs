use crate::bot::{Context, Error};
use crate::data::models::BoosterRole;
use crate::utils::{EmbedBuilder, EmbedColor};
use poise::serenity_prelude as serenity;
use serenity::all::UserId;
use serenity::prelude::Mentionable;

/// View all booster roles in the server (Administrator only)
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    description_localized(
        "en-US",
        "View all custom booster roles created in this server"
    ),
    aliases("ls")
)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| Error::Command("This command can only be used in guilds".to_string()))?;

    let admin_id = ctx.author().id;

    tracing::info!(
        admin_id = %admin_id,
        guild_id = %guild_id,
        command = "boosterrole.list",
        "Booster role list command invoked"
    );

    // Defer response to give us more time to process
    ctx.defer().await?;

    // Get all booster roles for the guild
    let booster_roles = match BoosterRole::get_all_for_guild(&ctx.data().db_pool, guild_id).await {
        Ok(roles) => roles,
        Err(e) => {
            tracing::error!(
                admin_id = %admin_id,
                guild_id = %guild_id,
                error = ?e,
                "Failed to fetch booster roles"
            );

            let embed = EmbedBuilder::error(
                "❌ Database Error",
                "Failed to fetch booster roles. Please try again."
            );

            ctx.send(poise::CreateReply::default().embed(embed)).await?;
            return Ok(());
        }
    };

    if booster_roles.is_empty() {
        let embed = EmbedBuilder::primary(
            "📝 Booster Roles",
            "No booster roles have been created in this server yet.\n\nBoosters can use `/boosterrole color <color> <name>` to create their custom roles."
        );

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    // Create paginated response for large lists
    const ROLES_PER_PAGE: usize = 10;
    let total_pages = (booster_roles.len() + ROLES_PER_PAGE - 1) / ROLES_PER_PAGE;
    let current_page = 1; // For now, just show the first page

    let start_idx = (current_page - 1) * ROLES_PER_PAGE;
    let end_idx = std::cmp::min(start_idx + ROLES_PER_PAGE, booster_roles.len());
    let page_roles = &booster_roles[start_idx..end_idx];

    let mut role_descriptions = Vec::new();
    
    for (i, role) in page_roles.iter().enumerate() {
        let user_mention = UserId::new(role.user_id as u64).mention();
        let role_mention = format!("<@&{}>", role.role_id);
        
        let created_at = role.created_at
            .as_ref()
            .map(|dt| format!("<t:{}:R>", chrono::DateTime::parse_from_str(dt, "%Y-%m-%d %H:%M:%S").map(|dt| dt.timestamp()).unwrap_or(0)))
            .unwrap_or_else(|| "Unknown".to_string());

        let description = format!(
            "**{}. {}** by {}\n└ Color: `{}` • Created: {}",
            start_idx + i + 1,
            role_mention,
            user_mention,
            role.primary_color,
            created_at
        );

        role_descriptions.push(description);
    }

    let role_list = role_descriptions.join("\n\n");

    let embed = serenity::CreateEmbed::new()
        .title("🎨 Server Booster Roles")
        .description(format!(
            "**All booster roles ({} total):**\n\n{}",
            booster_roles.len(),
            role_list
        ))
        .color(EmbedColor::Primary.value())
        .footer(serenity::CreateEmbedFooter::new(format!(
            "Page {} of {} • Requested by {}",
            current_page,
            total_pages,
            ctx.author().name
        )))
        .timestamp(serenity::Timestamp::now());

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    tracing::info!(
        admin_id = %admin_id,
        guild_id = %guild_id,
        role_count = booster_roles.len(),
        "Booster role list displayed successfully"
    );

    Ok(())
}