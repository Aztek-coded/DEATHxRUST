mod bot;
mod commands;
mod config;
mod data;
mod handlers;
mod utils;
mod testing;

use bot::create_poise_client;
use config::Settings;
use dotenv::dotenv;
use utils::BotResult;

#[tokio::main]
async fn main() -> BotResult<()> {
    dotenv().ok();

    let settings = Settings::from_env().map_err(|e| utils::BotError::Config(e.to_string()))?;

    println!("Initializing Discord bot...");

    let mut client = create_poise_client(&settings)
        .await
        .map_err(|e| utils::BotError::Config(e.to_string()))?;

    println!("Starting bot client...");
    client.start().await?;

    Ok(())
}
