//! Shared moderation helpers for F1 and later punish/history suites.
//!
//! Public API is intentionally unused until Wave 1 command suites land.

#![allow(dead_code)]

use crate::data::models::{ModerationAction, ModerationCase};
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter, Timestamp};
use serenity::all::{GuildId, Permissions, RoleId, UserId};
use sqlx::SqlitePool;
use thiserror::Error;

use super::EmbedColor;

/// Maximum stored reason length (characters).
pub const MAX_REASON_LEN: usize = 512;

#[derive(Error, Debug)]
pub enum ModerationError {
    #[error("You are not staff in this server")]
    NotStaff,

    #[error("Invalid reason: {0}")]
    InvalidReason(String),

    #[error("Case #{0} was not found")]
    CaseNotFound(i64),

    #[error("Unknown moderation action: {0}")]
    UnknownAction(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Discord API error: {0}")]
    Discord(#[from] serenity::Error),
}

impl From<ModerationError> for crate::bot::Error {
    fn from(err: ModerationError) -> Self {
        crate::bot::Error::Command(err.to_string())
    }
}

/// Trim whitespace. Empty input becomes `None`.
pub fn normalize_reason(input: Option<&str>) -> Option<String> {
    let trimmed = input.map(str::trim).filter(|s| !s.is_empty())?;
    Some(trimmed.to_string())
}

/// Validate a non-empty reason after normalize.
///
/// Rejects overlong text and bare `@everyone` / `@here` (error, do not strip).
pub fn validate_reason(reason: &str) -> Result<(), ModerationError> {
    if reason.len() > MAX_REASON_LEN {
        return Err(ModerationError::InvalidReason(format!(
            "reason must be at most {MAX_REASON_LEN} characters"
        )));
    }

    // Word-boundary style check: reject if the tokens appear as mention forms.
    if reason.contains("@everyone") || reason.contains("@here") {
        return Err(ModerationError::InvalidReason(
            "reason must not contain @everyone or @here".into(),
        ));
    }

    Ok(())
}

/// Normalize then validate. Returns `Ok(None)` for empty input.
pub fn prepare_reason(input: Option<&str>) -> Result<Option<String>, ModerationError> {
    match normalize_reason(input) {
        None => Ok(None),
        Some(r) => {
            validate_reason(&r)?;
            Ok(Some(r))
        }
    }
}

/// Pure staff check when role ids and permission bits are already known.
///
/// Returns true when the member is guild owner, has ADMINISTRATOR or MANAGE_GUILD,
/// or holds any configured staff role.
pub fn member_is_staff(
    user_id: UserId,
    owner_id: UserId,
    member_permissions: Permissions,
    member_role_ids: &[RoleId],
    staff_role_ids: &[RoleId],
) -> bool {
    if user_id == owner_id {
        return true;
    }
    if member_permissions.contains(Permissions::ADMINISTRATOR)
        || member_permissions.contains(Permissions::MANAGE_GUILD)
    {
        return true;
    }
    member_role_ids
        .iter()
        .any(|role| staff_role_ids.iter().any(|staff| staff == role))
}

/// Load staff role ids from the database for a guild.
pub async fn load_staff_role_ids(
    pool: &SqlitePool,
    guild_id: GuildId,
) -> Result<Vec<RoleId>, ModerationError> {
    let rows = crate::data::models::GuildStaffRole::list(pool, guild_id).await?;
    Ok(rows
        .into_iter()
        .map(|r| RoleId::new(r.role_id as u64))
        .collect())
}

/// Require staff using pre-fetched guild owner, permissions, and roles.
pub fn require_guild_staff(
    user_id: UserId,
    owner_id: UserId,
    member_permissions: Permissions,
    member_role_ids: &[RoleId],
    staff_role_ids: &[RoleId],
) -> Result<(), ModerationError> {
    if member_is_staff(
        user_id,
        owner_id,
        member_permissions,
        member_role_ids,
        staff_role_ids,
    ) {
        Ok(())
    } else {
        Err(ModerationError::NotStaff)
    }
}

fn format_reason(reason: Option<&str>) -> String {
    reason
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("No reason")
        .to_string()
}

fn action_label(action: &str) -> String {
    ModerationAction::parse(action)
        .map(|a| {
            let s = a.as_str();
            let mut chars = s.chars();
            match chars.next() {
                Some(c) => format!("{}{}", c.to_uppercase(), chars.as_str()),
                None => s.to_string(),
            }
        })
        .unwrap_or_else(|| action.to_string())
}

/// Embed confirming a new case (later punish suites).
pub fn case_created_embed(case: &ModerationCase) -> CreateEmbed {
    let mut description = format!(
        "**Case** `#{}`\n**Action** {}\n**Target** `<@{}>` (`{}`)\n**Moderator** `<@{}>`\n**Reason** {}",
        case.case_number,
        action_label(&case.action),
        case.target_id,
        case.target_id,
        case.moderator_id,
        format_reason(case.reason.as_deref()),
    );

    if let Some(secs) = case.duration_seconds {
        description.push_str(&format!("\n**Duration** {secs}s"));
    }

    CreateEmbed::new()
        .title("Moderation case created")
        .description(description)
        .color(EmbedColor::Success.value())
        .footer(CreateEmbedFooter::new(format!(
            "Case #{} · guild {}",
            case.case_number, case.guild_id
        )))
        .timestamp(Timestamp::now())
}

/// Embed for viewing a single case (`caselog` / history).
pub fn case_view_embed(case: &ModerationCase) -> CreateEmbed {
    let mut description = format!(
        "**Case** `#{}`\n**Action** {}\n**Target** `<@{}>` (`{}`)\n**Moderator** `<@{}>`\n**Reason** {}\n**Active** {}",
        case.case_number,
        action_label(&case.action),
        case.target_id,
        case.target_id,
        case.moderator_id,
        format_reason(case.reason.as_deref()),
        if case.is_active() { "yes" } else { "no" },
    );

    if let Some(secs) = case.duration_seconds {
        description.push_str(&format!("\n**Duration** {secs}s"));
    }
    if let Some(related) = case.related_case_id {
        description.push_str(&format!("\n**Related case id** {related}"));
    }
    if let Some(ref created) = case.created_at {
        description.push_str(&format!("\n**Created** {created}"));
    }

    CreateEmbed::new()
        .title(format!("Case #{}", case.case_number))
        .description(description)
        .color(EmbedColor::Primary.value())
        .timestamp(Timestamp::now())
}

pub fn moderation_error_embed(title: impl Into<String>, body: impl Into<String>) -> CreateEmbed {
    CreateEmbed::new()
        .title(title)
        .description(body)
        .color(EmbedColor::Error.value())
        .timestamp(Timestamp::now())
}

pub fn moderation_warning_embed(title: impl Into<String>, body: impl Into<String>) -> CreateEmbed {
    CreateEmbed::new()
        .title(title)
        .description(body)
        .color(EmbedColor::Warning.value())
        .timestamp(Timestamp::now())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_reason_trims_and_empty() {
        assert_eq!(normalize_reason(None), None);
        assert_eq!(normalize_reason(Some("   ")), None);
        assert_eq!(normalize_reason(Some("  spam  ")).as_deref(), Some("spam"));
    }

    #[test]
    fn validate_reason_length_and_mentions() {
        assert!(validate_reason("ok reason").is_ok());
        assert!(validate_reason(&"x".repeat(MAX_REASON_LEN)).is_ok());
        assert!(validate_reason(&"x".repeat(MAX_REASON_LEN + 1)).is_err());
        assert!(validate_reason("hello @everyone").is_err());
        assert!(validate_reason("ping @here please").is_err());
    }

    #[test]
    fn prepare_reason_pipeline() {
        assert_eq!(prepare_reason(None).unwrap(), None);
        assert_eq!(
            prepare_reason(Some("  hi  ")).unwrap().as_deref(),
            Some("hi")
        );
        assert!(prepare_reason(Some("@everyone")).is_err());
    }

    #[test]
    fn member_is_staff_rules() {
        let owner = UserId::new(1);
        let user = UserId::new(2);
        let staff_role = RoleId::new(10);
        let other_role = RoleId::new(11);

        assert!(member_is_staff(
            owner,
            owner,
            Permissions::empty(),
            &[],
            &[]
        ));
        assert!(member_is_staff(
            user,
            owner,
            Permissions::ADMINISTRATOR,
            &[],
            &[]
        ));
        assert!(member_is_staff(
            user,
            owner,
            Permissions::MANAGE_GUILD,
            &[],
            &[]
        ));
        assert!(member_is_staff(
            user,
            owner,
            Permissions::empty(),
            &[staff_role],
            &[staff_role]
        ));
        assert!(!member_is_staff(
            user,
            owner,
            Permissions::empty(),
            &[other_role],
            &[staff_role]
        ));
        assert!(require_guild_staff(
            user,
            owner,
            Permissions::empty(),
            &[other_role],
            &[staff_role]
        )
        .is_err());
    }

    #[test]
    fn embed_builders_use_project_colors() {
        let case = ModerationCase {
            id: 1,
            guild_id: 99,
            case_number: 3,
            action: "warn".into(),
            target_id: 5,
            moderator_id: 6,
            reason: Some("test".into()),
            duration_seconds: None,
            active: 1,
            related_case_id: None,
            created_at: Some("2026-01-01".into()),
            updated_at: None,
        };

        // Build embeds to ensure they do not panic; colors come from EmbedColor.
        let _ = case_created_embed(&case);
        let _ = case_view_embed(&case);
        let _ = moderation_error_embed("Error", "body");
        let _ = moderation_warning_embed("Warn", "body");
        assert_eq!(EmbedColor::Success.value(), 0x62CB77);
        assert_eq!(EmbedColor::Primary.value(), 0xC6AC80);
        assert_eq!(EmbedColor::Error.value(), 0x853535);
    }
}
