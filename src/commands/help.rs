use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn execute(ctx: &Context, msg: &Message) {
    let help_message = "📋 **Available Commands**\n\n\
        `!ping` - Test bot responsiveness\n\
        `!help` - Show this help message";
    
    if let Err(why) = msg.channel_id.say(&ctx.http, help_message).await {
        eprintln!("Error sending help response: {why:?}");
    }
}