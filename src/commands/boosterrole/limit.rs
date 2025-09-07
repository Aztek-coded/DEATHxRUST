use crate::data::models::{BoosterRole, GuildBoosterLimit};
use crate::utils::embed_builder::EmbedBuilder;
use crate::bot::{Context, Error};
use poise::serenity_prelude::{CreateEmbed, Mentionable};

#[poise::command(
    slash_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn limit(
    ctx: Context<'_>,
    #[description = "Maximum number of booster roles (0 = unlimited)"]
    #[min = 0]
    #[max = 100]
    max_roles: Option<i32>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(Error::Command("This command can only be used in a guild".to_string()))?;

    tracing::info!(
        guild_id = %guild_id,
        admin_id = %ctx.author().id,
        max_roles = ?max_roles,
        "Boosterrole limit command invoked"
    );

    if let Some(limit) = max_roles {
        GuildBoosterLimit::set(&ctx.data().db_pool, guild_id, limit, ctx.author().id).await?;

        let current_count = BoosterRole::get_all_for_guild(&ctx.data().db_pool, guild_id)
            .await?
            .len();

        let description = if limit == 0 {
            "Booster roles are now **unlimited** for this server.".to_string()
        } else {
            format!(
                "Maximum booster roles set to **{}**.\nCurrent booster roles: **{}**",
                limit, current_count
            )
        };

        let mut embed = EmbedBuilder::success("✅ Limit Updated", &description);

        if limit > 0 && current_count > limit as usize {
            embed = embed.field(
                "⚠️ Warning",
                format!(
                    "There are currently {} booster roles, which exceeds the new limit of {}.\nExisting roles will remain, but no new roles can be created until below the limit.",
                    current_count, limit
                ),
                false,
            );
        }

        embed = embed.footer(poise::serenity_prelude::CreateEmbedFooter::new(format!("Set by {}", ctx.author().mention())));

        ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
            .await?;
    } else {
        let current_limit = GuildBoosterLimit::get(&ctx.data().db_pool, guild_id).await?;
        let current_count = BoosterRole::get_all_for_guild(&ctx.data().db_pool, guild_id)
            .await?
            .len();

        let (description, status_text) = match current_limit {
            Some(0) | None => (
                format!(
                    "Booster roles are currently **unlimited**.\nActive booster roles: **{}**",
                    current_count
                ),
                "No limit set".to_string(),
            ),
            Some(limit) => (
                format!(
                    "Maximum booster roles: **{}**\nActive booster roles: **{}**\nAvailable slots: **{}**",
                    limit,
                    current_count,
                    if current_count >= limit as usize {
                        0
                    } else {
                        limit as usize - current_count
                    }
                ),
                format!("Limit: {}", limit),
            ),
        };

        let embed = EmbedBuilder::info("📊 Booster Role Limit", &description)
            .field("Status", &status_text, true)
            .footer(poise::serenity_prelude::CreateEmbedFooter::new("Use /boosterrole limit <number> to set a new limit"));

        ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
            .await?;
    }

    Ok(())
}