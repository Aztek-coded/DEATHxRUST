//! `ban`, `unban`, and `softban` commands.

use super::{
    audit_reason, clamp_delete_days, ensure_can_moderate_member, ensure_moderator, reply_mod_error,
    softban_delete_days,
};
use crate::bot::{Context, Error};
use crate::data::models::ModerationAction;
use crate::utils::{case_created_embed, prepare_reason, ResponseHelper};
use serenity::all::{Member, Permissions, User};

/// Ban a member from the guild.
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Moderation",
    required_bot_permissions = "BAN_MEMBERS",
    broadcast_typing
)]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "Member to ban"] member: Member,
    #[description = "Days of messages to delete (0-7)"] delete_history: Option<u8>,
    #[description = "Reason"] reason: Option<String>,
) -> Result<(), Error> {
    let (guild_id, moderator, guild) = match ensure_moderator(ctx, Permissions::BAN_MEMBERS).await {
        Ok(v) => v,
        Err(e) => return reply_mod_error(ctx, "Permission denied", e.to_string()).await,
    };

    if let Err(e) = ensure_can_moderate_member(ctx, &guild, &moderator, &member).await {
        return reply_mod_error(ctx, "Cannot ban", e.to_string()).await;
    }

    let reason = match prepare_reason(reason.as_deref()) {
        Ok(r) => r,
        Err(e) => return reply_mod_error(ctx, "Invalid reason", e.to_string()).await,
    };

    let dmd = clamp_delete_days(delete_history);
    let audit = audit_reason(moderator.user.id, reason.as_deref());
    let http = &ctx.serenity_context().http;
    let target_id = member.user.id;

    if let Err(e) = member.ban_with_reason(http, dmd, &audit).await {
        tracing::warn!(
            guild_id = %guild_id,
            target_id = %target_id,
            error = %e,
            "Ban Discord action failed"
        );
        return reply_mod_error(ctx, "Ban failed", e.to_string()).await;
    }

    tracing::info!(
        guild_id = %guild_id,
        target_id = %target_id,
        moderator_id = %moderator.user.id,
        delete_days = dmd,
        "Member banned"
    );

    match ctx
        .data()
        .create_moderation_case(
            guild_id,
            ModerationAction::Ban,
            target_id,
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
                target_id = %target_id,
                error = %e,
                "Ban succeeded but case write failed"
            );
            ResponseHelper::send_warning(
                ctx,
                "Ban applied",
                "The member was banned, but the case was not stored.",
            )
            .await?;
        }
    }

    Ok(())
}

/// Unban a user by id or mention.
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Moderation",
    required_bot_permissions = "BAN_MEMBERS",
    broadcast_typing
)]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "User to unban"] user: User,
    #[description = "Reason"] reason: Option<String>,
) -> Result<(), Error> {
    let (guild_id, moderator, _guild) = match ensure_moderator(ctx, Permissions::BAN_MEMBERS).await
    {
        Ok(v) => v,
        Err(e) => return reply_mod_error(ctx, "Permission denied", e.to_string()).await,
    };

    if user.id == moderator.user.id {
        return reply_mod_error(ctx, "Cannot unban", "You cannot unban yourself").await;
    }

    let reason = match prepare_reason(reason.as_deref()) {
        Ok(r) => r,
        Err(e) => return reply_mod_error(ctx, "Invalid reason", e.to_string()).await,
    };

    let audit = audit_reason(moderator.user.id, reason.as_deref());
    let http = &ctx.serenity_context().http;

    if let Err(e) = guild_id.unban(http, user.id).await {
        tracing::warn!(
            guild_id = %guild_id,
            target_id = %user.id,
            error = %e,
            "Unban Discord action failed"
        );
        return reply_mod_error(ctx, "Unban failed", e.to_string()).await;
    }

    tracing::info!(
        guild_id = %guild_id,
        target_id = %user.id,
        moderator_id = %moderator.user.id,
        reason = %audit,
        "User unbanned"
    );

    match ctx
        .data()
        .create_moderation_case(
            guild_id,
            ModerationAction::Unban,
            user.id,
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
                target_id = %user.id,
                error = %e,
                "Unban succeeded but case write failed"
            );
            ResponseHelper::send_warning(
                ctx,
                "Unban applied",
                "The user was unbanned, but the case was not stored.",
            )
            .await?;
        }
    }

    Ok(())
}

/// Softban: ban (to delete messages) then unban.
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Moderation",
    required_bot_permissions = "BAN_MEMBERS",
    broadcast_typing
)]
pub async fn softban(
    ctx: Context<'_>,
    #[description = "Member to softban"] member: Member,
    #[description = "Days of messages to delete (0-7, default 1)"] delete_history: Option<u8>,
    #[description = "Reason"] reason: Option<String>,
) -> Result<(), Error> {
    let (guild_id, moderator, guild) = match ensure_moderator(ctx, Permissions::BAN_MEMBERS).await {
        Ok(v) => v,
        Err(e) => return reply_mod_error(ctx, "Permission denied", e.to_string()).await,
    };

    if let Err(e) = ensure_can_moderate_member(ctx, &guild, &moderator, &member).await {
        return reply_mod_error(ctx, "Cannot softban", e.to_string()).await;
    }

    let reason = match prepare_reason(reason.as_deref()) {
        Ok(r) => r,
        Err(e) => return reply_mod_error(ctx, "Invalid reason", e.to_string()).await,
    };

    let dmd = softban_delete_days(delete_history);
    let audit = audit_reason(moderator.user.id, reason.as_deref());
    let http = &ctx.serenity_context().http;
    let target_id = member.user.id;

    if let Err(e) = member.ban_with_reason(http, dmd, &audit).await {
        tracing::warn!(
            guild_id = %guild_id,
            target_id = %target_id,
            error = %e,
            "Softban ban step failed"
        );
        return reply_mod_error(ctx, "Softban failed", e.to_string()).await;
    }

    let unban_ok = match guild_id.unban(http, target_id).await {
        Ok(()) => true,
        Err(e) => {
            tracing::error!(
                guild_id = %guild_id,
                target_id = %target_id,
                error = %e,
                "Softban unban step failed after ban"
            );
            false
        }
    };

    tracing::info!(
        guild_id = %guild_id,
        target_id = %target_id,
        moderator_id = %moderator.user.id,
        delete_days = dmd,
        unban_ok,
        "Member softbanned"
    );

    match ctx
        .data()
        .create_moderation_case(
            guild_id,
            ModerationAction::Softban,
            target_id,
            moderator.user.id,
            reason.as_deref(),
            None,
            None,
        )
        .await
    {
        Ok(case) => {
            if unban_ok {
                ResponseHelper::send_embed(ctx, case_created_embed(&case)).await?;
            } else {
                ResponseHelper::send_warning(
                    ctx,
                    "Softban partial",
                    &format!(
                        "The member was banned (messages deleted) but the automatic unban failed. Case #{} was stored. Unban them manually.",
                        case.case_number
                    ),
                )
                .await?;
            }
        }
        Err(e) => {
            tracing::error!(
                guild_id = %guild_id,
                target_id = %target_id,
                error = %e,
                "Softban Discord steps done but case write failed"
            );
            if unban_ok {
                ResponseHelper::send_warning(
                    ctx,
                    "Softban applied",
                    "The softban completed, but the case was not stored.",
                )
                .await?;
            } else {
                ResponseHelper::send_warning(
                    ctx,
                    "Softban partial",
                    "The member was banned but unban failed, and the case was not stored. Unban them manually.",
                )
                .await?;
            }
        }
    }

    Ok(())
}
