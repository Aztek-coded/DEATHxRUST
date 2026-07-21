//! Core moderation punish commands (suite: moderation-punish).
//!
//! Permission rule: guild staff (F1) **or** the Discord permission for the action.
//! Discord action runs first; case write follows on success.

pub mod ban;
pub mod timeout;
pub mod warn;

pub use ban::{ban, softban, unban};
pub use timeout::{timeout, untimeout};
pub use warn::{warn, warnings};

use crate::bot::{Context, Error};
use crate::utils::{
    load_staff_role_ids, member_is_staff, moderation_error_embed, ModerationError, ResponseHelper,
};
use serenity::all::{GuildId, Member, PartialGuild, Permissions, RoleId, UserId};

/// Discord maximum timeout length (28 days) in seconds.
pub const MAX_TIMEOUT_SECONDS: i64 = 28 * 24 * 60 * 60;

/// Parse human duration into seconds.
///
/// Accepts plain integers (seconds) or a number with a unit suffix:
/// `s`, `m`, `h`, `d`, `w` (case-insensitive). Rejects zero and values over
/// [`MAX_TIMEOUT_SECONDS`].
pub fn parse_duration_to_seconds(input: &str) -> Result<i64, String> {
    let s = input.trim().to_lowercase();
    if s.is_empty() {
        return Err("duration is required (e.g. 30m, 1h, 1d)".into());
    }

    let (num_part, mult) = match s.chars().last() {
        Some(c) if c.is_ascii_digit() => (s.as_str(), 1_i64),
        Some('s') => (&s[..s.len() - 1], 1),
        Some('m') => (&s[..s.len() - 1], 60),
        Some('h') => (&s[..s.len() - 1], 3600),
        Some('d') => (&s[..s.len() - 1], 86_400),
        Some('w') => (&s[..s.len() - 1], 604_800),
        _ => {
            return Err("invalid duration; use seconds or a unit suffix (s/m/h/d/w)".into());
        }
    };

    let num_part = num_part.trim();
    if num_part.is_empty() {
        return Err("invalid duration number".into());
    }

    let n: i64 = num_part
        .parse()
        .map_err(|_| "invalid duration number".to_string())?;

    if n <= 0 {
        return Err("duration must be greater than zero".into());
    }

    let secs = n
        .checked_mul(mult)
        .ok_or_else(|| "duration is too large".to_string())?;

    if secs > MAX_TIMEOUT_SECONDS {
        return Err(format!(
            "duration must be at most 28 days ({MAX_TIMEOUT_SECONDS} seconds)"
        ));
    }

    Ok(secs)
}

/// Clamp ban delete-message days to Discord's 0–7 range.
pub fn clamp_delete_days(days: Option<u8>) -> u8 {
    days.unwrap_or(0).min(7)
}

/// Softban default delete history (1 day) when omitted.
pub fn softban_delete_days(days: Option<u8>) -> u8 {
    days.unwrap_or(1).min(7)
}

fn compute_permissions(member: &Member, guild: &PartialGuild) -> Permissions {
    let everyone = guild.id.everyone_role();
    let mut perms = guild
        .roles
        .get(&everyone)
        .map(|r| r.permissions)
        .unwrap_or_else(Permissions::empty);

    for role_id in &member.roles {
        if let Some(role) = guild.roles.get(role_id) {
            perms |= role.permissions;
        }
    }

    if member.user.id == guild.owner_id {
        perms |= Permissions::all();
    }

    perms
}

fn highest_role_position(role_ids: &[RoleId], guild: &PartialGuild) -> u16 {
    role_ids
        .iter()
        .filter_map(|id| guild.roles.get(id).map(|r| r.position))
        .max()
        .unwrap_or(0)
}

/// Require guild context.
#[allow(clippy::result_large_err)] // Project `Error` is intentionally large
pub fn require_guild_id(ctx: Context<'_>) -> Result<GuildId, Error> {
    ctx.guild_id()
        .ok_or_else(|| Error::Command("This command can only be used in a server".into()))
}

/// Ensure the author may run a moderation action.
///
/// Allowed when the author is staff (owner / admin / manage guild / configured
/// staff role) **or** holds `needed_perm`.
#[allow(clippy::result_large_err)] // Project `Error` is intentionally large
pub async fn ensure_moderator(
    ctx: Context<'_>,
    needed_perm: Permissions,
) -> Result<(GuildId, Member, PartialGuild), Error> {
    let guild_id = require_guild_id(ctx)?;
    let http = ctx.serenity_context().http.clone();

    let author_member = ctx
        .author_member()
        .await
        .ok_or_else(|| Error::Command("Could not load your member profile".into()))?
        .into_owned();

    let guild = guild_id.to_partial_guild(&http).await?;
    let author_perms = compute_permissions(&author_member, &guild);
    let staff_roles = load_staff_role_ids(&ctx.data().db_pool, guild_id).await?;

    let is_staff = member_is_staff(
        author_member.user.id,
        guild.owner_id,
        author_perms,
        &author_member.roles,
        &staff_roles,
    );

    if !is_staff && !author_perms.contains(needed_perm) {
        return Err(ModerationError::NotStaff.into());
    }

    // Bot must hold the Discord permission for the action.
    let bot_id = ctx.framework().bot_id;
    let bot_member = guild_id
        .member(&http, bot_id)
        .await
        .map_err(|_| Error::Command("Could not load the bot member in this server".into()))?;
    let bot_perms = compute_permissions(&bot_member, &guild);
    if !bot_perms.contains(needed_perm) && !bot_perms.contains(Permissions::ADMINISTRATOR) {
        return Err(Error::Command(format!(
            "I need the `{}` permission to run this command",
            permission_label(needed_perm)
        )));
    }

    Ok((guild_id, author_member, guild))
}

