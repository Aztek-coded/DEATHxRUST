use crate::data::models::{BoosterRole, BoosterRoleLink};
use serenity::all::{Context, GuildId, GuildMemberUpdateEvent, Ready, Role};
use sqlx::SqlitePool;
use std::sync::Arc;

/// Event handler for boost-related events
pub struct BoostHandler {
    pub db_pool: Arc<SqlitePool>,
}

impl BoostHandler {
    pub fn new(db_pool: Arc<SqlitePool>) -> Self {
        Self { db_pool }
    }

    /// Handle boost status changes for a member
    pub async fn handle_boost_change(&self, ctx: &Context, event: &GuildMemberUpdateEvent) {
        let guild_id = event.guild_id;
        let user_id = event.user.id;

        // For now, we'll check the database and Discord API directly
        // since GuildMemberUpdateEvent doesn't contain old member info
        // This is a simplified approach - in production, you might want to cache member states
        
        // Get the member's current premium status
        let current_member = match guild_id.member(&ctx.http, user_id).await {
            Ok(member) => member,
            Err(_) => return, // Member not found or other error
        };

        // If member currently has premium status, no cleanup needed
        if current_member.premium_since.is_some() {
            return;
        }

        // Member doesn't have premium - check if they have a booster role to clean up

        tracing::info!(
            user_id = %user_id,
            guild_id = %guild_id,
            "Member lost boost status, cleaning up booster role"
        );

        // Get the booster role from database
        let booster_role = match BoosterRole::get(&self.db_pool, guild_id, user_id).await {
            Ok(Some(role)) => role,
            Ok(None) => {
                tracing::debug!(
                    user_id = %user_id,
                    guild_id = %guild_id,
                    "No booster role found in database for member who lost boost"
                );
                return;
            }
            Err(e) => {
                tracing::error!(
                    user_id = %user_id,
                    guild_id = %guild_id,
                    error = ?e,
                    "Failed to fetch booster role from database"
                );
                return;
            }
        };

        let role_id = serenity::all::RoleId::new(booster_role.role_id as u64);

        // Remove the role from Discord
        if let Err(e) = guild_id.member(&ctx.http, user_id).await {
            tracing::warn!(
                user_id = %user_id,
                guild_id = %guild_id,
                error = ?e,
                "Failed to get member for role removal"
            );
        } else if let Ok(member) = guild_id.member(&ctx.http, user_id).await {
            if member.roles.contains(&role_id) {
                if let Err(e) = member.remove_role(&ctx.http, role_id).await {
                    tracing::error!(
                        user_id = %user_id,
                        guild_id = %guild_id,
                        role_id = %role_id,
                        error = ?e,
                        "Failed to remove booster role from member"
                    );
                }
            }
        }

        // Try to delete the role from Discord if it still exists
        if let Err(e) = guild_id.delete_role(&ctx.http, role_id).await {
            // Role might already be deleted, so we'll log but not fail
            tracing::debug!(
                guild_id = %guild_id,
                role_id = %role_id,
                error = ?e,
                "Could not delete booster role from Discord (may already be deleted)"
            );
        } else {
            tracing::info!(
                guild_id = %guild_id,
                role_id = %role_id,
                "Successfully deleted booster role from Discord"
            );
        }

        // Clean up database entries
        if let Err(e) = BoosterRole::delete(&self.db_pool, guild_id, user_id).await {
            tracing::error!(
                user_id = %user_id,
                guild_id = %guild_id,
                error = ?e,
                "Failed to delete booster role from database"
            );
        } else {
            tracing::info!(
                user_id = %user_id,
                guild_id = %guild_id,
                "Successfully deleted booster role from database"
            );
        }

        // Clean up role link if it exists
        if let Err(e) = BoosterRoleLink::delete(&self.db_pool, guild_id, user_id).await {
            tracing::error!(
                user_id = %user_id,
                guild_id = %guild_id,
                error = ?e,
                "Failed to delete booster role link from database"
            );
        }

        tracing::info!(
            user_id = %user_id,
            guild_id = %guild_id,
            role_id = %role_id,
            "Boost expiration cleanup completed"
        );
    }

