use crate::bot::{Context, Error};

/// Check if the bot is responsive and show latency information
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = std::time::Instant::now();
    
    // Send initial response
    let reply = ctx.say("🏓 Calculating ping...").await?;
    
    let duration = start.elapsed();
    let ping_ms = duration.as_millis();
    
    // Edit the response with actual ping
    let response_content = format!(
        "🏓 Pong!\n\
        📊 **Response Time:** {}ms\n\
        🌐 **WebSocket Latency:** {}ms",
        ping_ms,
        ctx.ping().await.as_millis()
    );

    reply.edit(ctx, poise::CreateReply::default().content(response_content)).await?;

    Ok(())
}