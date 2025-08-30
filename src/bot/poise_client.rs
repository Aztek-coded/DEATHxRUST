use crate::bot::{framework::create_framework, intents::get_bot_intents};
use crate::config::Settings;
use serenity::{cache::Settings as CacheSettings, Client};

/// Create a Poise-based client
pub async fn create_poise_client(
    settings: &Settings,
) -> Result<Client, Box<dyn std::error::Error + Send + Sync>> {
    let framework = create_framework(settings.clone()).await;
    let intents = get_bot_intents();

    // Configure cache settings
    let mut cache_settings = CacheSettings::default();
    cache_settings.max_messages = 100; // Cache up to 100 messages per channel

    let client = Client::builder(&settings.discord_token, intents)
        .cache_settings(cache_settings)
        .framework(framework)
        .await?;

    Ok(client)
}
