use crate::bot::{Context, Error};
use crate::data::models::{BoosterRole, BoosterRoleLink, BoosterRoleShare};
use crate::utils::ResponseHelper;
use serenity::all::{RoleId, UserId};
use tracing::{error, info, instrument, warn};

/// Remove your custom booster role
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Booster Roles",
    required_bot_permissions = "MANAGE_ROLES",
    description_localized("en-US", "Remove your custom booster role"),
    aliases("rm", "delete", "del")
)]
#[instrument(
    skip(ctx),
    fields(
        user_id = %ctx.author().id,
        guild_id = ?ctx.guild_id(),
        command = "boosterrole.remove"
    )
)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Also remove all shares of this role"] 
    remove_shares: Option<bool>,
) -> Result<(), Error> {
    info!("Remove booster role command invoked");
    
    let guild_id = ctx.guild_id().ok_or(Error::Command("This command must be used in a guild".to_string()))?;
    let user_id = ctx.author().id;
    
    // Check if user has a booster role
    let data = ctx.data();
    let existing_role = BoosterRole::get(&data.db_pool, guild_id, user_id).await?;
    
    if existing_role.is_none() {
        ResponseHelper::send_error(
            ctx,
            "No Booster Role",
            "You don't have a custom booster role to remove."
        ).await?;
        return Ok(());
    }
    
    let role_data = existing_role.unwrap();
    let role_id = RoleId::new(role_data.role_id as u64);
    let role_name = role_data.role_name.clone();
    
    // Check if role is linked (admin-managed)
    if let Some(_linked_role) = BoosterRoleLink::get(&data.db_pool, guild_id, user_id).await? {
        ResponseHelper::send_error(
            ctx,
            "Role is Linked",
            "Your booster role is managed by an administrator and cannot be removed."
        ).await?;
        return Ok(());
    }
    
    // Handle role shares if they exist
    let shares = BoosterRoleShare::get_role_shares(&data.db_pool, guild_id, role_id).await?;
    let share_count = shares.len();
    
    if share_count > 0 && !remove_shares.unwrap_or(false) {
        ResponseHelper::send_error(
            ctx,
            "Role Has Active Shares",
            &format!(
                "This role is currently shared with {} member(s). \
                Use `/boosterrole remove remove_shares:true` to remove the role and all shares.",
                share_count
            )
        ).await?;
        return Ok(());
    }
    
    // Remove shares if requested
    if remove_shares.unwrap_or(false) && share_count > 0 {
        for share in shares {
            let shared_user_id = UserId::new(share.shared_with_id as u64);
            
            // Remove role from shared user
            if let Ok(member) = guild_id.member(&ctx.http(), shared_user_id).await {
                if let Err(e) = member.remove_role(&ctx.http(), role_id).await {
                    warn!(
                        "Failed to remove role {} from user {}: {}",
                        role_id, shared_user_id, e
                    );
                }
            }
            
            // Remove share from database
            BoosterRoleShare::remove(&data.db_pool, guild_id, role_id, shared_user_id).await?;
        }
        
        info!(
            guild_id = %guild_id,
            role_id = %role_id,
            shares_removed = share_count,
            "Removed all role shares"
        );
    }
    
    // Remove the role from Discord
    match guild_id.delete_role(&ctx.http(), role_id).await {
        Ok(_) => {
            info!(
                user_id = %user_id,
                guild_id = %guild_id,
                role_id = %role_id,
                "Role deleted from Discord"
            );
        }
        Err(e) => {
            error!(
                user_id = %user_id,
                guild_id = %guild_id,
                role_id = %role_id,
                error = ?e,
                "Failed to delete role from Discord"
            );
            // Continue with database cleanup even if Discord deletion fails
        }
    }
    
    // Remove from database
    BoosterRole::delete(&data.db_pool, guild_id, user_id).await?;
    
    info!(
        user_id = %user_id,
        guild_id = %guild_id,
        role_name = %role_name,
        shares_removed = share_count,
        "Booster role removed successfully"
    );
    
    // Send success response
    let mut description = format!("Your booster role **{}** has been successfully removed.", role_name);
    if share_count > 0 && remove_shares.unwrap_or(false) {
        description.push_str(&format!("\n\n{} role share(s) were also removed.", share_count));
    }
    
    ResponseHelper::send_success(
        ctx,
        "✅ Role Removed",
        &description
    ).await?;
    Ok(())
}