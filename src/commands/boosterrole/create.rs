use crate::bot::{Context, Error};
use crate::data::models::{BoosterRole, GuildBoosterLimit, RoleNameBlacklist};
use crate::utils::{ColorParser, ResponseHelper, RoleManager};
use poise::serenity_prelude as serenity;
use serenity::prelude::Mentionable;
use tracing::{error, info, instrument, warn};

/// Create a new custom booster role
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Booster Roles",
    required_bot_permissions = "MANAGE_ROLES",
    description_localized("en-US", "Create a new custom booster role with a name and color"),
    aliases("new", "make", "add"),
    broadcast_typing
)]
#[instrument(
    skip(ctx),
    fields(
        user_id = %ctx.author().id,
        guild_id = ?ctx.guild_id(),
        command = "boosterrole.create"
    )
)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "The name for your custom role"] name: String,
    #[description = "The color of your role (hex code like #FF0000 or color name like 'red')"]
    color: String,
    #[description = "Optional second color for future gradient features"] second_color: Option<
        String,
    >,
) -> Result<(), Error> {
    info!("Create booster role command invoked");

    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| Error::Command("This command can only be used in guilds".to_string()))?;
    let user_id = ctx.author().id;
    let data = ctx.data();

    ctx.defer().await?;

    // Check if user is a booster
    let member = guild_id
        .member(&ctx.serenity_context().http, user_id)
        .await
        .map_err(|e| Error::Command(format!("Failed to get member information: {}", e)))?;

    if !RoleManager::is_booster(&member) {
        warn!(
            guild_id = %guild_id,
            "Non-booster attempted boosterrole create command"
        );

        ResponseHelper::send_error(
            ctx,
            "❌ Server Booster Required",
            "This command is only available to server boosters! Please boost this server to use this feature.",
        )
        .await?;
        return Ok(());
    }

    // Check if user already has a booster role
    let existing_role = BoosterRole::get(&data.db_pool, guild_id, user_id)
        .await
        .map_err(|e| Error::Database(e))?;

    if existing_role.is_some() {
        ResponseHelper::send_error(
            ctx,
            "❌ Role Already Exists",
            "You already have a custom booster role! Use `/boosterrole color` to update it, or `/boosterrole remove` to delete it first.",
        )
        .await?;
        return Ok(());
    }

    // Check if role name contains blacklisted words
    match RoleNameBlacklist::is_blacklisted(&data.db_pool, guild_id, &name).await {
        Ok(true) => {
            warn!(
                guild_id = %guild_id,
                role_name = %name,
                "Attempted to use blacklisted word in role name"
            );

            ResponseHelper::send_error(
                ctx,
                "❌ Inappropriate Role Name",
                "The role name contains words that are not allowed. Please choose a different name.",
            )
            .await?;
            return Ok(());
        }
        Err(e) => {
            error!(
                error = ?e,
                guild_id = %guild_id,
                "Failed to check role name blacklist"
            );
        }
        _ => {}
    }

    // Validate role name
    if let Err(e) = RoleManager::validate_role_name(&name) {
        ResponseHelper::send_error(ctx, "❌ Invalid Role Name", &format!("{}", e)).await?;
        return Ok(());
    }

    // Parse primary color
    let primary_color = match ColorParser::parse(&color) {
        Ok(c) => c,
        Err(e) => {
            ResponseHelper::send_error(
                ctx,
                "❌ Invalid Color",
                &format!(
                    "{}\n\nSupported formats:\n• Hex codes: `#FF0000`, `FF0000`, `0xFF0000`\n• Color names: `red`, `blue`, `green`, etc.\n• Short hex: `#F00` (expands to `#FF0000`)",
                    e
                ),
            )
            .await?;
            return Ok(());
        }
    };

    // Parse secondary color if provided
    let secondary_color_parsed = if let Some(ref second_color) = second_color {
        match ColorParser::parse(second_color) {
            Ok(c) => Some(c),
            Err(e) => {
                ResponseHelper::send_error(ctx, "❌ Invalid Second Color", &format!("{}", e))
                    .await?;
                return Ok(());
            }
        }
    } else {
        None
    };

    // Check role limit before creating
    let (can_create, limit) = GuildBoosterLimit::check_limit(&data.db_pool, guild_id)
        .await
        .map_err(|e| Error::Database(e))?;

    if !can_create {
        let limit_text = match limit {
            Some(0) => "Role creation is currently disabled".to_string(),
            Some(l) => format!(
                "This server has reached the maximum limit of {} booster roles",
                l
            ),
            None => "Role creation limit exceeded".to_string(),
        };

        ResponseHelper::send_error(
            ctx,
            "❌ Role Limit Reached",
            &format!("{}\n\nPlease contact an administrator.", limit_text),
        )
        .await?;
        return Ok(());
    }

    // Create the role
    info!(
        guild_id = %guild_id,
        role_name = %name,
        color = %color,
        "Creating new booster role"
    );

    let role = match RoleManager::create_booster_role(
        ctx.serenity_context(),
        guild_id,
        user_id,
        &name,
        primary_color,
        &data.db_pool,
    )
    .await
    {
        Ok(role) => role,
        Err(e) => {
            error!(
                guild_id = %guild_id,
                error = ?e,
                "Failed to create booster role"
            );

            ResponseHelper::send_error(
                ctx,
                "❌ Role Creation Failed",
                &format!("Failed to create your custom role: {}", e),
            )
            .await?;
            return Ok(());
        }
    };

    // Assign role to user
    if let Err(e) =
        RoleManager::assign_role_to_member(ctx.serenity_context(), guild_id, user_id, role.id).await
    {
        error!(
            role_id = %role.id,
            guild_id = %guild_id,
            error = ?e,
            "Failed to assign booster role to member"
        );

        // Clean up the role if assignment fails
        if let Err(cleanup_error) =
            RoleManager::delete_role(ctx.serenity_context(), guild_id, role.id).await
        {
            error!(
                role_id = %role.id,
                guild_id = %guild_id,
                error = ?cleanup_error,
                "Failed to cleanup role after assignment failure"
            );
        }

        ResponseHelper::send_error(
            ctx,
            "❌ Role Assignment Failed",
            "Your role was created but couldn't be assigned to you. Please contact an administrator.",
        )
        .await?;
        return Ok(());
    }

    // Save to database
    let secondary_color_str = secondary_color_parsed
        .as_ref()
        .map(|c| ColorParser::to_hex_string(*c));

    if let Err(e) = BoosterRole::create(
        &data.db_pool,
        guild_id,
        user_id,
        role.id,
        &name,
        &ColorParser::to_hex_string(primary_color),
        secondary_color_str.as_deref(),
    )
    .await
    {
        error!(
            role_id = %role.id,
            guild_id = %guild_id,
            error = ?e,
            "Failed to save booster role to database"
        );

        warn!(
            role_id = %role.id,
            guild_id = %guild_id,
            "Role created successfully but database update failed - user can retry later"
        );
    }

    // Success response
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

    info!(
        role_id = %role.id,
        guild_id = %guild_id,
        role_name = %role.name,
        "Booster role created successfully"
    );

    Ok(())
}
