use crate::bot::{Context, Error};
use crate::data::models::{BoosterRole, BoosterRoleShare, GuildSharingLimit};
use crate::utils::{EmbedBuilder, ResponseHelper};
use serenity::all::{Role, RoleId, User, UserId};
use tracing::{info, instrument, warn};

/// Share your booster role with other members
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Booster Roles",
    subcommands("share_role", "share_remove", "share_max", "share_list", "share_limit"),
    description_localized("en-US", "Manage booster role sharing")
)]
pub async fn share(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Share your booster role with another member
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    rename = "role",
    category = "Booster Roles",
    required_bot_permissions = "MANAGE_ROLES",
    description_localized("en-US", "Share your booster role with another member")
)]
#[instrument(
    skip(ctx),
    fields(
        user_id = %ctx.author().id,
        guild_id = ?ctx.guild_id(),
        command = "boosterrole.share.role"
    )
)]
async fn share_role(
    ctx: Context<'_>,
    #[description = "Member to share your role with"] 
    user: User,
) -> Result<(), Error> {
    info!(target_user = %user.id, "Share role command invoked");
    
    let guild_id = ctx.guild_id().ok_or(Error::Command("This command must be used in a guild".to_string()))?;
    let owner_id = ctx.author().id;
    let data = ctx.data();
    
    // Prevent self-sharing
    if user.id == owner_id {
        ResponseHelper::send_error(
            ctx,
            "Invalid Target",
            "You cannot share your role with yourself."
        ).await?;
        return Ok(());
    }
    
    // Check if owner has a booster role
    let booster_role = BoosterRole::get(&data.db_pool, guild_id, owner_id).await?
        .ok_or(Error::Command("You don't have a booster role to share.".to_string()))?;
    
    let role_id = RoleId::new(booster_role.role_id as u64);
    
    // Check sharing limits
    let limits = GuildSharingLimit::get(&data.db_pool, guild_id).await?
        .unwrap_or(GuildSharingLimit {
            id: 0,
            guild_id: guild_id.get() as i64,
            max_members_per_role: 5,
            max_shared_roles_per_member: 3,
            set_by: 0,
            created_at: None,
            updated_at: None,
        });
    
    // Check role share count
    let current_shares = BoosterRoleShare::count_role_shares(&data.db_pool, guild_id, role_id).await?;
    if current_shares >= limits.max_members_per_role as i64 {
        ResponseHelper::send_error(
            ctx,
            "Share Limit Reached",
            &format!(
                "This role has reached the maximum share limit of {} members.",
                limits.max_members_per_role
            )
        ).await?;
        return Ok(());
    }
    
    // Check target user's shared role count
    let user_shares = BoosterRoleShare::count_user_shares(&data.db_pool, guild_id, user.id).await?;
    if user_shares >= limits.max_shared_roles_per_member as i64 {
        ResponseHelper::send_error(
            ctx,
            "User Share Limit Reached",
            &format!(
                "{} has reached the maximum limit of {} shared roles.",
                user.name,
                limits.max_shared_roles_per_member
            )
        ).await?;
        return Ok(());
    }
    
    // Check if already shared
    let existing_shares = BoosterRoleShare::get_role_shares(&data.db_pool, guild_id, role_id).await?;
    if existing_shares.iter().any(|s| s.shared_with_id == user.id.get() as i64 && s.is_active) {
        ResponseHelper::send_error(
            ctx,
            "Already Shared",
            &format!("Your role is already shared with {}.", user.name)
        ).await?;
        return Ok(());
    }
    
    // Add role to target user
    let member = guild_id.member(&ctx.http(), user.id).await?;
    member.add_role(&ctx.http(), role_id).await?;
    
    // Create share record
    BoosterRoleShare::create(&data.db_pool, guild_id, role_id, owner_id, user.id).await?;
    
    info!(
        owner_id = %owner_id,
        shared_with = %user.id,
        role_id = %role_id,
        guild_id = %guild_id,
        "Role shared successfully"
    );
    
    ResponseHelper::send_success(
        ctx,
        "✅ Role Shared",
        &format!(
            "Your booster role **{}** has been shared with <@{}>.",
            booster_role.role_name,
            user.id
        )
    ).await?;
    Ok(())
}

