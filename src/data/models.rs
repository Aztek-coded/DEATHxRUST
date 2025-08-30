use serenity::all::{GuildId, RoleId, UserId};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, FromRow)]
pub struct GuildPrefix {
    pub guild_id: i64,
    pub prefix: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl GuildPrefix {
    pub async fn get(pool: &SqlitePool, guild_id: u64) -> Result<Option<String>, sqlx::Error> {
        let result =
            sqlx::query_as::<_, GuildPrefix>("SELECT * FROM guild_prefixes WHERE guild_id = ?")
                .bind(guild_id as i64)
                .fetch_optional(pool)
                .await?;

        Ok(result.map(|gp| gp.prefix))
    }

    pub async fn set(pool: &SqlitePool, guild_id: u64, prefix: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO guild_prefixes (guild_id, prefix)
            VALUES (?, ?)
            ON CONFLICT (guild_id)
            DO UPDATE SET prefix = excluded.prefix,
                          updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(guild_id as i64)
        .bind(prefix)
        .execute(pool)
        .await?;

        tracing::info!(
            guild_id = %guild_id,
            new_prefix = %prefix,
            "Guild prefix updated"
        );

        Ok(())
    }

    pub async fn remove(pool: &SqlitePool, guild_id: u64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM guild_prefixes WHERE guild_id = ?")
            .bind(guild_id as i64)
            .execute(pool)
            .await?;

        let removed = result.rows_affected() > 0;

        if removed {
            tracing::info!(
                guild_id = %guild_id,
                "Guild prefix removed"
            );
        }

        Ok(removed)
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct BoosterRole {
    pub id: i64,
    pub guild_id: i64,
    pub user_id: i64,
    pub role_id: i64,
    pub role_name: String,
    pub primary_color: String,
    pub secondary_color: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl BoosterRole {
    pub async fn get(
        pool: &SqlitePool,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<Option<Self>, sqlx::Error> {
        tracing::debug!(
            "Database query: get_booster_role for user {} in guild {}",
            user_id,
            guild_id
        );

        let result = sqlx::query_as::<_, BoosterRole>(
            "SELECT * FROM booster_roles WHERE guild_id = ? AND user_id = ?",
        )
        .bind(guild_id.get() as i64)
        .bind(user_id.get() as i64)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn create(
        pool: &SqlitePool,
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
        role_name: &str,
        primary_color: &str,
        secondary_color: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!(
            "Database query: create_booster_role for user {} in guild {} with role {}",
            user_id,
            guild_id,
            role_id
        );

        sqlx::query(
            r#"
            INSERT INTO booster_roles (guild_id, user_id, role_id, role_name, primary_color, secondary_color)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT (guild_id, user_id)
            DO UPDATE SET 
                role_id = excluded.role_id,
                role_name = excluded.role_name,
                primary_color = excluded.primary_color,
                secondary_color = excluded.secondary_color,
                updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(user_id.get() as i64)
        .bind(role_id.get() as i64)
        .bind(role_name)
        .bind(primary_color)
        .bind(secondary_color)
        .execute(pool)
        .await?;

        tracing::info!(
            user_id = %user_id,
            guild_id = %guild_id,
            role_id = %role_id,
            role_name = %role_name,
            "Booster role database record created/updated"
        );

        Ok(())
    }

    pub async fn update(
        pool: &SqlitePool,
        guild_id: GuildId,
        user_id: UserId,
        role_name: &str,
        primary_color: &str,
        secondary_color: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!(
            "Database query: update_booster_role for user {} in guild {}",
            user_id,
            guild_id
        );

        sqlx::query(
            r#"
            UPDATE booster_roles 
            SET role_name = ?, primary_color = ?, secondary_color = ?, updated_at = CURRENT_TIMESTAMP
            WHERE guild_id = ? AND user_id = ?
            "#,
        )
        .bind(role_name)
        .bind(primary_color)
        .bind(secondary_color)
        .bind(guild_id.get() as i64)
        .bind(user_id.get() as i64)
        .execute(pool)
        .await?;

        tracing::info!(
            user_id = %user_id,
            guild_id = %guild_id,
            role_name = %role_name,
            "Booster role database record updated"
        );

        Ok(())
    }

    pub async fn delete(
        pool: &SqlitePool,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<bool, sqlx::Error> {
        tracing::debug!(
            "Database query: delete_booster_role for user {} in guild {}",
            user_id,
            guild_id
        );

        let result = sqlx::query("DELETE FROM booster_roles WHERE guild_id = ? AND user_id = ?")
            .bind(guild_id.get() as i64)
            .bind(user_id.get() as i64)
            .execute(pool)
            .await?;

        let deleted = result.rows_affected() > 0;

        if deleted {
            tracing::info!(
                user_id = %user_id,
                guild_id = %guild_id,
                "Booster role database record deleted"
            );
        }

        Ok(deleted)
    }

    pub async fn get_all_for_guild(pool: &SqlitePool, guild_id: GuildId) -> Result<Vec<Self>, sqlx::Error> {
        tracing::debug!(
            "Database query: get_all_booster_roles for guild {}",
            guild_id
        );

        let results = sqlx::query_as::<_, BoosterRole>(
            "SELECT * FROM booster_roles WHERE guild_id = ? ORDER BY created_at DESC",
        )
        .bind(guild_id.get() as i64)
        .fetch_all(pool)
        .await?;

        Ok(results)
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct BoosterRoleLink {
    pub id: i64,
    pub guild_id: i64,
    pub user_id: i64,
    pub linked_role_id: i64,
    pub linked_by: i64,
    pub created_at: Option<String>,
}

impl BoosterRoleLink {
    pub async fn get(
        pool: &SqlitePool,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<Option<Self>, sqlx::Error> {
        tracing::debug!(
            "Database query: get_booster_role_link for user {} in guild {}",
            user_id,
            guild_id
        );

        let result = sqlx::query_as::<_, BoosterRoleLink>(
            "SELECT * FROM booster_role_links WHERE guild_id = ? AND user_id = ?",
        )
        .bind(guild_id.get() as i64)
        .bind(user_id.get() as i64)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn create(
        pool: &SqlitePool,
        guild_id: GuildId,
        user_id: UserId,
        linked_role_id: RoleId,
        linked_by: UserId,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!(
            "Database query: create_booster_role_link for user {} in guild {} with role {}",
            user_id,
            guild_id,
            linked_role_id
        );

        sqlx::query(
            r#"
            INSERT INTO booster_role_links (guild_id, user_id, linked_role_id, linked_by)
            VALUES (?, ?, ?, ?)
            ON CONFLICT (guild_id, user_id)
            DO UPDATE SET 
                linked_role_id = excluded.linked_role_id,
                linked_by = excluded.linked_by,
                created_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(user_id.get() as i64)
        .bind(linked_role_id.get() as i64)
        .bind(linked_by.get() as i64)
        .execute(pool)
        .await?;

        tracing::info!(
            user_id = %user_id,
            guild_id = %guild_id,
            linked_role_id = %linked_role_id,
            linked_by = %linked_by,
            "Booster role link created/updated"
        );

        Ok(())
    }

    pub async fn delete(
        pool: &SqlitePool,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<bool, sqlx::Error> {
        tracing::debug!(
            "Database query: delete_booster_role_link for user {} in guild {}",
            user_id,
            guild_id
        );

        let result = sqlx::query("DELETE FROM booster_role_links WHERE guild_id = ? AND user_id = ?")
            .bind(guild_id.get() as i64)
            .bind(user_id.get() as i64)
            .execute(pool)
            .await?;

        let deleted = result.rows_affected() > 0;

        if deleted {
            tracing::info!(
                user_id = %user_id,
                guild_id = %guild_id,
                "Booster role link deleted"
            );
        }

        Ok(deleted)
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct RoleNameBlacklist {
    pub id: i64,
    pub guild_id: i64,
    pub word: String,
    pub added_by: i64,
    pub created_at: Option<String>,
}

impl RoleNameBlacklist {
    pub async fn get_all_for_guild(pool: &SqlitePool, guild_id: GuildId) -> Result<Vec<String>, sqlx::Error> {
        tracing::debug!(
            "Database query: get_blacklist for guild {}",
            guild_id
        );

        let results = sqlx::query_scalar::<_, String>(
            "SELECT word FROM role_name_blacklist WHERE guild_id = ? ORDER BY word ASC",
        )
        .bind(guild_id.get() as i64)
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn add_word(
        pool: &SqlitePool,
        guild_id: GuildId,
        word: &str,
        added_by: UserId,
    ) -> Result<bool, sqlx::Error> {
        tracing::debug!(
            "Database query: add_blacklist_word '{}' for guild {}",
            word,
            guild_id
        );

        let word_lower = word.to_lowercase();

        let result = sqlx::query(
            r#"
            INSERT INTO role_name_blacklist (guild_id, word, added_by)
            VALUES (?, ?, ?)
            ON CONFLICT (guild_id, word) DO NOTHING
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(&word_lower)
        .bind(added_by.get() as i64)
        .execute(pool)
        .await?;

        let added = result.rows_affected() > 0;

        if added {
            tracing::info!(
                guild_id = %guild_id,
                word = %word_lower,
                added_by = %added_by,
                "Blacklist word added"
            );
        }

        Ok(added)
    }

    pub async fn remove_word(
        pool: &SqlitePool,
        guild_id: GuildId,
        word: &str,
    ) -> Result<bool, sqlx::Error> {
        tracing::debug!(
            "Database query: remove_blacklist_word '{}' for guild {}",
            word,
            guild_id
        );

        let word_lower = word.to_lowercase();

        let result = sqlx::query("DELETE FROM role_name_blacklist WHERE guild_id = ? AND word = ?")
            .bind(guild_id.get() as i64)
            .bind(&word_lower)
            .execute(pool)
            .await?;

        let removed = result.rows_affected() > 0;

        if removed {
            tracing::info!(
                guild_id = %guild_id,
                word = %word_lower,
                "Blacklist word removed"
            );
        }

        Ok(removed)
    }

    pub async fn is_blacklisted(
        pool: &SqlitePool,
        guild_id: GuildId,
        text: &str,
    ) -> Result<bool, sqlx::Error> {
        let blacklist = Self::get_all_for_guild(pool, guild_id).await?;
        let text_lower = text.to_lowercase();
        
        for word in &blacklist {
            if text_lower.contains(&word.to_lowercase()) {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
}
