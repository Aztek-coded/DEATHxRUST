use crate::bot::{Context, Error};
use crate::data::models::{BoosterRole, RoleNameBlacklist};
use crate::utils::{ColorParser, EmbedBuilder, RoleManager};
use poise::serenity_prelude as serenity;
use serenity::prelude::Mentionable;

/// Create or update your custom booster role with personalized name and color
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    description_localized(
        "en-US",
        "Create or update your custom booster role with personalized name and color"
    ),
    aliases("colour")
)]
pub async fn color(
    ctx: Context<'_>,
    #[description = "The color of your role (hex code like #FF0000 or color name like 'red')"]
    color: String,
    #[description = "The name of your custom role"] name: String,
    #[description = "Optional second color for future gradient features"] second_color: Option<
        String,
    >,
) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| Error::Command("This command can only be used in guilds".to_string()))?;

    let user_id = ctx.author().id;

    tracing::info!(
        user_id = %user_id,
        guild_id = %guild_id,
        command = "boosterrole.color",
        role_name = %name,
        color = %color,
        "Booster role color command invoked"
    );

    // Defer response to give us more time to process
    ctx.defer().await?;

    // Get member object to check booster status
    let member = guild_id
        .member(&ctx.serenity_context().http, user_id)
        .await
        .map_err(|e| Error::Command(format!("Failed to get member information: {}", e)))?;

    // Check if user is a booster
    if !RoleManager::is_booster(&member) {
        tracing::warn!(
            user_id = %user_id,
            guild_id = %guild_id,
            "Non-booster attempted boosterrole color command"
        );

        let embed = EmbedBuilder::error(
            "❌ Server Booster Required",
            "This command is only available to server boosters! Please boost this server to use this feature."
        );

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    // Check if role name contains blacklisted words
    if let Err(e) = RoleNameBlacklist::is_blacklisted(&ctx.data().db_pool, guild_id, &name).await {
        tracing::error!(
            error = ?e,
            guild_id = %guild_id,
            "Failed to check role name blacklist"
        );
    } else if RoleNameBlacklist::is_blacklisted(&ctx.data().db_pool, guild_id, &name)
        .await
        .unwrap_or(false)
    {
        tracing::warn!(
            user_id = %user_id,
            guild_id = %guild_id,
            role_name = %name,
            "Attempted to use blacklisted word in role name"
        );

        let embed = EmbedBuilder::error(
            "❌ Inappropriate Role Name",
            "The role name contains words that are not allowed. Please choose a different name."
        );

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    // Validate role name
    if let Err(e) = RoleManager::validate_role_name(&name) {
        let embed = EmbedBuilder::error("❌ Invalid Role Name", &format!("{}", e));

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    // Parse primary color
    let primary_color = match ColorParser::parse(&color) {
        Ok(c) => c,
        Err(e) => {
            let embed = EmbedBuilder::error(
                "❌ Invalid Color",
                &format!("{}\n\nSupported formats:\n• Hex codes: `#FF0000`, `FF0000`, `0xFF0000`\n• Color names: `red`, `blue`, `green`, etc.\n• Short hex: `#F00` (expands to `#FF0000`)", e)
            );

            ctx.send(poise::CreateReply::default().embed(embed)).await?;
            return Ok(());
        }
    };

    // Parse secondary color if provided
    let secondary_color_parsed = if let Some(ref second_color) = second_color {
        match ColorParser::parse(second_color) {
            Ok(c) => Some(c),
            Err(e) => {
                let embed = EmbedBuilder::error("❌ Invalid Second Color", &format!("{}", e));

                ctx.send(poise::CreateReply::default().embed(embed)).await?;
                return Ok(());
            }
        }
    } else {
        None
    };

    // Check if user already has a booster role
    let existing_role = BoosterRole::get(&ctx.data().db_pool, guild_id, user_id)
        .await
        .map_err(|e| Error::Database(e))?;

    let role = if let Some(existing) = existing_role {
        // Update existing role
        tracing::info!(
            user_id = %user_id,
            guild_id = %guild_id,
            existing_role_id = %existing.role_id,
            "Updating existing booster role"
        );

        let role_id = serenity::RoleId::new(existing.role_id as u64);

        match RoleManager::update_booster_role(
            ctx.serenity_context(),
            guild_id,
            role_id,
            &name,
            primary_color,
        )
        .await
        {
            Ok(role) => role,
            Err(e) => {
                tracing::error!(
                    user_id = %user_id,
                    guild_id = %guild_id,
                    role_id = %role_id,
                    error = ?e,
                    "Failed to update existing booster role"
                );

                let embed = EmbedBuilder::error(
                    "❌ Role Update Failed",
                    "Failed to update your existing role. It may have been deleted. Try running the command again to create a new one."
                );

                ctx.send(poise::CreateReply::default().embed(embed)).await?;
                return Ok(());
            }
        }
    } else {
        // Create new role
        tracing::info!(
            user_id = %user_id,
            guild_id = %guild_id,
            "Creating new booster role"
        );

        match RoleManager::create_booster_role(
            ctx.serenity_context(),
            guild_id,
            user_id,
            &name,
            primary_color,
        )
        .await
        {
            Ok(role) => role,
            Err(e) => {
                tracing::error!(
                    user_id = %user_id,
                    guild_id = %guild_id,
                    error = ?e,
                    "Failed to create booster role"
                );

                let embed = EmbedBuilder::error(
                    "❌ Role Creation Failed",
                    &format!("Failed to create your custom role: {}", e),
                );

                ctx.send(poise::CreateReply::default().embed(embed)).await?;
                return Ok(());
            }
        }
    };

    // Assign role to user
    if let Err(e) =
        RoleManager::assign_role_to_member(ctx.serenity_context(), guild_id, user_id, role.id).await
    {
        tracing::error!(
            user_id = %user_id,
            guild_id = %guild_id,
            role_id = %role.id,
            error = ?e,
            "Failed to assign booster role to member"
        );

        // If role assignment fails, we should clean up the role
        if let Err(cleanup_error) =
            RoleManager::delete_role(ctx.serenity_context(), guild_id, role.id).await
        {
            tracing::error!(
                role_id = %role.id,
                guild_id = %guild_id,
                error = ?cleanup_error,
                "Failed to cleanup role after assignment failure"
            );
        }

        let embed = EmbedBuilder::error(
            "❌ Role Assignment Failed",
            "Your role was created but couldn't be assigned to you. Please contact an administrator."
        );

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    // Update database
    let secondary_color_str = secondary_color_parsed
        .as_ref()
        .map(|c| ColorParser::to_hex_string(*c));
    if let Err(e) = BoosterRole::create(
        &ctx.data().db_pool,
        guild_id,
        user_id,
        role.id,
        &name,
        &ColorParser::to_hex_string(primary_color),
        secondary_color_str.as_deref(),
    )
    .await
    {
        tracing::error!(
            user_id = %user_id,
            guild_id = %guild_id,
            role_id = %role.id,
            error = ?e,
            "Failed to save booster role to database"
        );

        // Database failed but Discord role exists - this is recoverable
        // Just warn in logs, user can try again later
        tracing::warn!(
            user_id = %user_id,
            guild_id = %guild_id,
            role_id = %role.id,
            "Role created successfully but database update failed - user can retry later"
        );
    }

    // Create success response
    let mut embed = serenity::CreateEmbed::new()
        .title("✅ Booster Role Created!")
        .description(format!(
            "Your custom role **{}** has been created and assigned!\n\nRole: {}\nColor: `{}`",
            role.name,
            role.mention(),
            ColorParser::to_hex_string(primary_color)
        ))
        .color(primary_color)
        .thumbnail(ctx.author().avatar_url().unwrap_or_default())
        .footer(serenity::CreateEmbedFooter::new(format!(
            "Created by {}",
            ctx.author().name
        )))
        .timestamp(serenity::Timestamp::now());

    if let Some(second_color_hex) = secondary_color_str {
        embed = embed.field("Second Color", format!("`{}`", second_color_hex), true);
    }

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    tracing::info!(
        user_id = %user_id,
        guild_id = %guild_id,
        role_id = %role.id,
        role_name = %role.name,
        "Booster role created successfully"
    );

    Ok(())
}