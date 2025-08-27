use serenity::prelude::*;
use crate::config::Settings;
use crate::handlers::EventHandler;
use crate::bot::intents::get_bot_intents;
use crate::utils::BotResult;

pub async fn create_client(settings: &Settings) -> BotResult<Client> {
    let intents = get_bot_intents();
    
    let client = Client::builder(&settings.discord_token, intents)
        .event_handler(EventHandler::new())
        .await?;
    
    Ok(client)
}