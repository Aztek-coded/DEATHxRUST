use serenity::model::channel::Message;
use serenity::prelude::*;
use crate::commands;

pub async fn handle_message(ctx: Context, msg: Message) {
    if msg.author.bot {
        return;
    }

    let content = msg.content.trim();
    
    match content {
        "!ping" => {
            commands::ping::execute(&ctx, &msg).await;
        }
        "!help" => {
            commands::help::execute(&ctx, &msg).await;
        }
        _ => {}
    }
}