/// Remove yourself from a shared booster role
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    rename = "remove",
    category = "Booster Roles",
    required_bot_permissions = "MANAGE_ROLES",
    description_localized("en-US", "Remove yourself from a shared booster role")
)]
#[instrument(
    skip(ctx),
    fields(
        user_id = %ctx.author().id,
        guild_id = ?ctx.guild_id(),
        command = "boosterrole.share.remove"
    )
)]
async fn share_remove(
    ctx: Context<'_>,
    #[description = "The shared role to remove yourself from"] 
    role: Role,
) -> Result<(), Error> {
    info!(role_id = %role.id, "Remove share command invoked");
    
    let guild_id = ctx.guild_id().ok_or(Error::Command("This command must be used in a guild".to_string()))?;
    let user_id = ctx.author().id;
    let data = ctx.data();
    
    // Check if user has this shared role
    let shares = BoosterRoleShare::get_shared_with_user(&data.db_pool, guild_id, user_id).await?;
    
    let _share = shares.iter()
        .find(|s| s.role_id == role.id.get() as i64 && s.is_active)
        .ok_or(Error::Command("You don't have access to this shared role.".to_string()))?;
    
    // Remove role from user
    let member = guild_id.member(&ctx.http(), user_id).await?;
    if let Err(e) = member.remove_role(&ctx.http(), role.id).await {
        warn!(
            "Failed to remove role {} from user {}: {}",
            role.id, user_id, e
        );
    }
    
    // Remove share record
    BoosterRoleShare::remove(&data.db_pool, guild_id, role.id, user_id).await?;
    
    info!(
        user_id = %user_id,
        role_id = %role.id,
        guild_id = %guild_id,
        "Share removed successfully"
    );
    
    ResponseHelper::send_success(
        ctx,
        "✅ Share Removed",
        &format!("You have been removed from the shared role **{}**.", role.name)
    ).await?;
    Ok(())
}

/// Set maximum members per shared role (Admin only)
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    rename = "max",
    category = "Booster Roles",
    required_permissions = "MANAGE_GUILD",
    description_localized("en-US", "Set maximum members per shared role")
)]
#[instrument(
    skip(ctx),
    fields(
        user_id = %ctx.author().id,
        guild_id = ?ctx.guild_id(),
        command = "boosterrole.share.max"
    )
)]
async fn share_max(
    ctx: Context<'_>,
    #[description = "Maximum members per shared role (1-25)"] 
    #[min = 1]
    #[max = 25]
    max_members: i32,
) -> Result<(), Error> {
    info!(max_members = max_members, "Set share max command invoked");
    
    let guild_id = ctx.guild_id().ok_or(Error::Command("This command must be used in a guild".to_string()))?;
    let user_id = ctx.author().id;
    let data = ctx.data();
    
    // Get current limits or defaults
    let current_limits = GuildSharingLimit::get(&data.db_pool, guild_id).await?
        .unwrap_or(GuildSharingLimit {
            id: 0,
            guild_id: guild_id.get() as i64,
            max_members_per_role: 5,
            max_shared_roles_per_member: 3,
            set_by: 0,
            created_at: None,
            updated_at: None,
        });
    
    // Update limits
    GuildSharingLimit::set(
        &data.db_pool,
        guild_id,
        max_members,
        current_limits.max_shared_roles_per_member,
        user_id
    ).await?;
    
    info!(
        guild_id = %guild_id,
        max_members = max_members,
        set_by = %user_id,
        "Share max limit set"
    );
    
    ResponseHelper::send_success(
        ctx,
        "✅ Limit Updated",
        &format!("Maximum members per shared role set to **{}**.", max_members)
    ).await?;
    Ok(())
}

