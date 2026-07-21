//! `timeout` and `untimeout` commands.

use super::{
    audit_reason, ensure_can_moderate_member, ensure_moderator, parse_duration_to_seconds,
    reply_mod_error,
};
use crate::bot::{Context, Error};
use crate::data::models::ModerationAction;
use crate::utils::{case_created_embed, prepare_reason, ResponseHelper};
use chrono::{Duration as ChronoDuration, Utc};
use serenity::all::{EditMember, Member, Permissions, Timestamp};

/// Timeout a member using Discord communication disable.
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Moderation",
    required_bot_permissions = "MODERATE_MEMBERS",
    broadcast_typing
)]
pub async fn timeout(
    ctx: Context<'_>,
    #[description = "Member to timeout"] member: Member,
    #[description = "Duration (e.g. 30m, 1h, 1d)"] duration: String,
    #[description = "Reason"] reason: Option<String>,
) -> Result<(), Error> {
    let (guild_id, moderator, guild) =
        match ensure_moderator(ctx, Permissions::MODERATE_MEMBERS).await {
            Ok(v) => v,
            Err(e) => return reply_mod_error(ctx, "Permission denied", e.to_string()).await,
        };

    if let Err(e) = ensure_can_moderate_member(ctx, &guild, &moderator, &member).await {
        return reply_mod_error(ctx, "Cannot timeout", e.to_string()).await;
    }

    let secs = match parse_duration_to_seconds(&duration) {
        Ok(s) => s,
        Err(e) => return reply_mod_error(ctx, "Invalid duration", e.to_string()).await,
    };

    let reason = match prepare_reason(reason.as_deref()) {
        Ok(r) => r,
        Err(e) => return reply_mod_error(ctx, "Invalid reason", e.to_string()).await,
    };

    let until_chrono = Utc::now() + ChronoDuration::seconds(secs);
    let until = match Timestamp::from_unix_timestamp(until_chrono.timestamp()) {
        Ok(t) => t,
        Err(_) => {
            return reply_mod_error(ctx, "Invalid duration", "Could not build timeout end time")
                .await;
        }
    };

    let audit = audit_reason(moderator.user.id, reason.as_deref());
    let http = ctx.serenity_context();
    let mut target = member.clone();

    let builder = EditMember::new()
        .disable_communication_until_datetime(until)
        .audit_log_reason(&audit);

    if let Err(e) = target.edit(http, builder).await {
        tracing::warn!(
            guild_id = %guild_id,
            target_id = %target.user.id,
            error = %e,
            "Timeout Discord action failed"
        );
        return reply_mod_error(ctx, "Timeout failed", e.to_string()).await;
    }

    tracing::info!(
        guild_id = %guild_id,
        target_id = %target.user.id,
        moderator_id = %moderator.user.id,
        duration_seconds = secs,
        reason = %audit,
        "Member timed out"
    );

    match ctx
        .data()
        .create_moderation_case(
            guild_id,
            ModerationAction::Timeout,
            target.user.id,
            moderator.user.id,
            reason.as_deref(),
            Some(secs),
            None,
        )
        .await
    {
        Ok(case) => {
            ResponseHelper::send_embed(ctx, case_created_embed(&case)).await?;
        }
        Err(e) => {
            tracing::error!(
                guild_id = %guild_id,
                target_id = %target.user.id,
                error = %e,
                "Timeout succeeded but case write failed"
            );
            ResponseHelper::send_warning(
                ctx,
                "Timeout applied",
                "The member was timed out, but the case was not stored.",
            )
            .await?;
        }
    }

    Ok(())
}

/// Remove a member's timeout.
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Moderation",
    required_bot_permissions = "MODERATE_MEMBERS",
    broadcast_typing
)]
pub async fn untimeout(
    ctx: Context<'_>,
    #[description = "Member to remove timeout from"] member: Member,
    #[description = "Reason"] reason: Option<String>,
) -> Result<(), Error> {
    let (guild_id, moderator, guild) =
        match ensure_moderator(ctx, Permissions::MODERATE_MEMBERS).await {
            Ok(v) => v,
            Err(e) => return reply_mod_error(ctx, "Permission denied", e.to_string()).await,
        };

    if let Err(e) = ensure_can_moderate_member(ctx, &guild, &moderator, &member).await {
        return reply_mod_error(ctx, "Cannot untimeout", e.to_string()).await;
    }

    let reason = match prepare_reason(reason.as_deref()) {
        Ok(r) => r,
        Err(e) => return reply_mod_error(ctx, "Invalid reason", e.to_string()).await,
    };

    let audit = audit_reason(moderator.user.id, reason.as_deref());
    let http = ctx.serenity_context();
    let mut target = member.clone();

    let builder = EditMember::new()
        .enable_communication()
        .audit_log_reason(&audit);

    if let Err(e) = target.edit(http, builder).await {
        tracing::warn!(
            guild_id = %guild_id,
            target_id = %target.user.id,
            error = %e,
            "Untimeout Discord action failed"
        );
        return reply_mod_error(ctx, "Untimeout failed", e.to_string()).await;
    }

    tracing::info!(
        guild_id = %guild_id,
        target_id = %target.user.id,
        moderator_id = %moderator.user.id,
        reason = %audit,
        "Member untimeout"
    );

    match ctx
        .data()
        .create_moderation_case(
            guild_id,
            ModerationAction::Untimeout,
            target.user.id,
            moderator.user.id,
            reason.as_deref(),
            None,
            None,
        )
        .await
    {
        Ok(case) => {
            ResponseHelper::send_embed(ctx, case_created_embed(&case)).await?;
        }
        Err(e) => {
            tracing::error!(
                guild_id = %guild_id,
                target_id = %target.user.id,
                error = %e,
                "Untimeout succeeded but case write failed"
            );
            ResponseHelper::send_warning(
                ctx,
                "Timeout removed",
                "The timeout was removed, but the case was not stored.",
            )
            .await?;
        }
    }

    Ok(())
}