fn permission_label(perm: Permissions) -> &'static str {
    if perm.contains(Permissions::BAN_MEMBERS) {
        "Ban Members"
    } else if perm.contains(Permissions::MODERATE_MEMBERS) {
        "Moderate Members"
    } else if perm.contains(Permissions::KICK_MEMBERS) {
        "Kick Members"
    } else {
        "required moderation"
    }
}

/// Hierarchy and self-target checks for actions that target a guild member.
#[allow(clippy::result_large_err)] // Project `Error` is intentionally large
pub async fn ensure_can_moderate_member(
    ctx: Context<'_>,
    guild: &PartialGuild,
    moderator: &Member,
    target: &Member,
) -> Result<(), Error> {
    if target.user.id == moderator.user.id {
        return Err(Error::Command("You cannot moderate yourself".into()));
    }

    if target.user.id == guild.owner_id {
        return Err(Error::Command(
            "You cannot moderate the server owner".into(),
        ));
    }

    if target.user.id == ctx.framework().bot_id {
        return Err(Error::Command(
            "You cannot moderate the bot with this command".into(),
        ));
    }

    // Moderators who are not the owner must outrank the target.
    if moderator.user.id != guild.owner_id {
        let mod_pos = highest_role_position(&moderator.roles, guild);
        let target_pos = highest_role_position(&target.roles, guild);
        if target_pos >= mod_pos {
            return Err(Error::Command(
                "You cannot moderate a member with an equal or higher top role".into(),
            ));
        }
    }

    // Bot must outrank the target (unless target has no roles / bot is owner — rare).
    let http = &ctx.serenity_context().http;
    let bot_member = guild.id.member(http, ctx.framework().bot_id).await?;
    if bot_member.user.id != guild.owner_id {
        let bot_pos = highest_role_position(&bot_member.roles, guild);
        let target_pos = highest_role_position(&target.roles, guild);
        if target_pos >= bot_pos {
            return Err(Error::Command(
                "I cannot moderate this member because their top role is equal or higher than mine"
                    .into(),
            ));
        }
    }

    Ok(())
}

/// Build a Discord audit-log reason (max 512 chars).
pub fn audit_reason(moderator_id: UserId, reason: Option<&str>) -> String {
    let raw = match reason {
        Some(r) if !r.is_empty() => format!("[{moderator_id}] {r}"),
        _ => format!("Action by {moderator_id}"),
    };
    if raw.len() > 512 {
        raw.chars().take(512).collect()
    } else {
        raw
    }
}

/// Reply with a moderation error embed and return `Ok(())` so the framework
/// does not double-report.
#[allow(clippy::result_large_err)] // Project `Error` is intentionally large
pub async fn reply_mod_error(
    ctx: Context<'_>,
    title: &str,
    body: impl Into<String>,
) -> Result<(), Error> {
    ResponseHelper::send_embed(ctx, moderation_error_embed(title, body)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_duration_units() {
        assert_eq!(parse_duration_to_seconds("30").unwrap(), 30);
        assert_eq!(parse_duration_to_seconds("30s").unwrap(), 30);
        assert_eq!(parse_duration_to_seconds("30m").unwrap(), 1800);
        assert_eq!(parse_duration_to_seconds("1h").unwrap(), 3600);
        assert_eq!(parse_duration_to_seconds("1d").unwrap(), 86_400);
        assert_eq!(parse_duration_to_seconds("1w").unwrap(), 604_800);
        assert_eq!(parse_duration_to_seconds(" 2H ").unwrap(), 7200);
    }

    #[test]
    fn parse_duration_rejects_bad() {
        assert!(parse_duration_to_seconds("").is_err());
        assert!(parse_duration_to_seconds("0").is_err());
        assert!(parse_duration_to_seconds("-5m").is_err());
        assert!(parse_duration_to_seconds("abc").is_err());
        assert!(parse_duration_to_seconds("29d").is_err());
        assert!(parse_duration_to_seconds(&format!("{}s", MAX_TIMEOUT_SECONDS + 1)).is_err());
        assert_eq!(
            parse_duration_to_seconds(&format!("{MAX_TIMEOUT_SECONDS}s")).unwrap(),
            MAX_TIMEOUT_SECONDS
        );
    }

    #[test]
    fn clamp_delete_days_bounds() {
        assert_eq!(clamp_delete_days(None), 0);
        assert_eq!(clamp_delete_days(Some(3)), 3);
        assert_eq!(clamp_delete_days(Some(99)), 7);
        assert_eq!(softban_delete_days(None), 1);
        assert_eq!(softban_delete_days(Some(0)), 0);
        assert_eq!(softban_delete_days(Some(9)), 7);
    }
}
