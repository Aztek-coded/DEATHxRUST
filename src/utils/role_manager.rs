use crate::bot::Error;
use crate::utils::{BotError, ColorParser};
use serenity::all::{Colour, EditRole, Guild, GuildId, Member, Role, RoleId, UserId};
use serenity::prelude::Context as SerenityContext;

pub struct RoleManager;

impl RoleManager {
    /// Creates a booster role with the specified name and color
    pub async fn create_booster_role(
        ctx: &SerenityContext,
        guild_id: GuildId,
        user_id: UserId,
        role_name: &str,
        color: u32,
    ) -> Result<Role, Error> {
        tracing::info!(
            user_id = %user_id,
            guild_id = %guild_id,
            role_name = %role_name,
            color = %ColorParser::to_hex_string(color),
            "Creating booster role"
        );

        let guild = guild_id
            .to_guild_cached(&ctx.cache)
            .map(|g| g.clone())
            .ok_or_else(|| BotError::Other("Guild not found in cache".to_string()))?;

        // Validate color is within Discord's range
        if !ColorParser::is_valid_discord_color(color) {
            return Err(BotError::InvalidColor(format!(
                "Color {} is outside Discord's valid range",
                color
            ))
            .into());
        }

        // Find appropriate position for the role
        let position = Self::find_booster_role_position(&guild).await?;

        let role_builder = EditRole::default()
            .name(role_name)
            .colour(Colour::new(color))
            .hoist(false)
            .mentionable(false)
            .permissions(serenity::all::Permissions::empty());

        let role = guild_id.create_role(&ctx.http, role_builder).await?;

        // Move role to appropriate position
        if position > 0 {
            if let Err(e) = Self::move_role_to_position(ctx, guild_id, role.id, position).await {
                tracing::warn!(
                    role_id = %role.id,
                    position = position,
                    error = ?e,
                    "Failed to move role to desired position, keeping default position"
                );
            }
        }

        tracing::info!(
            user_id = %user_id,
            guild_id = %guild_id,
            role_id = %role.id,
            role_name = %role.name,
            "Booster role created successfully"
        );

        Ok(role)
    }

    /// Updates an existing role with new name and color
    pub async fn update_booster_role(
        ctx: &SerenityContext,
        guild_id: GuildId,
        role_id: RoleId,
        role_name: &str,
        color: u32,
    ) -> Result<Role, Error> {
        tracing::info!(
            guild_id = %guild_id,
            role_id = %role_id,
            role_name = %role_name,
            color = %ColorParser::to_hex_string(color),
            "Updating booster role"
        );

        // Validate color is within Discord's range
        if !ColorParser::is_valid_discord_color(color) {
            return Err(BotError::InvalidColor(format!(
                "Color {} is outside Discord's valid range",
                color
            ))
            .into());
        }

        let _role = guild_id
            .to_guild_cached(&ctx.cache)
            .and_then(|guild| guild.roles.get(&role_id).cloned())
            .ok_or_else(|| BotError::Other("Role not found".to_string()))?;

        let edit_builder = EditRole::default()
            .name(role_name)
            .colour(Colour::new(color));

        let role = guild_id.edit_role(&ctx.http, role_id, edit_builder).await?;

        tracing::info!(
            guild_id = %guild_id,
            role_id = %role_id,
            role_name = %role.name,
            "Booster role updated successfully"
        );

        Ok(role)
    }

    /// Assigns a role to a member
    pub async fn assign_role_to_member(
        ctx: &SerenityContext,
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
    ) -> Result<(), Error> {
        tracing::debug!(
            user_id = %user_id,
            guild_id = %guild_id,
            role_id = %role_id,
            "Assigning role to member"
        );

        let member = guild_id.member(&ctx.http, user_id).await?;

        // Check if member already has this role
        if member.roles.contains(&role_id) {
            tracing::debug!(
                user_id = %user_id,
                role_id = %role_id,
                "Member already has this role"
            );
            return Ok(());
        }

        member.add_role(&ctx.http, role_id).await?;

        tracing::info!(
            user_id = %user_id,
            guild_id = %guild_id,
            role_id = %role_id,
            "Role assigned to member successfully"
        );

        Ok(())
    }

    /// Removes a role from a member if they have it
    pub async fn remove_role_from_member(
        ctx: &SerenityContext,
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
    ) -> Result<(), Error> {
        tracing::debug!(
            user_id = %user_id,
            guild_id = %guild_id,
            role_id = %role_id,
            "Removing role from member"
        );

        let member = guild_id.member(&ctx.http, user_id).await?;

        // Check if member has this role
        if !member.roles.contains(&role_id) {
            tracing::debug!(
                user_id = %user_id,
                role_id = %role_id,
                "Member doesn't have this role"
            );
            return Ok(());
        }

        member.remove_role(&ctx.http, role_id).await?;

        tracing::info!(
            user_id = %user_id,
            guild_id = %guild_id,
            role_id = %role_id,
            "Role removed from member successfully"
        );

        Ok(())
    }

