use crate::bot::{Context, Error};
use serenity::all::User;

/// Get information about the server, a user, or the bot
#[poise::command(slash_command, prefix_command)]
pub async fn info(
    ctx: Context<'_>,
    #[description = "What to get info about (server/user/bot)"] info_type: Option<String>,
    #[description = "User to get info about (only for user type)"] target: Option<User>,
) -> Result<(), Error> {
    let response = match info_type.as_deref() {
        Some("server") => {
            if let Some(guild_id) = ctx.guild_id() {
                if let Some(guild) = ctx.guild() {
                    format!(
                        "🏰 **Server Information**\n\
                        **Name:** {}\n\
                        **ID:** {}\n\
                        **Owner:** <@{}>\n\
                        **Members:** {}\n\
                        **Created:** <t:{}:R>",
                        guild.name,
                        guild.id,
                        guild.owner_id,
                        guild.member_count,
                        guild.id.created_at().unix_timestamp()
                    )
                } else {
                    // Fallback when guild data isn't cached
                    format!(
                        "🏰 **Server Information**\n\
                        **ID:** {}\n\
                        **Created:** <t:{}:R>\n\
                        *(Some information unavailable - guild not cached)*",
                        guild_id,
                        guild_id.created_at().unix_timestamp()
                    )
                }
            } else {
                "❌ This command can only be used in a server.".to_string()
            }
        }
        Some("user") => {
            let user = target.as_ref().unwrap_or_else(|| ctx.author());
            format!(
                "👤 **User Information**\n\
                **Name:** {}\n\
                **ID:** {}\n\
                **Created:** <t:{}:R>\n\
                **Bot:** {}",
                user.tag(),
                user.id,
                user.id.created_at().unix_timestamp(),
                if user.bot { "Yes" } else { "No" }
            )
        }
        Some("bot") => {
            let current_user = ctx.cache().current_user();
            format!(
                "🤖 **Bot Information**\n\
                **Name:** {}\n\
                **ID:** {}\n\
                **Created:** <t:{}:R>\n\
                **Servers:** {}",
                current_user.tag(),
                current_user.id,
                current_user.id.created_at().unix_timestamp(),
                ctx.cache().guilds().len()
            )
        }
        _ => {
            "ℹ️ **Information Command**\n\n\
            **Usage:** `/info [type] [target]`\n\
            **Types:** `server`, `user`, `bot`\n\n\
            Use without parameters to see this help message.".to_string()
        }
    };

    ctx.say(response).await?;
    Ok(())
}