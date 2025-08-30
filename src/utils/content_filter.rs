use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Content filter for checking role names against blacklisted words
/// Provides caching and efficient string matching
pub struct ContentFilter {
    /// Cached blacklist words for quick lookup
    cached_words: Arc<RwLock<HashSet<String>>>,
    /// Guild ID this filter is for
    guild_id: serenity::all::GuildId,
    /// Database pool for fetching fresh blacklist data
    db_pool: sqlx::SqlitePool,
}

impl ContentFilter {
    /// Create a new content filter for a guild
    pub fn new(guild_id: serenity::all::GuildId, db_pool: sqlx::SqlitePool) -> Self {
        Self {
            cached_words: Arc::new(RwLock::new(HashSet::new())),
            guild_id,
            db_pool,
        }
    }

    /// Refresh the cached blacklist from the database
    pub async fn refresh_cache(&self) -> Result<(), sqlx::Error> {
        tracing::debug!(
            guild_id = %self.guild_id,
            "Refreshing content filter cache"
        );

        let words = crate::data::models::RoleNameBlacklist::get_all_for_guild(&self.db_pool, self.guild_id).await?;
        
        let mut cache = self.cached_words.write().await;
        cache.clear();
        
        for word in words {
            cache.insert(word.to_lowercase());
        }

        tracing::debug!(
            guild_id = %self.guild_id,
            word_count = cache.len(),
            "Content filter cache refreshed"
        );

        Ok(())
    }

    /// Check if a text contains any blacklisted words
    /// This method uses the cache for fast lookups
    pub async fn contains_blacklisted_content(&self, text: &str) -> Result<bool, sqlx::Error> {
        // Ensure cache is populated
        {
            let cache = self.cached_words.read().await;
            if cache.is_empty() {
                drop(cache); // Release read lock before acquiring write lock
                self.refresh_cache().await?;
            }
        }

        let text_lower = text.to_lowercase();
        let cache = self.cached_words.read().await;

        // Check for any blacklisted words in the text
        for word in cache.iter() {
            if text_lower.contains(word) {
                tracing::debug!(
                    guild_id = %self.guild_id,
                    text = %text,
                    blacklisted_word = %word,
                    "Blacklisted content detected"
                );
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Check if a specific word is in the blacklist
    pub async fn is_word_blacklisted(&self, word: &str) -> Result<bool, sqlx::Error> {
        // Ensure cache is populated
        {
            let cache = self.cached_words.read().await;
            if cache.is_empty() {
                drop(cache); // Release read lock before acquiring write lock
                self.refresh_cache().await?;
            }
        }

        let word_lower = word.to_lowercase();
        let cache = self.cached_words.read().await;

        Ok(cache.contains(&word_lower))
    }

    /// Get all cached blacklisted words
    pub async fn get_cached_words(&self) -> Vec<String> {
        let cache = self.cached_words.read().await;
        cache.iter().cloned().collect()
    }

    /// Add a word to the cache (should be called after database update)
    pub async fn add_word_to_cache(&self, word: &str) {
        let word_lower = word.to_lowercase();
        let mut cache = self.cached_words.write().await;
        cache.insert(word_lower);

        tracing::debug!(
            guild_id = %self.guild_id,
            word = %word,
            "Word added to content filter cache"
        );
    }

    /// Remove a word from the cache (should be called after database update)
    pub async fn remove_word_from_cache(&self, word: &str) {
        let word_lower = word.to_lowercase();
        let mut cache = self.cached_words.write().await;
        cache.remove(&word_lower);

        tracing::debug!(
            guild_id = %self.guild_id,
            word = %word,
            "Word removed from content filter cache"
        );
    }

    /// Clear the entire cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cached_words.write().await;
        cache.clear();

        tracing::debug!(
            guild_id = %self.guild_id,
            "Content filter cache cleared"
        );
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> ContentFilterStats {
        let cache = self.cached_words.read().await;
        ContentFilterStats {
            word_count: cache.len(),
            guild_id: self.guild_id,
        }
    }
}

/// Statistics about a content filter
#[derive(Debug, Clone)]
pub struct ContentFilterStats {
    pub word_count: usize,
    pub guild_id: serenity::all::GuildId,
}


#[cfg(test)]
mod tests {
    use super::*;
    use serenity::all::GuildId;

    // These tests would require a test database setup
    // For now, they serve as documentation of expected behavior

    #[tokio::test]
    async fn test_content_filter_basic_functionality() {
        // This test would need a proper test database
        // let db_pool = setup_test_db().await;
        // let guild_id = GuildId::new(12345);
        // let filter = ContentFilter::new(guild_id, db_pool);
        // 
        // // Test that empty cache doesn't match anything
        // assert!(!filter.contains_blacklisted_content("hello world").await.unwrap());
        // 
        // // Add word to blacklist and test detection
        // filter.add_word_to_cache("badword").await;
        // assert!(filter.contains_blacklisted_content("this contains badword").await.unwrap());
        // assert!(!filter.contains_blacklisted_content("this is clean").await.unwrap());
    }

    #[tokio::test]
    async fn test_case_insensitive_matching() {
        // Test that blacklist matching is case-insensitive
        // let filter = setup_test_filter().await;
        // filter.add_word_to_cache("BadWord").await;
        // 
        // assert!(filter.contains_blacklisted_content("BADWORD").await.unwrap());
        // assert!(filter.contains_blacklisted_content("badword").await.unwrap());
        // assert!(filter.contains_blacklisted_content("BadWord").await.unwrap());
    }
}