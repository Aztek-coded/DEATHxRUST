use crate::bot::{Context, Error};
use crate::data::models::{BoosterRole, GuildBoosterBaseRole};
use crate::utils::ResponseHelper;
use serenity::all::{EditRole, Role, RoleId};
use tracing::{info, instrument, warn};

/// Set the base role for booster role hierarchy positioning
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Booster Roles",
    required_permissions = "MANAGE_GUILD",
    required_bot_permissions = "MANAGE_ROLES",
    description_localized("en-US", "Set the base role for booster role hierarchy positioning")
)]
#[instrument(
    skip(ctx),
    fields(
        user_id = %ctx.author().id,
        guild_id = ?ctx.guild_id(),
        command = "boosterrole.base"
    )
)]
pub async fn base(
    ctx: Context<'_>,
    #[description = "Role to position booster roles above"] 
    role: Option<Role>,
    #[description = "Remove the base role setting"] 
    remove: Option<bool>,
) -> Result<(), Error> {
    info!("Base role command invoked");
    
    let guild_id = ctx.guild_id().ok_or(Error::Command("This command must be used in a guild".to_string()))?;
    let user_id = ctx.author().id;
    let data = ctx.data();
    
    // Handle removal
    if remove.unwrap_or(false) {
        let removed = GuildBoosterBaseRole::remove(&data.db_pool, guild_id).await?;
        
        if removed {
            info!(
                guild_id = %guild_id,
                set_by = %user_id,
                "Base role setting removed"
            );
            
            ResponseHelper::send_success(
                ctx,
                "✅ Base Role Removed",
                "The base role setting has been removed. Booster roles will now use default positioning."
            ).await?;
            return Ok(());
        } else {
            ResponseHelper::send_error(
                ctx,
                "No Base Role Set",
                "There is no base role currently configured for this server."
            ).await?;
            return Ok(());
        }
    }
    
    // Handle viewing current setting
    if role.is_none() {
        let current_base = GuildBoosterBaseRole::get(&data.db_pool, guild_id).await?;
        
        if let Some(base_role_id) = current_base {
            let base_role = {
                let guild = guild_id.to_guild_cached(&ctx.serenity_context().cache)
                    .ok_or(Error::Command("Guild not found in cache".to_string()))?;
                guild.roles.get(&base_role_id).cloned()
            };
            
            if let Some(base_role) = base_role {
                let embed = crate::utils::EmbedBuilder::info(
                    "📍 Current Base Role",
                    &format!(
                        "Booster roles are positioned above: <@&{}>\n\
                        Role name: **{}**\n\
                        Position: **#{}**",
                        base_role.id,
                        base_role.name,
                        base_role.position
                    )
                );
                
                ctx.send(poise::CreateReply::default().embed(embed)).await?;
            } else {
                ResponseHelper::send_error(
                    ctx,
                    "Base Role Not Found",
                    "The configured base role no longer exists in this server."
                ).await?;
                return Ok(());
            }
        } else {
            ResponseHelper::send_info(
                ctx,
                "No Base Role Set",
                "No base role is currently configured. Booster roles use default positioning."
            ).await?;
            return Ok(());
        }
        
        return Ok(());
    }
    
    // Set new base role
    let new_base_role = role.unwrap();
    
    // Validate the role isn't too high in hierarchy
    let bot_member = guild_id.member(&ctx.http(), ctx.framework().bot_id).await?;
    
    let highest_bot_role_position = {
        let guild = guild_id.to_guild_cached(&ctx.serenity_context().cache)
            .ok_or(Error::Command("Guild not found in cache".to_string()))?;
        bot_member.roles.iter()
            .filter_map(|r| guild.roles.get(r))
            .map(|r| r.position)
            .max()
            .unwrap_or(0)
    };
    
    if new_base_role.position >= highest_bot_role_position {
        ResponseHelper::send_error(
            ctx,
            "Invalid Base Role",
            "The base role must be below the bot's highest role in the hierarchy."
        ).await?;
        return Ok(());
    }
    
    // Store the new base role
    GuildBoosterBaseRole::set(&data.db_pool, guild_id, new_base_role.id, user_id).await?;
    
    info!(
        guild_id = %guild_id,
        base_role_id = %new_base_role.id,
        base_role_name = %new_base_role.name,
        set_by = %user_id,
        "Base role set successfully"
    );
    
    // Reposition existing booster roles
    let booster_roles = sqlx::query_as::<_, BoosterRole>(
        "SELECT * FROM booster_roles WHERE guild_id = ?"
    )
    .bind(guild_id.get() as i64)
    .fetch_all(&data.db_pool)
    .await?;
    
    let mut repositioned_count = 0;
    for booster_role in booster_roles {
        let role_id = RoleId::new(booster_role.role_id as u64);
        
        // Check if role exists
        let role_exists = {
            let guild = guild_id.to_guild_cached(&ctx.serenity_context().cache)
                .ok_or(Error::Command("Guild not found in cache".to_string()))?;
            guild.roles.get(&role_id).is_some()
        };
        
        if role_exists {
            // Position it above the base role
            let new_position = new_base_role.position + 1;
            
            match guild_id.edit_role(
                &ctx.http(),
                role_id,
                EditRole::new().position(new_position as u16)
            ).await {
                Ok(_) => {
                    repositioned_count += 1;
                    info!(
                        role_id = %role_id,
                        new_position = new_position,
                        "Repositioned booster role"
                    );
                }
                Err(e) => {
                    warn!(
                        role_id = %role_id,
                        error = ?e,
                        "Failed to reposition booster role"
                    );
                }
            }
        }
    }
    
    // Send success response
    let mut description = format!(
        "Base role set to <@&{}>.\n\
        Booster roles will now be positioned above this role.",
        new_base_role.id
    );
    
    if repositioned_count > 0 {
        description.push_str(&format!(
            "\n\n✨ {} existing booster role(s) have been repositioned.",
            repositioned_count
        ));
    }
    
    ResponseHelper::send_success(
        ctx,
        "✅ Base Role Set",
        &description
    ).await?;
    Ok(())
}

