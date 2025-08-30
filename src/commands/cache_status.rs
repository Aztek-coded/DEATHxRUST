use crate::bot::{Context, Error};
use crate::utils::{EmbedColor, ResponseHelper};
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter, Timestamp};

/// Display cache statistics and information
#[poise::command(slash_command, prefix_command, aliases("cache", "c", "status"))]
pub async fn cache_status(ctx: Context<'_>) -> Result<(), Error> {
    // Extract all cache data before any await points to avoid Send issues
    let cache_data = {
        let cache = ctx.cache();

        let guild_count = cache.guilds().len();
        let user_count = cache.user_count();
        let channel_count = cache.guild_channel_count();
        let cached_guilds: Vec<String> = cache
            .guilds()
            .iter()
            .filter_map(|guild_id| {
                cache
                    .guild(*guild_id)
                    .map(|guild| format!("{} ({})", guild.name, guild.id))
            })
            .take(5) // Show first 5 guilds to avoid spam
            .collect();

        (guild_count, user_count, channel_count, cached_guilds)
    };

    let (guild_count, user_count, channel_count, cached_guilds) = cache_data;

    let guilds_display = if cached_guilds.is_empty() {
        "No guilds cached".to_string()
    } else if guild_count > 5 {
        format!(
            "{}\n... and {} more",
            cached_guilds.join("\n"),
            guild_count - 5
        )
    } else {
        cached_guilds.join("\n")
    };

    let embed = CreateEmbed::new()
        .title("📊 Cache Status")
        .color(EmbedColor::Info.value())
        .field("🏰 Cached Guilds", guild_count.to_string(), true)
        .field("👥 Cached Users", user_count.to_string(), true)
        .field("💬 Cached Channels", channel_count.to_string(), true)
        .field("📋 Guild Details", guilds_display, false)
        .footer(CreateEmbedFooter::new(format!(
            "Requested by {}",
            ctx.author().name
        )))
        .timestamp(Timestamp::now());

    ResponseHelper::send_embed(ctx, embed).await?;
    Ok(())
}
