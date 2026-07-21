//! Moderation case store (F1 foundation).
//!
//! Case numbers are allocated per guild via `guild_moderation_counters` so concurrent
//! writers do not race on `MAX(case_number)`.
//!
//! Public API is for later suites (`moderation-punish`, `moderation-history`); allow
//! unused-item warnings until those command modules land.

#![allow(dead_code)]

use serenity::all::{GuildId, UserId};
use sqlx::{FromRow, SqlitePool};

/// Stable action labels stored as TEXT in `moderation_cases.action`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModerationAction {
    Warn,
    Ban,
    Unban,
    Softban,
    Timeout,
    Untimeout,
}

impl ModerationAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Warn => "warn",
            Self::Ban => "ban",
            Self::Unban => "unban",
            Self::Softban => "softban",
            Self::Timeout => "timeout",
            Self::Untimeout => "untimeout",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "warn" => Some(Self::Warn),
            "ban" => Some(Self::Ban),
            "unban" => Some(Self::Unban),
            "softban" => Some(Self::Softban),
            "timeout" => Some(Self::Timeout),
            "untimeout" => Some(Self::Untimeout),
            _ => None,
        }
    }
}

impl std::fmt::Display for ModerationAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct ModerationCase {
    pub id: i64,
    pub guild_id: i64,
    pub case_number: i64,
    pub action: String,
    pub target_id: i64,
    pub moderator_id: i64,
    pub reason: Option<String>,
    pub duration_seconds: Option<i64>,
    /// SQLite stores as INTEGER 0/1.
    pub active: i64,
    pub related_case_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl ModerationCase {
    pub fn action_enum(&self) -> Option<ModerationAction> {
        ModerationAction::parse(&self.action)
    }

    pub fn is_active(&self) -> bool {
        self.active != 0
    }

    /// Create a case and assign the next per-guild case number.
    ///
    /// Uses a transaction and `guild_moderation_counters` so concurrent creates
    /// do not collide on `(guild_id, case_number)`.
    pub async fn create(
        pool: &SqlitePool,
        guild_id: GuildId,
        action: ModerationAction,
        target_id: UserId,
        moderator_id: UserId,
        reason: Option<&str>,
        duration_seconds: Option<i64>,
        related_case_id: Option<i64>,
    ) -> Result<Self, sqlx::Error> {
        let guild_i64 = guild_id.get() as i64;
        let target_i64 = target_id.get() as i64;
        let moderator_i64 = moderator_id.get() as i64;

        let mut tx = pool.begin().await?;

        // Allocate next case number: first row inserts 1; later rows increment.
        let case_number: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO guild_moderation_counters (guild_id, last_case_number)
            VALUES (?, 1)
            ON CONFLICT(guild_id) DO UPDATE SET
                last_case_number = last_case_number + 1
            RETURNING last_case_number
            "#,
        )
        .bind(guild_i64)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO moderation_cases (
                guild_id,
                case_number,
                action,
                target_id,
                moderator_id,
                reason,
                duration_seconds,
                active,
                related_case_id
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, 1, ?)
            "#,
        )
        .bind(guild_i64)
        .bind(case_number)
        .bind(action.as_str())
        .bind(target_i64)
        .bind(moderator_i64)
        .bind(reason)
        .bind(duration_seconds)
        .bind(related_case_id)
        .execute(&mut *tx)
        .await?;

        let row = sqlx::query_as::<_, Self>(
            r#"
            SELECT * FROM moderation_cases
            WHERE guild_id = ? AND case_number = ?
            "#,
        )
        .bind(guild_i64)
        .bind(case_number)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        tracing::info!(
            guild_id = %guild_id,
            case_number = case_number,
            action = %action,
            target_id = %target_id,
            moderator_id = %moderator_id,
            "Moderation case created"
        );

