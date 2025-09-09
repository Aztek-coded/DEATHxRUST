use serenity::all::{GuildId, RoleId, UserId};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, FromRow)]
pub struct GuildPrefix {
    #[allow(dead_code)]
    pub guild_id: i64,
    pub prefix: String,
    #[allow(dead_code)]
    pub created_at: Option<String>,
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
    pub guild_id: i64,
    pub user_id: i64,
    pub role_id: i64,
    pub role_name: String,
    pub primary_color: String,
    pub secondary_color: Option<String>,
    pub created_at: Option<String>,
    #[allow(dead_code)]
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

    pub async fn get_all_for_guild(
        pool: &SqlitePool,
        guild_id: GuildId,
    ) -> Result<Vec<Self>, sqlx::Error> {
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

    pub async fn update_color(
        pool: &SqlitePool,
        guild_id: GuildId,
        user_id: UserId,
        primary_color: &str,
        secondary_color: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!(
            "Database query: update_booster_role_color for user {} in guild {}",
            user_id,
            guild_id
        );

        sqlx::query(
            r#"
            UPDATE booster_roles 
            SET primary_color = ?, secondary_color = ?, updated_at = CURRENT_TIMESTAMP
            WHERE guild_id = ? AND user_id = ?
            "#,
        )
        .bind(primary_color)
        .bind(secondary_color)
        .bind(guild_id.get() as i64)
        .bind(user_id.get() as i64)
        .execute(pool)
        .await?;

        tracing::info!(
            user_id = %user_id,
            guild_id = %guild_id,
            primary_color = %primary_color,
            "Booster role color updated"
        );

        Ok(())
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct BoosterRoleLink {
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
    pub guild_id: i64,
    #[allow(dead_code)]
    pub user_id: i64,
    #[allow(dead_code)]
    pub linked_role_id: i64,
    #[allow(dead_code)]
    pub linked_by: i64,
    #[allow(dead_code)]
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

        let result =
            sqlx::query("DELETE FROM booster_role_links WHERE guild_id = ? AND user_id = ?")
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
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
    pub guild_id: i64,
    #[allow(dead_code)]
    pub word: String,
    #[allow(dead_code)]
    pub added_by: i64,
    #[allow(dead_code)]
    pub created_at: Option<String>,
}

impl RoleNameBlacklist {
    pub async fn get_all_for_guild(
        pool: &SqlitePool,
        guild_id: GuildId,
    ) -> Result<Vec<String>, sqlx::Error> {
        tracing::debug!("Database query: get_blacklist for guild {}", guild_id);

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

#[derive(Debug, Clone, FromRow)]
pub struct GuildBoosterLimit {
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
    pub guild_id: i64,
    #[allow(dead_code)]
    pub max_roles: i32,
    #[allow(dead_code)]
    pub set_by: i64,
    #[allow(dead_code)]
    pub created_at: Option<String>,
    #[allow(dead_code)]
    pub updated_at: Option<String>,
}

impl GuildBoosterLimit {
    pub async fn get(pool: &SqlitePool, guild_id: GuildId) -> Result<Option<i32>, sqlx::Error> {
        tracing::debug!("Database query: get_booster_limit for guild {}", guild_id);

        let result = sqlx::query_scalar::<_, i32>(
            "SELECT max_roles FROM guild_booster_limits WHERE guild_id = ?",
        )
        .bind(guild_id.get() as i64)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn set(
        pool: &SqlitePool,
        guild_id: GuildId,
        max_roles: i32,
        set_by: UserId,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!(
            "Database query: set_guild_booster_limit for guild {} to {}",
            guild_id,
            max_roles
        );

        sqlx::query(
            r#"
            INSERT INTO guild_booster_limits (guild_id, max_roles, set_by)
            VALUES (?, ?, ?)
            ON CONFLICT (guild_id)
            DO UPDATE SET 
                max_roles = excluded.max_roles,
                set_by = excluded.set_by,
                updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(max_roles)
        .bind(set_by.get() as i64)
        .execute(pool)
        .await?;

        tracing::info!(
            guild_id = %guild_id,
            max_roles = max_roles,
            set_by = %set_by,
            "Guild booster limit set"
        );

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn remove(pool: &SqlitePool, guild_id: GuildId) -> Result<bool, sqlx::Error> {
        tracing::debug!(
            "Database query: remove_guild_booster_limit for guild {}",
            guild_id
        );

        let result = sqlx::query("DELETE FROM guild_booster_limits WHERE guild_id = ?")
            .bind(guild_id.get() as i64)
            .execute(pool)
            .await?;

        let removed = result.rows_affected() > 0;

        if removed {
            tracing::info!(guild_id = %guild_id, "Guild booster limit removed");
        }

        Ok(removed)
    }

    pub async fn check_limit(
        pool: &SqlitePool,
        guild_id: GuildId,
    ) -> Result<(bool, Option<i32>), sqlx::Error> {
        let limit = Self::get(pool, guild_id).await?;

        if let Some(max) = limit {
            if max == 0 {
                return Ok((false, Some(0)));
            }

            let current_count = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM booster_roles WHERE guild_id = ?",
            )
            .bind(guild_id.get() as i64)
            .fetch_one(pool)
            .await?;

            Ok((current_count < max as i64, Some(max)))
        } else {
            Ok((true, None))
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct GuildBoosterAward {
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
    pub guild_id: i64,
    #[allow(dead_code)]
    pub award_role_id: i64,
    #[allow(dead_code)]
    pub set_by: i64,
    #[allow(dead_code)]
    pub created_at: Option<String>,
    #[allow(dead_code)]
    pub updated_at: Option<String>,
}

impl GuildBoosterAward {
    pub async fn get(pool: &SqlitePool, guild_id: GuildId) -> Result<Option<RoleId>, sqlx::Error> {
        tracing::debug!("Database query: get_booster_award for guild {}", guild_id);

        let result = sqlx::query_scalar::<_, i64>(
            "SELECT award_role_id FROM guild_booster_awards WHERE guild_id = ?",
        )
        .bind(guild_id.get() as i64)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|id| RoleId::new(id as u64)))
    }

    pub async fn set(
        pool: &SqlitePool,
        guild_id: GuildId,
        award_role_id: RoleId,
        set_by: UserId,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!(
            "Database query: set_guild_booster_award for guild {} to role {}",
            guild_id,
            award_role_id
        );

        sqlx::query(
            r#"
            INSERT INTO guild_booster_awards (guild_id, award_role_id, set_by)
            VALUES (?, ?, ?)
            ON CONFLICT (guild_id)
            DO UPDATE SET 
                award_role_id = excluded.award_role_id,
                set_by = excluded.set_by,
                updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(award_role_id.get() as i64)
        .bind(set_by.get() as i64)
        .execute(pool)
        .await?;

        tracing::info!(
            guild_id = %guild_id,
            award_role_id = %award_role_id,
            set_by = %set_by,
            "Guild booster award role set"
        );

        Ok(())
    }

    pub async fn remove(pool: &SqlitePool, guild_id: GuildId) -> Result<bool, sqlx::Error> {
        tracing::debug!(
            "Database query: remove_guild_booster_award for guild {}",
            guild_id
        );

        let result = sqlx::query("DELETE FROM guild_booster_awards WHERE guild_id = ?")
            .bind(guild_id.get() as i64)
            .execute(pool)
            .await?;

        let removed = result.rows_affected() > 0;

        if removed {
            tracing::info!(guild_id = %guild_id, "Guild booster award removed");
        }

        Ok(removed)
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct BoosterRenameHistory {
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
    pub guild_id: i64,
    #[allow(dead_code)]
    pub user_id: i64,
    pub old_name: String,
    pub new_name: String,
    pub renamed_at: String,
}

impl BoosterRenameHistory {
    pub async fn add(
        pool: &SqlitePool,
        guild_id: GuildId,
        user_id: UserId,
        old_name: &str,
        new_name: &str,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!(
            "Database query: add_rename_history for user {} in guild {}",
            user_id,
            guild_id
        );

        sqlx::query(
            r#"
            INSERT INTO booster_rename_history (guild_id, user_id, old_name, new_name)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(user_id.get() as i64)
        .bind(old_name)
        .bind(new_name)
        .execute(pool)
        .await?;

        tracing::info!(
            user_id = %user_id,
            guild_id = %guild_id,
            old_name = %old_name,
            new_name = %new_name,
            "Rename history recorded"
        );

        Ok(())
    }

    pub async fn get_last_rename(
        pool: &SqlitePool,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<Option<Self>, sqlx::Error> {
        tracing::debug!(
            "Database query: get_last_rename for user {} in guild {}",
            user_id,
            guild_id
        );

        let result = sqlx::query_as::<_, BoosterRenameHistory>(
            r#"
            SELECT * FROM booster_rename_history 
            WHERE guild_id = ? AND user_id = ?
            ORDER BY renamed_at DESC
            LIMIT 1
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(user_id.get() as i64)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn check_rate_limit(
        pool: &SqlitePool,
        guild_id: GuildId,
        user_id: UserId,
        cooldown_minutes: i64,
    ) -> Result<bool, sqlx::Error> {
        tracing::debug!(
            "Database query: check_rename_rate_limit for user {} in guild {}",
            user_id,
            guild_id
        );

        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM booster_rename_history 
            WHERE guild_id = ? AND user_id = ?
            AND renamed_at > datetime('now', ? || ' minutes')
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(user_id.get() as i64)
        .bind(format!("-{}", cooldown_minutes))
        .fetch_one(pool)
        .await?;

        Ok(count == 0)
    }
}

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct BoosterRoleShare {
    #[allow(dead_code)]
    pub id: i64,
    pub guild_id: i64,
    pub role_id: i64,
    pub owner_id: i64,
    pub shared_with_id: i64,
    pub shared_at: Option<String>,
    pub expires_at: Option<String>,
    pub is_active: bool,
}

impl BoosterRoleShare {
    pub async fn create(
        pool: &SqlitePool,
        guild_id: GuildId,
        role_id: RoleId,
        owner_id: UserId,
        shared_with_id: UserId,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!(
            "Database query: create_role_share for role {} shared with user {}",
            role_id,
            shared_with_id
        );

        sqlx::query(
            r#"
            INSERT INTO booster_role_shares (guild_id, role_id, owner_id, shared_with_id)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(role_id.get() as i64)
        .bind(owner_id.get() as i64)
        .bind(shared_with_id.get() as i64)
        .execute(pool)
        .await?;

        tracing::info!(
            guild_id = %guild_id,
            role_id = %role_id,
            owner_id = %owner_id,
            shared_with_id = %shared_with_id,
            "Role share created"
        );

        Ok(())
    }

    pub async fn remove(
        pool: &SqlitePool,
        guild_id: GuildId,
        role_id: RoleId,
        shared_with_id: UserId,
    ) -> Result<bool, sqlx::Error> {
        tracing::debug!(
            "Database query: remove_role_share for role {} and user {}",
            role_id,
            shared_with_id
        );

        let result = sqlx::query(
            r#"
            UPDATE booster_role_shares 
            SET is_active = FALSE
            WHERE guild_id = ? AND role_id = ? AND shared_with_id = ? AND is_active = TRUE
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(role_id.get() as i64)
        .bind(shared_with_id.get() as i64)
        .execute(pool)
        .await?;

        let removed = result.rows_affected() > 0;

        if removed {
            tracing::info!(
                guild_id = %guild_id,
                role_id = %role_id,
                shared_with_id = %shared_with_id,
                "Role share removed"
            );
        }

        Ok(removed)
    }

    pub async fn get_shared_with_user(
        pool: &SqlitePool,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<Vec<Self>, sqlx::Error> {
        tracing::debug!(
            "Database query: get_shared_roles for user {} in guild {}",
            user_id,
            guild_id
        );

        let shares = sqlx::query_as::<_, BoosterRoleShare>(
            r#"
            SELECT * FROM booster_role_shares 
            WHERE guild_id = ? AND shared_with_id = ? AND is_active = TRUE
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(user_id.get() as i64)
        .fetch_all(pool)
        .await?;

        Ok(shares)
    }

    pub async fn get_role_shares(
        pool: &SqlitePool,
        guild_id: GuildId,
        role_id: RoleId,
    ) -> Result<Vec<Self>, sqlx::Error> {
        tracing::debug!(
            "Database query: get_role_shares for role {} in guild {}",
            role_id,
            guild_id
        );

        let shares = sqlx::query_as::<_, BoosterRoleShare>(
            r#"
            SELECT * FROM booster_role_shares 
            WHERE guild_id = ? AND role_id = ? AND is_active = TRUE
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(role_id.get() as i64)
        .fetch_all(pool)
        .await?;

        Ok(shares)
    }

    pub async fn count_role_shares(
        pool: &SqlitePool,
        guild_id: GuildId,
        role_id: RoleId,
    ) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM booster_role_shares 
            WHERE guild_id = ? AND role_id = ? AND is_active = TRUE
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(role_id.get() as i64)
        .fetch_one(pool)
        .await?;

        Ok(count)
    }

    pub async fn count_user_shares(
        pool: &SqlitePool,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM booster_role_shares 
            WHERE guild_id = ? AND shared_with_id = ? AND is_active = TRUE
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(user_id.get() as i64)
        .fetch_one(pool)
        .await?;

        Ok(count)
    }
}

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct GuildSharingLimit {
    #[allow(dead_code)]
    pub id: i64,
    pub guild_id: i64,
    pub max_members_per_role: i32,
    pub max_shared_roles_per_member: i32,
    #[allow(dead_code)]
    pub set_by: i64,
    #[allow(dead_code)]
    pub created_at: Option<String>,
    #[allow(dead_code)]
    pub updated_at: Option<String>,
}

impl GuildSharingLimit {
    pub async fn get(pool: &SqlitePool, guild_id: GuildId) -> Result<Option<Self>, sqlx::Error> {
        tracing::debug!("Database query: get_sharing_limits for guild {}", guild_id);

        let result = sqlx::query_as::<_, GuildSharingLimit>(
            "SELECT * FROM guild_sharing_limits WHERE guild_id = ?",
        )
        .bind(guild_id.get() as i64)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn set(
        pool: &SqlitePool,
        guild_id: GuildId,
        max_members_per_role: i32,
        max_shared_roles_per_member: i32,
        set_by: UserId,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!(
            "Database query: set_sharing_limits for guild {}",
            guild_id
        );

        sqlx::query(
            r#"
            INSERT INTO guild_sharing_limits (guild_id, max_members_per_role, max_shared_roles_per_member, set_by)
            VALUES (?, ?, ?, ?)
            ON CONFLICT (guild_id)
            DO UPDATE SET 
                max_members_per_role = excluded.max_members_per_role,
                max_shared_roles_per_member = excluded.max_shared_roles_per_member,
                set_by = excluded.set_by,
                updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(max_members_per_role)
        .bind(max_shared_roles_per_member)
        .bind(set_by.get() as i64)
        .execute(pool)
        .await?;

        tracing::info!(
            guild_id = %guild_id,
            max_members_per_role = %max_members_per_role,
            max_shared_roles_per_member = %max_shared_roles_per_member,
            set_by = %set_by,
            "Guild sharing limits set"
        );

        Ok(())
    }
}

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct GuildBoosterBaseRole {
    #[allow(dead_code)]
    pub id: i64,
    pub guild_id: i64,
    pub base_role_id: i64,
    #[allow(dead_code)]
    pub set_by: i64,
    #[allow(dead_code)]
    pub created_at: Option<String>,
    #[allow(dead_code)]
    pub updated_at: Option<String>,
}

impl GuildBoosterBaseRole {
    pub async fn get(pool: &SqlitePool, guild_id: GuildId) -> Result<Option<RoleId>, sqlx::Error> {
        tracing::debug!("Database query: get_base_role for guild {}", guild_id);

        let result = sqlx::query_scalar::<_, i64>(
            "SELECT base_role_id FROM guild_booster_base_roles WHERE guild_id = ?",
        )
        .bind(guild_id.get() as i64)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|id| RoleId::new(id as u64)))
    }

    pub async fn set(
        pool: &SqlitePool,
        guild_id: GuildId,
        base_role_id: RoleId,
        set_by: UserId,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!(
            "Database query: set_base_role for guild {} to role {}",
            guild_id,
            base_role_id
        );

        sqlx::query(
            r#"
            INSERT INTO guild_booster_base_roles (guild_id, base_role_id, set_by)
            VALUES (?, ?, ?)
            ON CONFLICT (guild_id)
            DO UPDATE SET 
                base_role_id = excluded.base_role_id,
                set_by = excluded.set_by,
                updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(guild_id.get() as i64)
        .bind(base_role_id.get() as i64)
        .bind(set_by.get() as i64)
        .execute(pool)
        .await?;

        tracing::info!(
            guild_id = %guild_id,
            base_role_id = %base_role_id,
            set_by = %set_by,
            "Guild booster base role set"
        );

        Ok(())
    }

    pub async fn remove(pool: &SqlitePool, guild_id: GuildId) -> Result<bool, sqlx::Error> {
        tracing::debug!(
            "Database query: remove_base_role for guild {}",
            guild_id
        );

        let result = sqlx::query("DELETE FROM guild_booster_base_roles WHERE guild_id = ?")
            .bind(guild_id.get() as i64)
            .execute(pool)
            .await?;

        let removed = result.rows_affected() > 0;

        if removed {
            tracing::info!(guild_id = %guild_id, "Guild booster base role removed");
        }

        Ok(removed)
    }
}