    /// Deletes a role from the guild
    pub async fn delete_role(
        ctx: &SerenityContext,
        guild_id: GuildId,
        role_id: RoleId,
    ) -> Result<(), Error> {
        tracing::info!(
            guild_id = %guild_id,
            role_id = %role_id,
            "Deleting role"
        );

        guild_id.delete_role(&ctx.http, role_id).await?;

        tracing::info!(
            guild_id = %guild_id,
            role_id = %role_id,
            "Role deleted successfully"
        );

        Ok(())
    }

    /// Checks if a member is a server booster
    pub fn is_booster(member: &Member) -> bool {
        member.premium_since.is_some()
    }

    /// Finds appropriate position for booster role in hierarchy
    /// Places it above regular members but below important roles
    async fn find_booster_role_position(guild: &Guild) -> Result<u16, BotError> {
        let bot_member = guild
            .members
            .values()
            .find(|m| m.user.bot)
            .ok_or_else(|| BotError::Other("Bot member not found in guild".to_string()))?;

        // Find the bot's highest role position
        let bot_highest_role = bot_member
            .roles
            .iter()
            .filter_map(|role_id| guild.roles.get(role_id))
            .max_by_key(|role| role.position)
            .ok_or_else(|| BotError::Other("Bot has no roles".to_string()))?;

        // Place booster roles a few positions below bot's highest role to avoid conflicts
        let target_position = bot_highest_role.position.saturating_sub(5);

        tracing::debug!(
            bot_highest_position = bot_highest_role.position,
            target_position = target_position,
            "Calculated booster role position"
        );

        Ok(target_position.max(1)) // Ensure position is at least 1
    }

    /// Moves a role to a specific position in the hierarchy
    async fn move_role_to_position(
        _ctx: &SerenityContext,
        guild_id: GuildId,
        role_id: RoleId,
        position: u16,
    ) -> Result<(), Error> {
        tracing::debug!(
            guild_id = %guild_id,
            role_id = %role_id,
            position = position,
            "Moving role to position"
        );

        // Note: Serenity doesn't have a direct method to set role position
        // This would need to be implemented using the REST API directly
        // For now, we'll just log this and accept the default position
        tracing::warn!(
            guild_id = %guild_id,
            role_id = %role_id,
            position = position,
            "Role position adjustment not implemented - using default position"
        );

        Ok(())
    }

    /// Validates role name to ensure it meets Discord requirements
    pub fn validate_role_name(name: &str) -> Result<(), BotError> {
        let name = name.trim();

        if name.is_empty() {
            return Err(BotError::Command("Role name cannot be empty".to_string()));
        }

        if name.len() > 100 {
            return Err(BotError::Command(
                "Role name cannot exceed 100 characters".to_string(),
            ));
        }

        // Check for forbidden characters or patterns
        if name.contains('@') || name.contains('#') || name.contains(':') {
            return Err(BotError::Command(
                "Role name contains forbidden characters (@, #, :)".to_string(),
            ));
        }

        if name.to_lowercase() == "everyone" || name.to_lowercase() == "here" {
            return Err(BotError::Command(
                "Role name cannot be 'everyone' or 'here'".to_string(),
            ));
        }

        Ok(())
    }

    /// Clean up orphaned roles (roles that exist in database but not in Discord)
    pub async fn cleanup_orphaned_roles(
        ctx: &SerenityContext,
        guild_id: GuildId,
        database_role_ids: Vec<RoleId>,
    ) -> Result<Vec<RoleId>, Error> {
        tracing::info!(
            guild_id = %guild_id,
            role_count = database_role_ids.len(),
            "Starting orphaned role cleanup"
        );

        let guild = guild_id
            .to_guild_cached(&ctx.cache)
            .map(|g| g.clone())
            .ok_or_else(|| BotError::Other("Guild not found in cache".to_string()))?;

        let mut orphaned_roles = Vec::new();

        for role_id in database_role_ids {
            if !guild.roles.contains_key(&role_id) {
                tracing::warn!(
                    guild_id = %guild_id,
                    role_id = %role_id,
                    "Found orphaned role (exists in database but not in Discord)"
                );
                orphaned_roles.push(role_id);
            }
        }

        tracing::info!(
            guild_id = %guild_id,
            orphaned_count = orphaned_roles.len(),
            "Orphaned role cleanup completed"
        );

        Ok(orphaned_roles)
    }
}
