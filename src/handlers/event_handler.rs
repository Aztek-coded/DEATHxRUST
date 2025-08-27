use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::handlers::{message, ready};

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        EventHandler
    }
}

#[async_trait]
impl serenity::prelude::EventHandler for EventHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        message::handle_message(ctx, msg).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::handle_ready(ctx, ready).await;
    }
}