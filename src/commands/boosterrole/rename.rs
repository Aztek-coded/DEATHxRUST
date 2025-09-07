use crate::data::models::{BoosterRole, BoosterRenameHistory, RoleNameBlacklist};
use crate::utils::embed_builder::EmbedBuilder;
use crate::bot::{Context, Error};
use chrono::{DateTime, Duration, Utc};
use poise::serenity_prelude::{CreateEmbed, EditRole, RoleId};

const RENAME_COOLDOWN_MINUTES: i64 = 60;

#[poise::command(slash_command, guild_only)]
pub async fn rename(
    ctx: Context<'_>,
    #[description = "New name for your booster role"]
    #[min_length = 1]
    #[max_length = 100]
    new_name: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(Error::Command("This command can only be used in a guild".to_string()))?;
    let user_id = ctx.author().id;

    tracing::info!(
        guild_id = %guild_id,
        user_id = %user_id,
        new_name = %new_name,
        "Boosterrole rename command invoked"
    );

    let member = guild_id
        .member(&ctx.serenity_context().http, user_id)
        .await?;

    if member.premium_since.is_none() {
        let embed = EmbedBuilder::error(
            "❌ Not a Booster",
            "You must be actively boosting this server to rename your booster role.",
        );

        ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
            .await?;
        return Ok(());
    }

    let role_record = BoosterRole::get(&ctx.data().db_pool, guild_id, user_id).await?;

    let role_record = match role_record {
        Some(r) => r,
        None => {
            let embed = EmbedBuilder::error(
                "❌ No Booster Role",
                "You don't have a booster role yet. Use `/boosterrole color` to create one first.",
            );

            ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
                .await?;
            return Ok(());
        }
    };

    let can_rename = BoosterRenameHistory::check_rate_limit(
        &ctx.data().db_pool,
        guild_id,
        user_id,
        RENAME_COOLDOWN_MINUTES,
    )
    .await?;

    if !can_rename {
        let last_rename =
            BoosterRenameHistory::get_last_rename(&ctx.data().db_pool, guild_id, user_id).await?;

        if let Some(last) = last_rename {
            if let Ok(last_time) = DateTime::parse_from_rfc3339(&last.renamed_at) {
                let last_utc: DateTime<Utc> = last_time.with_timezone(&Utc);
                let cooldown_end = last_utc + Duration::minutes(RENAME_COOLDOWN_MINUTES);
                let now = Utc::now();

                if cooldown_end > now {
                    let remaining = cooldown_end - now;
                    let minutes_left = remaining.num_minutes();

                    let embed = EmbedBuilder::error(
                        "⏱️ Cooldown Active",
                        &format!(
                            "You can rename your role again in **{} minutes**.\n\nLast rename: {} → {}",
                            minutes_left, last.old_name, last.new_name
                        ),
                    );

                    tracing::warn!(
                        user_id = %user_id,
                        cooldown_remaining = ?remaining,
                        "Rename rate limit hit"
                    );

                    ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
                        .await?;
                    return Ok(());
                }
            }
        }
    }

    if RoleNameBlacklist::is_blacklisted(&ctx.data().db_pool, guild_id, &new_name).await? {
        let embed = EmbedBuilder::error(
            "🚫 Name Not Allowed",
            "This name contains blacklisted words and cannot be used.",
        );

        ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
            .await?;
        return Ok(());
    }

    let role_id = RoleId::new(role_record.role_id as u64);
    let old_name = role_record.role_name.clone();

    if let Err(e) = guild_id
        .edit_role(&ctx.serenity_context().http, role_id, EditRole::new().name(&new_name))
        .await
    {
        tracing::error!(
            "Failed to rename role {} in guild {}: {}",
            role_id,
            guild_id,
            e
        );

        let embed = EmbedBuilder::error(
            "❌ Rename Failed",
            "Failed to rename the role. Please try again later.",
        );

        ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
            .await?;
        return Ok(());
    }

    BoosterRole::update(
        &ctx.data().db_pool,
        guild_id,
        user_id,
        &new_name,
        &role_record.primary_color,
        role_record.secondary_color.as_deref(),
    )
    .await?;

    BoosterRenameHistory::add(
        &ctx.data().db_pool,
        guild_id,
        user_id,
        &old_name,
        &new_name,
    )
    .await?;

    let embed = EmbedBuilder::success(
        "✅ Role Renamed",
        &format!("Your booster role has been renamed from **{}** to **{}**.", old_name, new_name),
    )
    .footer(poise::serenity_prelude::CreateEmbedFooter::new(format!(
        "You can rename again in {} minutes",
        RENAME_COOLDOWN_MINUTES
    )));

    ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
        .await?;

    tracing::info!(
        user_id = %user_id,
        guild_id = %guild_id,
        old_name = %old_name,
        new_name = %new_name,
        "Booster role renamed successfully"
    );

    Ok(())
}