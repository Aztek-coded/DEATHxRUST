use crate::bot::{Context, Error};
use crate::utils::{EmbedBuilder, EmbedColor, ResponseHelper};
use poise::serenity_prelude::CreateEmbed;

/// Check if the bot is responsive and show latency information
#[poise::command(
    slash_command,
    prefix_command,
    aliases("p", "pong", "latency"),
    broadcast_typing
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = std::time::Instant::now();

    // Send initial response as embed
    let initial_embed = EmbedBuilder::info("Ping", "Calculating ping...");
    let reply = ResponseHelper::send_embed(ctx, initial_embed).await?;

    let duration = start.elapsed();
    let ping_ms = duration.as_millis();
    let ws_latency = ctx.ping().await.as_millis();

    // Determine color based on latency
    let color = if ping_ms < 100 {
        EmbedColor::Success
    } else if ping_ms < 300 {
        EmbedColor::Warning
    } else {
        EmbedColor::Error
    };

    // Create detailed response embed
    let response_embed = CreateEmbed::new()
        .title("🏓 Pong!")
        .color(color.value())
        .field("📊 Response Time", format!("{}ms", ping_ms), true)
        .field("🌐 WebSocket Latency", format!("{}ms", ws_latency), true)
        .field(
            "📈 Status",
            match ping_ms {
                0..=99 => "Excellent",
                100..=299 => "Good",
                300..=599 => "Fair",
                _ => "Poor",
            },
            true,
        )
        .footer(poise::serenity_prelude::CreateEmbedFooter::new(format!(
            "Requested by {}",
            ctx.author().name
        )))
        .timestamp(poise::serenity_prelude::Timestamp::now());

    ResponseHelper::edit_to_embed(&reply, ctx, response_embed).await?;

    Ok(())
}
