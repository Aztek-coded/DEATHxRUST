use crate::config::Settings;
use crate::data::models::GuildPrefix;
use crate::utils::BotError;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Bot application data that will be accessible in all commands
#[derive(Debug, Clone)]
pub struct Data {
    pub settings: Settings,
    pub db_pool: SqlitePool,
    pub prefix_cache: Arc<RwLock<HashMap<u64, String>>>,
}

impl Data {
    pub fn new(settings: Settings, db_pool: SqlitePool) -> Self {
        Self {
            settings,
            db_pool,
            prefix_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_guild_prefix(&self, guild_id: u64) -> Result<Option<String>, Error> {
        let cache = self.prefix_cache.read().await;
        if let Some(prefix) = cache.get(&guild_id) {
            return Ok(Some(prefix.clone()));
        }
        drop(cache);

        let prefix = GuildPrefix::get(&self.db_pool, guild_id).await?;

        if let Some(ref p) = prefix {
            let mut cache = self.prefix_cache.write().await;
            cache.insert(guild_id, p.clone());
        }

        Ok(prefix)
    }

    pub async fn set_guild_prefix(&self, guild_id: u64, prefix: &str) -> Result<(), Error> {
        GuildPrefix::set(&self.db_pool, guild_id, prefix).await?;

        let mut cache = self.prefix_cache.write().await;
        cache.insert(guild_id, prefix.to_string());

        Ok(())
    }

    pub async fn remove_guild_prefix(&self, guild_id: u64) -> Result<bool, Error> {
        let removed = GuildPrefix::remove(&self.db_pool, guild_id).await?;

        if removed {
            let mut cache = self.prefix_cache.write().await;
            cache.remove(&guild_id);
        }

        Ok(removed)
    }
}

/// Custom error type for the bot
#[derive(Debug)]
pub enum Error {
    Serenity(serenity::Error),
    Config(String),
    Command(String),
    Database(sqlx::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Serenity(e) => write!(f, "Serenity error: {}", e),
            Error::Config(e) => write!(f, "Configuration error: {}", e),
            Error::Command(e) => write!(f, "Command error: {}", e),
            Error::Database(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<serenity::Error> for Error {
    fn from(error: serenity::Error) -> Self {
        Self::Serenity(error)
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl From<BotError> for Error {
    fn from(error: BotError) -> Self {
        match error {
            BotError::Config(msg) => Error::Config(msg),
            BotError::Discord(e) => Error::Serenity(e),
            BotError::Io(e) => Error::Command(format!("IO error: {}", e)),
            BotError::Command(msg) => Error::Command(msg),
            BotError::InvalidColor(color) => Error::Command(format!("Invalid color: {}", color)),
            BotError::Other(msg) => Error::Command(msg),
        }
    }
}

/// Type aliases for easier usage throughout the codebase
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;
pub type Framework = poise::Framework<Data, Error>;
