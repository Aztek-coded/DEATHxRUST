use crate::bot::{Context, Error};
use crate::utils::{EmbedColor, ResponseHelper};
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter, Timestamp};
use serenity::all::{PremiumTier, User};

/// Get information about the server, a user, or the bot
#[poise::command(
    slash_command,
    prefix_command,
    aliases("i", "information", "stats"),
    broadcast_typing
)]
pub async fn info(
    ctx: Context<'_>,
    #[description = "What to get info about (server/user/bot)"] info_type: Option<String>,
    #[description = "User to get info about (only for user type)"] target: Option<User>,
) -> Result<(), Error> {
    let embed = match info_type.as_deref() {
        Some("server") => {
            if let Some(_guild_id) = ctx.guild_id() {
                // Extract all guild data before any await points to avoid Send issues
                let guild_data = ctx.guild().map(|guild| {
                    let channel_count = guild.channels.len();
                    let role_count = guild.roles.len();
                    let boost_tier = match guild.premium_tier {
                        PremiumTier::Tier0 => "0",
                        PremiumTier::Tier1 => "1",
                        PremiumTier::Tier2 => "2",
                        PremiumTier::Tier3 => "3",
                        _ => "Unknown",
                    };
                    let boost_count = guild.premium_subscription_count.unwrap_or(0);
                    let guild_name = guild.name.clone();
                    let guild_id = guild.id;
                    let owner_id = guild.owner_id;
                    let member_count = guild.member_count;
                    let icon_url = guild.icon_url().unwrap_or_default();
                    let verification_level = guild.verification_level;

                    (
                        guild_name,
                        guild_id,
                        owner_id,
                        member_count,
                        channel_count,
                        role_count,
                        boost_tier.to_string(),
                        boost_count,
                        icon_url,
                        verification_level,
                    )
                });

                if let Some((
                    guild_name,
                    guild_id,
                    owner_id,
                    member_count,
                    channel_count,
                    role_count,
                    boost_tier,
                    boost_count,
                    icon_url,
                    verification_level,
                )) = guild_data
                {
                    CreateEmbed::new()
                        .title("🏰 Server Information")
                        .color(EmbedColor::Primary.value())
                        .thumbnail(icon_url)
                        .field("📛 Name", guild_name, true)
                        .field("🆔 Server ID", guild_id.to_string(), true)
                        .field("👑 Owner", format!("<@{}>", owner_id), true)
                        .field("👥 Members", member_count.to_string(), true)
                        .field("💬 Channels", channel_count.to_string(), true)
                        .field("🎭 Roles", role_count.to_string(), true)
                        .field("🚀 Boost Level", format!("Level {}", boost_tier), true)
                        .field("✨ Boosts", boost_count.to_string(), true)
                        .field(
                            "📅 Created",
                            format!("<t:{}:R>", guild_id.created_at().unix_timestamp()),
                            true,
                        )
                        .field(
                            "🔐 Verification Level",
                            format!("{:?}", verification_level),
                            true,
                        )
                        .footer(CreateEmbedFooter::new(format!(
                            "Requested by {}",
                            ctx.author().name
                        )))
                        .timestamp(Timestamp::now())
                } else {
                    return ResponseHelper::send_error(
                        ctx,
                        "Cache Error",
                        "Server information is not available in cache.",
                    )
                    .await
                    .map(|_| ());
                }
            } else {
                return ResponseHelper::send_error(
                    ctx,
                    "Server Only",
                    "This command can only be used in a server.",
                )
                .await
                .map(|_| ());
            }
        }
        Some("user") => {
            let user = target.as_ref().unwrap_or_else(|| ctx.author());

            let mut embed = CreateEmbed::new()
                .title("👤 User Information")
                .color(if user.bot {
                    EmbedColor::Secondary.value()
                } else {
                    EmbedColor::Info.value()
                })
                .thumbnail(
                    user.avatar_url()
                        .unwrap_or_else(|| user.default_avatar_url()),
                )
                .field("📛 Username", user.tag(), true)
                .field("🆔 User ID", user.id.to_string(), true)
                .field("🤖 Bot", if user.bot { "Yes" } else { "No" }, true)
                .field(
                    "📅 Account Created",
                    format!("<t:{}:R>", user.id.created_at().unix_timestamp()),
                    true,
                );

            if let Some(banner_url) = user.banner_url() {
                embed = embed.image(banner_url);
            }

            embed
                .footer(CreateEmbedFooter::new(format!(
                    "Requested by {}",
                    ctx.author().name
                )))
                .timestamp(Timestamp::now())
        }
        Some("bot") => {
            // Extract cache data before any await points
            let cache = ctx.cache();
            let current_user = cache.current_user();
            let bot_name = current_user.tag();
            let bot_id = current_user.id;
            let avatar_url = current_user
                .avatar_url()
                .unwrap_or_else(|| current_user.default_avatar_url());
            let guild_count = cache.guilds().len();
            let _ = current_user; // Drop the cache reference explicitly
            let _ = cache; // Drop the cache reference explicitly

            CreateEmbed::new()
                .title("🤖 Bot Information")
                .color(EmbedColor::Primary.value())
                .thumbnail(avatar_url)
                .field("📛 Bot Name", bot_name, true)
                .field("🆔 Bot ID", bot_id.to_string(), true)
                .field("🏠 Servers", guild_count.to_string(), true)
                .field(
                    "📅 Created",
                    format!("<t:{}:R>", bot_id.created_at().unix_timestamp()),
                    true,
                )
                .field("⚙️ Framework", "Poise + Serenity", true)
                .field("🦀 Language", "Rust", true)
                .footer(CreateEmbedFooter::new(format!(
                    "Requested by {}",
                    ctx.author().name
                )))
                .timestamp(Timestamp::now())
        }
        _ => {
            // Get actual prefix for this guild
            let actual_prefix = if let Some(guild_id) = ctx.guild_id() {
                ctx.data()
                    .get_guild_prefix(guild_id.get())
                    .await?
                    .unwrap_or_else(|| ctx.data().settings.command_prefix.clone())
            } else {
                ctx.data().settings.command_prefix.clone()
            };

            CreateEmbed::new()
                .title("ℹ️ Information Command")
                .color(EmbedColor::Info.value())
                .description("Get detailed information about servers, users, or the bot.")
                .field(
                    "📝 Usage",
                    format!(
                        "`/info [type] [target]`\n`{}info [type] [target]`",
                        actual_prefix
                    ),
                    false,
                )
                .field(
                    "📋 Available Types",
                    "• `server` - Display server information\n\
                     • `user` - Display user information\n\
                     • `bot` - Display bot information",
                    false,
                )
                .field(
                    "💡 Examples",
                    format!(
                        "`/info server` - Show current server info\n\
                     `{}info user @username` - Show user info\n\
                     `{}info bot` - Show bot statistics",
                        actual_prefix, actual_prefix
                    ),
                    false,
                )
                .footer(CreateEmbedFooter::new(
                    "Use without parameters to see this help",
                ))
                .timestamp(Timestamp::now())
        }
    };

    ResponseHelper::send_embed(ctx, embed).await?;
    Ok(())
}
