//! `warn` and `warnings` commands.

use super::{ensure_can_moderate_member, reply_mod_error};
use crate::bot::{Context, Error};
use crate::data::models::{ModerationAction, ModerationCase};
use crate::utils::{case_created_embed, prepare_reason, EmbedColor, ResponseHelper};
use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage, Member, Permissions};

/// Maximum warning lines shown in one reply.
const WARNINGS_LIST_LIMIT: i64 = 15;

/// Warn a member and try to DM them.
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Moderation",
    broadcast_typing
)]
pub async fn warn(
    ctx: Context<'_>,
    #[description = "Member to warn"] member: Member,
    #[description = "Reason"] reason: Option<String>,
) -> Result<(), Error> {
    // Staff or Moderate/Kick Members may warn.
    let needed = Permissions::MODERATE_MEMBERS | Permissions::KICK_MEMBERS;
    let (guild_id, moderator, guild) = match ensure_moderator_any(ctx, needed).await {
        Ok(v) => v,
        Err(e) => return reply_mod_error(ctx, "Permission denied", e.to_string()).await,
    };

    if let Err(e) = ensure_can_moderate_member(ctx, &guild, &moderator, &member).await {
        return reply_mod_error(ctx, "Cannot warn", e.to_string()).await;
    }

    let reason = match prepare_reason(reason.as_deref()) {
        Ok(r) => r,
        Err(e) => return reply_mod_error(ctx, "Invalid reason", e.to_string()).await,
    };

    let guild_name = guild.name.clone();
    let reason_text = reason
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("No reason");

    let dm_body = format!(
        "You received a warning in **{}**.\n**Reason:** {}",
        guild_name, reason_text
    );

    let dm_ok = member
        .user
        .direct_message(
            ctx.serenity_context(),
            CreateMessage::new().content(dm_body),
        )
        .await
        .is_ok();

    if !dm_ok {
        tracing::warn!(
            guild_id = %guild_id,
            target_id = %member.user.id,
            "Warn DM failed (user may have DMs closed)"
        );
    }

    match ctx
        .data()
        .create_moderation_case(
            guild_id,
            ModerationAction::Warn,
            member.user.id,
            moderator.user.id,
            reason.as_deref(),
            None,
            None,
        )
        .await
    {
        Ok(case) => {
            let mut embed = case_created_embed(&case);
            if !dm_ok {
                embed = embed.footer(CreateEmbedFooter::new(
                    "Case stored · DM could not be delivered",
                ));
            }
            ResponseHelper::send_embed(ctx, embed).await?;
        }
        Err(e) => {
            tracing::error!(
                guild_id = %guild_id,
                target_id = %member.user.id,
                error = %e,
                "Warn case write failed"
            );
            // No Discord punish action beyond DM; report failure.
            return reply_mod_error(
                ctx,
                "Warn failed",
                "Could not store the warning case. Try again.",
            )
            .await;
        }
    }

    tracing::info!(
        guild_id = %guild_id,
        target_id = %member.user.id,
        moderator_id = %moderator.user.id,
        dm_ok,
        "Member warned"
    );

    Ok(())
}

/// List warnings for a member.
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Moderation",
    aliases("warns"),
    broadcast_typing
)]
pub async fn warnings(
    ctx: Context<'_>,
    #[description = "Member to inspect"] member: Member,
) -> Result<(), Error> {
    let needed = Permissions::MODERATE_MEMBERS | Permissions::KICK_MEMBERS;
    let (guild_id, _moderator, _guild) = match ensure_moderator_any(ctx, needed).await {
        Ok(v) => v,
        Err(e) => return reply_mod_error(ctx, "Permission denied", e.to_string()).await,
    };

    let pool = &ctx.data().db_pool;
    let total = ModerationCase::count_for_target_action(
        pool,
        guild_id,
        member.user.id,
        ModerationAction::Warn,
    )
    .await?;

    if total == 0 {
        ResponseHelper::send_info(
            ctx,
            "Warnings",
            &format!("No warnings for <@{}>.", member.user.id),
        )
        .await?;
        return Ok(());
    }

    // Fetch recent cases for target and keep warn actions only.
    let rows =
        ModerationCase::list_for_target(pool, guild_id, member.user.id, WARNINGS_LIST_LIMIT * 3, 0)
            .await?;

    let warns: Vec<_> = rows
        .into_iter()
        .filter(|c| c.action == ModerationAction::Warn.as_str())
        .take(WARNINGS_LIST_LIMIT as usize)
        .collect();

    let mut lines = Vec::with_capacity(warns.len());
    for case in &warns {
        let reason = case
            .reason
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or("No reason");
        // Keep each line short so the full embed stays under Discord's 4096 limit.
        let reason_short = if reason.chars().count() > 120 {
            format!("{}…", reason.chars().take(119).collect::<String>())
        } else {
            reason.to_string()
        };
        let when = case.created_at.as_deref().unwrap_or("unknown time");
        lines.push(format!(
            "**#{}** · mod `<@{}>` · {}\n└ {}",
            case.case_number, case.moderator_id, when, reason_short
        ));
    }

    let mut description = if lines.is_empty() {
        format!("Total warnings: **{total}** (could not load rows).")
    } else {
        format!("Total: **{total}**\n\n{}", lines.join("\n\n"))
    };
    if description.len() > 3900 {
        description.truncate(3900);
        description.push('…');
    }

    let embed = CreateEmbed::new()
        .title(format!("Warnings for {}", member.user.name))
        .description(description)
        .color(EmbedColor::Primary.value())
        .footer(CreateEmbedFooter::new(format!(
            "Showing up to {WARNINGS_LIST_LIMIT} · user {}",
            member.user.id
        )));

    ResponseHelper::send_embed(ctx, embed).await?;
    Ok(())
}

/// Like [`super::ensure_moderator`] but treats `needed` as "any of these bits".
async fn ensure_moderator_any(
    ctx: Context<'_>,
    needed_any: Permissions,
) -> Result<(serenity::all::GuildId, Member, serenity::all::PartialGuild), Error> {
    use crate::utils::{load_staff_role_ids, member_is_staff};

    let guild_id = super::require_guild_id(ctx)?;
    let http = ctx.serenity_context().http.clone();

    let author_member = ctx
        .author_member()
        .await
        .ok_or_else(|| Error::Command("Could not load your member profile".into()))?
        .into_owned();

    let guild = guild_id.to_partial_guild(&http).await?;

    // Reuse permission aggregation from ensure_moderator by calling it with one
    // bit if possible; for multi-bit OR we inline the staff check.
    let everyone = guild.id.everyone_role();
    let mut author_perms = guild
        .roles
        .get(&everyone)
        .map(|r| r.permissions)
        .unwrap_or_else(Permissions::empty);
    for role_id in &author_member.roles {
        if let Some(role) = guild.roles.get(role_id) {
            author_perms |= role.permissions;
        }
    }
    if author_member.user.id == guild.owner_id {
        author_perms |= Permissions::all();
    }

    let staff_roles = load_staff_role_ids(&ctx.data().db_pool, guild_id).await?;
    let is_staff = member_is_staff(
        author_member.user.id,
        guild.owner_id,
        author_perms,
        &author_member.roles,
        &staff_roles,
    );

    let has_needed = !author_perms.intersection(needed_any).is_empty()
        || author_perms.contains(Permissions::ADMINISTRATOR);

    if !is_staff && !has_needed {
        return Err(crate::utils::ModerationError::NotStaff.into());
    }

    Ok((guild_id, author_member, guild))
}
