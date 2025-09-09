use serenity::all::{ChannelId, GuildId, RoleId, UserId};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, FromRow)]
pub struct GuildStaffRole {
    pub id: i64,
    pub guild_id: i64,
    pub role_id: i64,
    pub added_by: i64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl GuildStaffRole {
    pub async fn add(
        pool: &SqlitePool,
        guild_id: GuildId,
        role_id: RoleId,
        added_by: UserId,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!(
            "Database query: add_staff_role for guild {} role {}",
            guild_id,
            role_id
        );

        sqlx::query(
            r#"
            INSERT INTO guild_staff_roles (guild_id, role_id, added_by)
            VALUES (?, ?, ?)
            ON CONFLICT (guild_id, role_id)
            DO UPDATE SET updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(role_id.get() as i64)
        .bind(added_by.get() as i64)
        .execute(pool)
        .await?;

        tracing::info!(
            guild_id = %guild_id,
            role_id = %role_id,
            added_by = %added_by,
            "Staff role added to database"
        );

        Ok(())
    }

    pub async fn remove(
        pool: &SqlitePool,
        guild_id: GuildId,
        role_id: RoleId,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM guild_staff_roles WHERE guild_id = ? AND role_id = ?",
        )
        .bind(guild_id.get() as i64)
        .bind(role_id.get() as i64)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn list(pool: &SqlitePool, guild_id: GuildId) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM guild_staff_roles WHERE guild_id = ? ORDER BY created_at DESC",
        )
        .bind(guild_id.get() as i64)
        .fetch_all(pool)
        .await
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct GuildAutoNickname {
    pub guild_id: i64,
    pub nickname_template: String,
    pub set_by: i64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl GuildAutoNickname {
    pub async fn set(
        pool: &SqlitePool,
        guild_id: GuildId,
        template: &str,
        set_by: UserId,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO guild_auto_nicknames (guild_id, nickname_template, set_by)
            VALUES (?, ?, ?)
            ON CONFLICT (guild_id)
            DO UPDATE SET 
                nickname_template = excluded.nickname_template,
                set_by = excluded.set_by,
                updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(template)
        .bind(set_by.get() as i64)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get(
        pool: &SqlitePool,
        guild_id: GuildId,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM guild_auto_nicknames WHERE guild_id = ?",
        )
        .bind(guild_id.get() as i64)
        .fetch_optional(pool)
        .await
    }

    pub async fn remove(pool: &SqlitePool, guild_id: GuildId) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM guild_auto_nicknames WHERE guild_id = ?")
            .bind(guild_id.get() as i64)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct GuildJoinLogChannel {
    pub guild_id: i64,
    pub channel_id: i64,
    pub set_by: i64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl GuildJoinLogChannel {
    pub async fn set(
        pool: &SqlitePool,
        guild_id: GuildId,
        channel_id: ChannelId,
        set_by: UserId,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO guild_join_log_channels (guild_id, channel_id, set_by)
            VALUES (?, ?, ?)
            ON CONFLICT (guild_id)
            DO UPDATE SET 
                channel_id = excluded.channel_id,
                set_by = excluded.set_by,
                updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(channel_id.get() as i64)
        .bind(set_by.get() as i64)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get(
        pool: &SqlitePool,
        guild_id: GuildId,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM guild_join_log_channels WHERE guild_id = ?",
        )
        .bind(guild_id.get() as i64)
        .fetch_optional(pool)
        .await
    }

    pub async fn remove(pool: &SqlitePool, guild_id: GuildId) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM guild_join_log_channels WHERE guild_id = ?")
            .bind(guild_id.get() as i64)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct GuildPremiumRole {
    pub guild_id: i64,
    pub role_id: i64,
    pub set_by: i64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl GuildPremiumRole {
    pub async fn set(
        pool: &SqlitePool,
        guild_id: GuildId,
        role_id: RoleId,
        set_by: UserId,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO guild_premium_roles (guild_id, role_id, set_by)
            VALUES (?, ?, ?)
            ON CONFLICT (guild_id)
            DO UPDATE SET 
                role_id = excluded.role_id,
                set_by = excluded.set_by,
                updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(role_id.get() as i64)
        .bind(set_by.get() as i64)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get(
        pool: &SqlitePool,
        guild_id: GuildId,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM guild_premium_roles WHERE guild_id = ?")
            .bind(guild_id.get() as i64)
            .fetch_optional(pool)
            .await
    }

    pub async fn remove(pool: &SqlitePool, guild_id: GuildId) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM guild_premium_roles WHERE guild_id = ?")
            .bind(guild_id.get() as i64)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[derive(Debug, Clone)]
pub struct SettingsAuditLog {
    pub guild_id: GuildId,
    pub user_id: UserId,
    pub action: String,
    pub details: Option<String>,
}

impl SettingsAuditLog {
    pub async fn log(
        pool: &SqlitePool,
        guild_id: GuildId,
        user_id: UserId,
        action: &str,
        details: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO settings_audit_log (guild_id, user_id, action, details)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(user_id.get() as i64)
        .bind(action)
        .bind(details)
        .execute(pool)
        .await?;

        tracing::info!(
            guild_id = %guild_id,
            user_id = %user_id,
            action = %action,
            details = ?details,
            "Settings audit log entry created"
        );

        Ok(())
    }
}