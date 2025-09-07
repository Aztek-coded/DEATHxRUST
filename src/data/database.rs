use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Database { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        // Migrations are handled inline in init_database for now
        Ok(())
    }
}

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

    tracing::info!("Database initialized successfully");

    Ok(pool)
}
