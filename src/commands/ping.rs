use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn execute(ctx: &Context, msg: &Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong! 🏓").await {
        eprintln!("Error sending ping response: {why:?}");
    }
}