use crate::bot::Error;
use serenity::all::{GuildId, UserId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};

pub struct SettingsRateLimiter {
    limits: Arc<RwLock<HashMap<(GuildId, UserId), Instant>>>,
}

impl SettingsRateLimiter {
    pub fn new() -> Self {
        Self {
            limits: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn check_limit(&self, guild_id: GuildId, user_id: UserId) -> Result<(), Error> {
        let mut limits = self.limits.write().await;
        let key = (guild_id, user_id);

        if let Some(last_use) = limits.get(&key) {
            if last_use.elapsed() < Duration::from_secs(60) {
                let remaining = 60 - last_use.elapsed().as_secs();
                return Err(format!(
                    "Please wait {} seconds between settings changes",
                    remaining
                )
                .into());
            }
        }

        limits.insert(key, Instant::now());
        Ok(())
    }

    pub async fn clear(&self) {
        let mut limits = self.limits.write().await;
        limits.clear();
    }
}

impl Default for SettingsRateLimiter {
    fn default() -> Self {
        Self::new()
    }
}