use crate::data::models::GuildBoosterAward;
use crate::utils::embed_builder::EmbedBuilder;
use crate::bot::{Context, Error};
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter, Mentionable, Role};

#[poise::command(
    slash_command,
    guild_only,
    subcommands("set", "unset", "view"),
    subcommand_required
)]
pub async fn award(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    slash_command,
    guild_only,
    required_permissions = "MANAGE_GUILD | MANAGE_ROLES",
    default_member_permissions = "MANAGE_GUILD | MANAGE_ROLES"
)]
async fn set(
    ctx: Context<'_>,
    #[description = "Role to award to new boosters"] role: Role,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(Error::Command("This command can only be used in a guild".to_string()))?;

    tracing::info!(
        guild_id = %guild_id,
        admin_id = %ctx.author().id,
        award_role_id = %role.id,
        "Setting booster award role"
    );

    if role.id == guild_id.everyone_role() {
        let embed = EmbedBuilder::error(
            "❌ Invalid Role",
            "Cannot use @everyone as an award role.",
        );

        ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
            .await?;
        return Ok(());
    }

    if role.managed {
        let embed = EmbedBuilder::error(
            "❌ Invalid Role",
            "Cannot use a managed role (bot role, booster role, etc.) as an award role.",
        );

        ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
            .await?;
        return Ok(());
    }

    GuildBoosterAward::set(&ctx.data().db_pool, guild_id, role.id, ctx.author().id).await?;

    let embed = EmbedBuilder::success(
        "✅ Award Role Set",
        &format!(
            "New boosters will now receive the {} role.\n\nThis role will be automatically assigned when someone starts boosting.",
            role.mention()
        ),
    )
    .field("Role Details", &format!("Name: {}\nID: {}", role.name, role.id), false)
    .footer(CreateEmbedFooter::new(format!("Set by {}", ctx.author().mention())));

    ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
        .await?;

    tracing::info!(
        guild_id = %guild_id,
        award_role_id = %role.id,
        set_by = %ctx.author().id,
        "Booster award role configured"
    );

    Ok(())
}

#[poise::command(
    slash_command,
    guild_only,
    required_permissions = "MANAGE_GUILD | MANAGE_ROLES",
    default_member_permissions = "MANAGE_GUILD | MANAGE_ROLES"
)]
async fn unset(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(Error::Command("This command can only be used in a guild".to_string()))?;

    tracing::info!(
        guild_id = %guild_id,
        admin_id = %ctx.author().id,
        "Removing booster award role"
    );

    let current_award = GuildBoosterAward::get(&ctx.data().db_pool, guild_id).await?;

    if current_award.is_none() {
        let embed = EmbedBuilder::info(
            "ℹ️ No Award Role Set",
            "There is no award role configured for this server.",
        );

        ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
            .await?;
        return Ok(());
    }

    let removed = GuildBoosterAward::remove(&ctx.data().db_pool, guild_id).await?;

    if removed {
        let embed = EmbedBuilder::success(
            "✅ Award Role Removed",
            "The booster award role has been removed. New boosters will no longer receive an additional role.",
        )
        .footer(CreateEmbedFooter::new(format!("Removed by {}", ctx.author().mention())));

        ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
            .await?;

        tracing::info!(
            guild_id = %guild_id,
            removed_by = %ctx.author().id,
            "Booster award role removed"
        );
    } else {
        let embed = EmbedBuilder::error(
            "❌ Removal Failed",
            "Failed to remove the award role. Please try again.",
        );

        ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
            .await?;
    }

    Ok(())
}

#[poise::command(slash_command, guild_only)]
async fn view(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(Error::Command("This command can only be used in a guild".to_string()))?;

    tracing::info!(
        guild_id = %guild_id,
        user_id = %ctx.author().id,
        "Viewing booster award role"
    );

    let award_role_id = GuildBoosterAward::get(&ctx.data().db_pool, guild_id).await?;

    match award_role_id {
        Some(role_id) => {
            let guild = guild_id.to_partial_guild(&ctx.serenity_context().http).await?;
            
            if let Some(role) = guild.roles.get(&role_id) {
                let embed = EmbedBuilder::info(
                    "🎁 Booster Award Role",
                    &format!(
                        "New boosters receive: {}\n\nThis role is automatically assigned when someone starts boosting the server.",
                        role.mention()
                    ),
                )
                .field("Role Details", &format!("Name: {}\nID: {}\nColor: #{:06X}", role.name, role.id, role.colour.0), false)
                .footer(CreateEmbedFooter::new("Use /boosterrole award set to change or unset to remove"));

                ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
                    .await?;
            } else {
                let embed = EmbedBuilder::warning(
                    "⚠️ Award Role Missing",
                    &format!(
                        "An award role was configured (ID: {}), but it no longer exists.\n\nUse `/boosterrole award unset` to clear this configuration.",
                        role_id
                    ),
                );

                ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
                    .await?;
            }
        }
        None => {
            let embed = EmbedBuilder::info(
                "ℹ️ No Award Role Set",
                "There is no award role configured for this server.\n\nUse `/boosterrole award set` to configure one.",
            )
            .footer(CreateEmbedFooter::new("Award roles are automatically given to new boosters"));

            ctx.send(poise::CreateReply::default().embed(CreateEmbed::from(embed)))
                .await?;
        }
    }

    Ok(())
}