        Ok(row)
    }

    pub async fn get(
        pool: &SqlitePool,
        guild_id: GuildId,
        case_number: i64,
    ) -> Result<Option<Self>, sqlx::Error> {
        tracing::debug!(
            guild_id = %guild_id,
            case_number = case_number,
            "Fetching moderation case"
        );

        sqlx::query_as::<_, Self>(
            r#"
            SELECT * FROM moderation_cases
            WHERE guild_id = ? AND case_number = ?
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(case_number)
        .fetch_optional(pool)
        .await
    }

    pub async fn list_for_target(
        pool: &SqlitePool,
        guild_id: GuildId,
        target_id: UserId,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"
            SELECT * FROM moderation_cases
            WHERE guild_id = ? AND target_id = ?
            ORDER BY case_number DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(target_id.get() as i64)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    pub async fn list_for_moderator(
        pool: &SqlitePool,
        guild_id: GuildId,
        moderator_id: UserId,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"
            SELECT * FROM moderation_cases
            WHERE guild_id = ? AND moderator_id = ?
            ORDER BY case_number DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(moderator_id.get() as i64)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    pub async fn count_for_target(
        pool: &SqlitePool,
        guild_id: GuildId,
        target_id: UserId,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM moderation_cases
            WHERE guild_id = ? AND target_id = ?
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(target_id.get() as i64)
        .fetch_one(pool)
        .await
    }

    pub async fn count_for_target_action(
        pool: &SqlitePool,
        guild_id: GuildId,
        target_id: UserId,
        action: ModerationAction,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM moderation_cases
            WHERE guild_id = ? AND target_id = ? AND action = ?
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(target_id.get() as i64)
        .bind(action.as_str())
        .fetch_one(pool)
        .await
    }

    pub async fn update_reason(
        pool: &SqlitePool,
        guild_id: GuildId,
        case_number: i64,
        reason: Option<&str>,
    ) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE moderation_cases
            SET reason = ?, updated_at = CURRENT_TIMESTAMP
            WHERE guild_id = ? AND case_number = ?
            "#,
        )
        .bind(reason)
        .bind(guild_id.get() as i64)
        .bind(case_number)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        tracing::info!(
            guild_id = %guild_id,
            case_number = case_number,
            "Moderation case reason updated"
        );

        Self::get(pool, guild_id, case_number).await
    }

    pub async fn set_active(
        pool: &SqlitePool,
        guild_id: GuildId,
        case_number: i64,
        active: bool,
    ) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE moderation_cases
            SET active = ?, updated_at = CURRENT_TIMESTAMP
            WHERE guild_id = ? AND case_number = ?
            "#,
        )
        .bind(if active { 1i64 } else { 0i64 })
        .bind(guild_id.get() as i64)
        .bind(case_number)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        Self::get(pool, guild_id, case_number).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::init_database;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestDb {
        pool: SqlitePool,
        path: PathBuf,
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            // Pool may still hold handles; ignore cleanup errors on Windows/macOS.
            let _ = std::fs::remove_file(&self.path);
            let _ = std::fs::remove_file(format!("{}-wal", self.path.display()));
            let _ = std::fs::remove_file(format!("{}-shm", self.path.display()));
        }
    }

    async fn test_db() -> TestDb {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "moderation_case_test_{}_{}_{}.db",
            std::process::id(),
            nanos,
            n
        ));
        let path_str = path.to_string_lossy().to_string();
        let pool = init_database(&path_str)
            .await
            .unwrap_or_else(|e| panic!("init test db {}: {e}", path.display()));
        TestDb { pool, path }
    }

    #[tokio::test]
    async fn create_assigns_monotonic_case_numbers_per_guild() {
        let db = test_db().await;
        let pool = &db.pool;
        let guild = GuildId::new(100);
        let other_guild = GuildId::new(200);
        let target = UserId::new(1);
        let mod_user = UserId::new(2);

        let c1 = ModerationCase::create(
            pool,
            guild,
            ModerationAction::Warn,
            target,
            mod_user,
            Some("first"),
            None,
            None,
        )
        .await
        .unwrap();
        let c2 = ModerationCase::create(
            pool,
            guild,
            ModerationAction::Ban,
            target,
            mod_user,
            Some("second"),
            None,
            None,
        )
        .await
        .unwrap();
        let other = ModerationCase::create(
            pool,
            other_guild,
            ModerationAction::Timeout,
            target,
            mod_user,
            None,
            Some(60),
            None,
        )
        .await
        .unwrap();

        assert_eq!(c1.case_number, 1);
        assert_eq!(c2.case_number, 2);
        assert_eq!(other.case_number, 1);
        assert_eq!(c1.action, "warn");
        assert_eq!(c2.action, "ban");
        assert_eq!(other.duration_seconds, Some(60));
    }

    #[tokio::test]
    async fn get_list_count_and_update_reason() {
        let db = test_db().await;
        let pool = &db.pool;
        let guild = GuildId::new(42);
        let target = UserId::new(7);
        let other_target = UserId::new(8);
        let mod_user = UserId::new(9);

        ModerationCase::create(
            pool,
            guild,
            ModerationAction::Warn,
            target,
            mod_user,
            Some("a"),
            None,
            None,
        )
        .await
        .unwrap();
        ModerationCase::create(
            pool,
            guild,
            ModerationAction::Timeout,
            target,
            mod_user,
            Some("b"),
            Some(120),
            None,
        )
        .await
        .unwrap();
        ModerationCase::create(
            pool,
            guild,
            ModerationAction::Warn,
            other_target,
            mod_user,
            Some("c"),
            None,
            None,
        )
        .await
        .unwrap();

        let got = ModerationCase::get(pool, guild, 2).await.unwrap().unwrap();
        assert_eq!(got.action, "timeout");
        assert_eq!(got.reason.as_deref(), Some("b"));

        let list = ModerationCase::list_for_target(pool, guild, target, 10, 0)
            .await
            .unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].case_number, 2);
        assert_eq!(list[1].case_number, 1);

        assert_eq!(
            ModerationCase::count_for_target(pool, guild, target)
                .await
                .unwrap(),
            2
        );
        assert_eq!(
            ModerationCase::count_for_target_action(pool, guild, target, ModerationAction::Warn)
                .await
                .unwrap(),
            1
        );

        let updated = ModerationCase::update_reason(pool, guild, 1, Some("edited"))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated.reason.as_deref(), Some("edited"));

        let missing = ModerationCase::update_reason(pool, guild, 99, Some("nope"))
            .await
            .unwrap();
        assert!(missing.is_none());

        let mod_list = ModerationCase::list_for_moderator(pool, guild, mod_user, 10, 0)
            .await
            .unwrap();
        assert_eq!(mod_list.len(), 3);

        let deactivated = ModerationCase::set_active(pool, guild, 2, false)
            .await
            .unwrap()
            .unwrap();
        assert!(!deactivated.is_active());
    }

    #[test]
    fn action_parse_roundtrip() {
        for action in [
            ModerationAction::Warn,
            ModerationAction::Ban,
            ModerationAction::Unban,
            ModerationAction::Softban,
            ModerationAction::Timeout,
            ModerationAction::Untimeout,
        ] {
            assert_eq!(ModerationAction::parse(action.as_str()), Some(action));
        }
        assert_eq!(ModerationAction::parse("jail"), None);
    }
}
