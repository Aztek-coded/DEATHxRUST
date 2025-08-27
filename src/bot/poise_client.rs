use crate::config::Settings;
use crate::bot::{framework::create_framework, intents::get_bot_intents};
use serenity::Client;

/// Create a Poise-based client
pub async fn create_poise_client(settings: &Settings) -> Result<Client, Box<dyn std::error::Error + Send + Sync>> {
    let framework = create_framework(settings.clone()).await;
    let intents = get_bot_intents();
    
    let client = Client::builder(&settings.discord_token, intents)
        .framework(framework)
        .await?;
    
    Ok(client)
}