    /// Clean up orphaned roles (roles in database but not in Discord)
    pub async fn cleanup_orphaned_roles(&self, ctx: &Context, guild_id: GuildId) {
        tracing::debug!(
            guild_id = %guild_id,
            "Starting orphaned role cleanup"
        );

        // Get all booster roles from database
        let booster_roles = match BoosterRole::get_all_for_guild(&self.db_pool, guild_id).await {
            Ok(roles) => roles,
            Err(e) => {
                tracing::error!(
                    guild_id = %guild_id,
                    error = ?e,
                    "Failed to fetch booster roles for orphan cleanup"
                );
                return;
            }
        };

        if booster_roles.is_empty() {
            return;
        }

        // Get roles via HTTP API since cache access isn't Send-safe
        let guild_roles = match ctx.http.get_guild_roles(guild_id).await {
            Ok(roles) => roles,
            Err(e) => {
                tracing::error!(
                    guild_id = %guild_id,
                    error = ?e,
                    "Failed to fetch guild roles for orphan cleanup"
                );
                return;
            }
        };

        let role_ids: std::collections::HashSet<serenity::all::RoleId> = 
            guild_roles.iter().map(|role| role.id).collect();

        let mut orphaned_count = 0;

        for booster_role in booster_roles {
            let role_id = serenity::all::RoleId::new(booster_role.role_id as u64);
            let user_id = serenity::all::UserId::new(booster_role.user_id as u64);

            // Check if role still exists in Discord
            if !role_ids.contains(&role_id) {
                tracing::warn!(
                    guild_id = %guild_id,
                    user_id = %user_id,
                    role_id = %role_id,
                    "Found orphaned booster role, cleaning up database"
                );

                // Remove from database
                if let Err(e) = BoosterRole::delete(&self.db_pool, guild_id, user_id).await {
                    tracing::error!(
                        user_id = %user_id,
                        guild_id = %guild_id,
                        error = ?e,
                        "Failed to delete orphaned booster role from database"
                    );
                } else {
                    orphaned_count += 1;
                }

                // Clean up role link if it exists
                if let Err(e) = BoosterRoleLink::delete(&self.db_pool, guild_id, user_id).await {
                    tracing::error!(
                        user_id = %user_id,
                        guild_id = %guild_id,
                        error = ?e,
                        "Failed to delete orphaned role link from database"
                    );
                }
            }
        }

        if orphaned_count > 0 {
            tracing::info!(
                guild_id = %guild_id,
                orphaned_count = orphaned_count,
                "Cleaned up orphaned booster roles"
            );
        }
    }

    /// Handle ready event - start cleanup tasks
    pub async fn on_ready(&self, ctx: &Context, ready: &Ready) {
        tracing::info!(
            bot_user = %ready.user.name,
            "Boost handler ready, starting orphaned role cleanup"
        );

        // Clean up orphaned roles for all guilds on startup
        let guilds: Vec<GuildId> = ready.guilds.iter().map(|g| g.id).collect();
        
        for guild_id in guilds {
            self.cleanup_orphaned_roles(ctx, guild_id).await;
        }

        tracing::info!("Initial orphaned role cleanup completed");
    }

    /// Handle role deletions to clean up database
    pub async fn on_guild_role_delete(&self, guild_id: GuildId, removed_role_id: serenity::all::RoleId, _role_data_if_available: Option<Role>) {
        tracing::debug!(
            guild_id = %guild_id,
            role_id = %removed_role_id,
            "Role deletion event received"
        );

        // Check if this was a booster role and clean up database
        let role_id_i64 = removed_role_id.get() as i64;

        // Find and remove any booster role records with this role ID
        match sqlx::query("DELETE FROM booster_roles WHERE guild_id = ? AND role_id = ?")
            .bind(guild_id.get() as i64)
            .bind(role_id_i64)
            .execute(&*self.db_pool)
            .await
        {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    tracing::info!(
                        guild_id = %guild_id,
                        role_id = %removed_role_id,
                        "Cleaned up booster role database record after role deletion"
                    );
                }
            }
            Err(e) => {
                tracing::error!(
                    guild_id = %guild_id,
                    role_id = %removed_role_id,
                    error = ?e,
                    "Failed to clean up booster role after role deletion"
                );
            }
        }

        // Also clean up any role links with this role ID
        match sqlx::query("DELETE FROM booster_role_links WHERE guild_id = ? AND linked_role_id = ?")
            .bind(guild_id.get() as i64)
            .bind(role_id_i64)
            .execute(&*self.db_pool)
            .await
        {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    tracing::info!(
                        guild_id = %guild_id,
                        role_id = %removed_role_id,
                        "Cleaned up booster role link after role deletion"
                    );
                }
            }
            Err(e) => {
                tracing::error!(
                    guild_id = %guild_id,
                    role_id = %removed_role_id,
                    error = ?e,
                    "Failed to clean up role link after role deletion"
                );
            }
        }
    }
}