/// View all members in booster roles (Admin only)
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    rename = "list",
    category = "Booster Roles",
    required_permissions = "MANAGE_GUILD",
    description_localized("en-US", "View all members in booster roles")
)]
#[instrument(
    skip(ctx),
    fields(
        user_id = %ctx.author().id,
        guild_id = ?ctx.guild_id(),
        command = "boosterrole.share.list"
    )
)]
async fn share_list(ctx: Context<'_>) -> Result<(), Error> {
    info!("Share list command invoked");
    
    let guild_id = ctx.guild_id().ok_or(Error::Command("This command must be used in a guild".to_string()))?;
    let data = ctx.data();
    
    // Get all booster roles
    let booster_roles = sqlx::query_as::<_, BoosterRole>(
        "SELECT * FROM booster_roles WHERE guild_id = ?"
    )
    .bind(guild_id.get() as i64)
    .fetch_all(&data.db_pool)
    .await?;
    
    if booster_roles.is_empty() {
        ResponseHelper::send_info(
            ctx,
            "No Booster Roles",
            "There are no booster roles in this server."
        ).await?;
        return Ok(());
    }
    
    let mut description = String::from("👥 **Booster Role Shares**\n\n");
    
    for role in booster_roles.iter().take(10) {
        let role_id = RoleId::new(role.role_id as u64);
        let owner_id = UserId::new(role.user_id as u64);
        
        // Get shares for this role
        let shares = BoosterRoleShare::get_role_shares(&data.db_pool, guild_id, role_id).await?;
        
        description.push_str(&format!("**{}**\n", role.role_name));
        description.push_str(&format!("Owner: <@{}>\n", owner_id));
        
        if shares.is_empty() {
            description.push_str("No shares\n");
        } else {
            description.push_str("Shared with: ");
            for share in shares.iter().take(5) {
                description.push_str(&format!("<@{}> ", share.shared_with_id));
            }
            if shares.len() > 5 {
                description.push_str(&format!("... and {} more", shares.len() - 5));
            }
            description.push_str("\n");
        }
        description.push_str("\n");
    }
    
    if booster_roles.len() > 10 {
        description.push_str(&format!("\n_Showing 10 of {} booster roles_", booster_roles.len()));
    }
    
    let embed = EmbedBuilder::info("👥 Booster Role Shares", &description);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    
    Ok(())
}

/// Set maximum shared roles per member (Admin only)
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    rename = "limit",
    category = "Booster Roles",
    required_permissions = "MANAGE_GUILD",
    description_localized("en-US", "Set maximum shared roles per member")
)]
#[instrument(
    skip(ctx),
    fields(
        user_id = %ctx.author().id,
        guild_id = ?ctx.guild_id(),
        command = "boosterrole.share.limit"
    )
)]
async fn share_limit(
    ctx: Context<'_>,
    #[description = "Maximum shared roles per member (1-10)"] 
    #[min = 1]
    #[max = 10]
    max_roles: i32,
) -> Result<(), Error> {
    info!(max_roles = max_roles, "Set share limit command invoked");
    
    let guild_id = ctx.guild_id().ok_or(Error::Command("This command must be used in a guild".to_string()))?;
    let user_id = ctx.author().id;
    let data = ctx.data();
    
    // Get current limits or defaults
    let current_limits = GuildSharingLimit::get(&data.db_pool, guild_id).await?
        .unwrap_or(GuildSharingLimit {
            id: 0,
            guild_id: guild_id.get() as i64,
            max_members_per_role: 5,
            max_shared_roles_per_member: 3,
            set_by: 0,
            created_at: None,
            updated_at: None,
        });
    
    // Update limits
    GuildSharingLimit::set(
        &data.db_pool,
        guild_id,
        current_limits.max_members_per_role,
        max_roles,
        user_id
    ).await?;
    
    info!(
        guild_id = %guild_id,
        max_roles = max_roles,
        set_by = %user_id,
        "Share role limit set"
    );
    
    ResponseHelper::send_success(
        ctx,
        "✅ Limit Updated",
        &format!("Maximum shared roles per member set to **{}**.", max_roles)
    ).await?;
    Ok(())
}

