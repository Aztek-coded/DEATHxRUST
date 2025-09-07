use crate::data::models::BoosterRole;
use crate::utils::embed_builder::EmbedBuilder;
use crate::bot::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateEmbed, RoleId};
use std::collections::HashSet;

#[poise::command(
    slash_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn cleanup(
    ctx: Context<'_>,
    #[description = "Preview changes without deleting (dry run)"] dry_run: Option<bool>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(Error::Command("This command can only be used in a guild".to_string()))?;
    let dry_run = dry_run.unwrap_or(false);

    tracing::info!(
        guild_id = %guild_id,
        admin_id = %ctx.author().id,
        dry_run = dry_run,
        "Boosterrole cleanup initiated"
    );

    ctx.defer().await?;

    let guild = guild_id.to_partial_guild(&ctx.serenity_context().http).await?;
    let members = guild.members(&ctx.serenity_context().http, None, None).await?;

    let booster_member_ids: HashSet<u64> = members
        .iter()
        .filter(|m| m.premium_since.is_some())
        .map(|m| m.user.id.get())
        .collect();

    let all_roles = BoosterRole::get_all_for_guild(&ctx.data().db_pool, guild_id).await?;

    let mut orphaned_roles = Vec::new();
    let mut stats = CleanupStats::default();

    for role_record in &all_roles {
        let user_id = serenity::UserId::new(role_record.user_id as u64);
        let role_id = RoleId::new(role_record.role_id as u64);

        let is_orphaned = if !booster_member_ids.contains(&user_id.get()) {
            stats.no_boost_count += 1;
            true
        } else if !guild.roles.contains_key(&role_id) {
            stats.role_deleted_count += 1;
            true
        } else {
            let member = guild.member(&ctx.serenity_context().http, user_id).await;
            if member.is_err() {
                stats.member_left_count += 1;
                true
            } else {
                false
            }
        };

        if is_orphaned {
            orphaned_roles.push((user_id, role_id, role_record.role_name.clone()));
        }
    }

    tracing::debug!(
        orphaned_count = orphaned_roles.len(),
        "Found orphaned roles for cleanup"
    );

    if orphaned_roles.is_empty() {
        let embed = EmbedBuilder::success(
            "✨ No Cleanup Needed",
            "All booster roles are properly assigned. No orphaned roles found.",
        );

        ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
            .await?;
        return Ok(());
    }

    if dry_run {
        let role_list = orphaned_roles
            .iter()
            .take(10)
            .map(|(user_id, _, role_name)| format!("• <@{}> - {}", user_id, role_name))
            .collect::<Vec<_>>()
            .join("\n");

        let more_text = if orphaned_roles.len() > 10 {
            format!("\n*...and {} more*", orphaned_roles.len() - 10)
        } else {
            String::new()
        };

        let embed = EmbedBuilder::info(
            "🔍 Cleanup Preview (Dry Run)",
            &format!(
                "Found **{}** orphaned role(s) that would be removed:",
                orphaned_roles.len()
            ),
        )
        .field("Orphaned Roles", &format!("{}{}", role_list, more_text), false)
        .field(
            "Breakdown",
            &format!(
                "• No longer boosting: {}\n• Role deleted: {}\n• Member left: {}",
                stats.no_boost_count, stats.role_deleted_count, stats.member_left_count
            ),
            false,
        )
        .footer(serenity::CreateEmbedFooter::new("Run without dry_run to actually remove these roles"));

        ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
            .await?;
    } else {
        let mut removed_count = 0;
        let mut failed_count = 0;

        for (user_id, role_id, _) in &orphaned_roles {
            if guild.roles.contains_key(role_id) {
                if let Err(e) = guild_id
                    .delete_role(&ctx.serenity_context().http, *role_id)
                    .await
                {
                    tracing::error!(
                        "Failed to delete role {} in guild {}: {}",
                        role_id,
                        guild_id,
                        e
                    );
                    failed_count += 1;
                } else {
                    removed_count += 1;
                }
            }

            if let Err(e) = BoosterRole::delete(&ctx.data().db_pool, guild_id, *user_id).await {
                tracing::error!(
                    "Failed to delete database record for user {} in guild {}: {}",
                    user_id,
                    guild_id,
                    e
                );
            }
        }

        let embed = if failed_count > 0 {
            EmbedBuilder::warning(
                "⚠️ Cleanup Partially Complete",
                &format!(
                    "Removed **{}** orphaned role(s), but **{}** failed to delete.",
                    removed_count, failed_count
                ),
            )
        } else {
            EmbedBuilder::success(
                "✅ Cleanup Complete",
                &format!("Successfully removed **{}** orphaned role(s).", removed_count),
            )
        }
        .field(
            "Statistics",
            &format!(
                "• No longer boosting: {}\n• Role deleted: {}\n• Member left: {}",
                stats.no_boost_count, stats.role_deleted_count, stats.member_left_count
            ),
            false,
        );

        ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
            .await?;

        tracing::info!(
            guild_id = %guild_id,
            removed_count = removed_count,
            failed_count = failed_count,
            "Cleanup operation completed"
        );
    }

    Ok(())
}

#[derive(Default)]
struct CleanupStats {
    no_boost_count: usize,
    role_deleted_count: usize,
    member_left_count: usize,
}