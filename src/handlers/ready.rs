use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub async fn handle_ready(_ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
}