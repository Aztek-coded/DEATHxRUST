mod bot;
mod commands;
mod config;
mod handlers;
mod utils;

use dotenv::dotenv;
use config::Settings;
use bot::create_poise_client;
use utils::BotResult;

#[tokio::main]
async fn main() -> BotResult<()> {
    dotenv().ok();
    
    let settings = Settings::from_env()
        .map_err(|e| utils::BotError::Config(e.to_string()))?;
    
    println!("Initializing Discord bot...");
    
    let mut client = create_poise_client(&settings).await
        .map_err(|e| utils::BotError::Config(e.to_string()))?;
    
    println!("Starting bot client...");
    client.start().await?;
    
    Ok(())
}