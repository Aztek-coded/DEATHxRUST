use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;


pub async fn init_database(database_path: &str) -> Result<SqlitePool, sqlx::Error> {
    let database_dir = Path::new(database_path).parent();
    if let Some(dir) = database_dir {
        std::fs::create_dir_all(dir).map_err(|e| sqlx::Error::Io(e))?;
    }

    let database_url = format!("sqlite:{}", database_path);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            database_url
                .parse::<sqlx::sqlite::SqliteConnectOptions>()?
                .create_if_missing(true),
        )
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS guild_prefixes (
            guild_id BIGINT PRIMARY KEY,
            prefix TEXT NOT NULL CHECK(length(prefix) <= 5),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_guild_prefixes_guild_id 
        ON guild_prefixes(guild_id)
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Creating booster_roles table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS booster_roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id BIGINT NOT NULL,
            user_id BIGINT NOT NULL,
            role_id BIGINT NOT NULL,
            role_name TEXT NOT NULL,
            primary_color TEXT NOT NULL,
            secondary_color TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(guild_id, user_id)
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_booster_roles_guild_user 
        ON booster_roles(guild_id, user_id)
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Creating booster_role_links table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS booster_role_links (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id BIGINT NOT NULL,
            user_id BIGINT NOT NULL,
            linked_role_id BIGINT NOT NULL,
            linked_by BIGINT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(guild_id, user_id)
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_booster_role_links_guild_user 
        ON booster_role_links(guild_id, user_id)
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Creating role_name_blacklist table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS role_name_blacklist (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id BIGINT NOT NULL,
            word TEXT NOT NULL,
            added_by BIGINT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(guild_id, word)
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_role_name_blacklist_guild 
        ON role_name_blacklist(guild_id)
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Creating guild_booster_limits table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS guild_booster_limits (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id BIGINT NOT NULL UNIQUE,
            max_roles INTEGER NOT NULL DEFAULT 0,
            set_by BIGINT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_guild_booster_limits_guild 
        ON guild_booster_limits(guild_id)
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Creating guild_booster_awards table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS guild_booster_awards (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id BIGINT NOT NULL UNIQUE,
            award_role_id BIGINT NOT NULL,
            set_by BIGINT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_guild_booster_awards_guild 
        ON guild_booster_awards(guild_id)
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Creating booster_rename_history table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS booster_rename_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id BIGINT NOT NULL,
            user_id BIGINT NOT NULL,
            old_name TEXT NOT NULL,
            new_name TEXT NOT NULL,
            renamed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_rename_user 
        ON booster_rename_history(guild_id, user_id, renamed_at)
        "#,
    )
    .execute(&pool)
    .await?;

    // New tables for boosterrole extensions
    tracing::info!("Creating booster_role_shares table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS booster_role_shares (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id BIGINT NOT NULL,
            role_id BIGINT NOT NULL,
            owner_id BIGINT NOT NULL,
            shared_with_id BIGINT NOT NULL,
            shared_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            expires_at TIMESTAMP NULL,
            is_active BOOLEAN DEFAULT TRUE,
            CONSTRAINT unique_role_share UNIQUE(guild_id, role_id, shared_with_id)
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_role_shares_guild_id 
        ON booster_role_shares(guild_id)
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_role_shares_owner_id 
        ON booster_role_shares(owner_id)
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_role_shares_shared_with 
        ON booster_role_shares(shared_with_id)
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_role_shares_active 
        ON booster_role_shares(is_active) WHERE is_active = TRUE
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Creating guild_sharing_limits table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS guild_sharing_limits (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id BIGINT NOT NULL UNIQUE,
            max_members_per_role INTEGER DEFAULT 5,
            max_shared_roles_per_member INTEGER DEFAULT 3,
            set_by BIGINT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Creating guild_booster_base_roles table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS guild_booster_base_roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id BIGINT NOT NULL UNIQUE,
            base_role_id BIGINT NOT NULL,
            set_by BIGINT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Settings Command Suite Tables
    tracing::info!("Creating guild_staff_roles table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS guild_staff_roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id BIGINT NOT NULL,
            role_id BIGINT NOT NULL,
            added_by BIGINT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(guild_id, role_id)
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_guild_staff_roles 
        ON guild_staff_roles(guild_id)
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Creating guild_auto_nicknames table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS guild_auto_nicknames (
            guild_id BIGINT PRIMARY KEY,
            nickname_template TEXT NOT NULL,
            set_by BIGINT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Creating guild_join_log_channels table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS guild_join_log_channels (
            guild_id BIGINT PRIMARY KEY,
            channel_id BIGINT NOT NULL,
            set_by BIGINT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Creating guild_premium_roles table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS guild_premium_roles (
            guild_id BIGINT PRIMARY KEY,
            role_id BIGINT NOT NULL,
            set_by BIGINT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Creating settings_audit_log table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS settings_audit_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id BIGINT NOT NULL,
            user_id BIGINT NOT NULL,
            action TEXT NOT NULL,
            details TEXT,
            timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_settings_audit_log 
        ON settings_audit_log(guild_id, timestamp)
        "#,
    )
    .execute(&pool)
    .await?;

    tracing::info!("Database initialized successfully");

    Ok(pool)
}
