use crate::bot::{Context, Error};
use crate::data::models::BoosterRoleLink;
use crate::utils::{EmbedBuilder, EmbedColor};
use poise::serenity_prelude as serenity;
use serenity::all::{Member, Role};
use serenity::prelude::Mentionable;

/// Link an existing role to a booster (Administrator only)
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    description_localized(
        "en-US",
        "Link an existing role to a booster member for management purposes"
    ),
    broadcast_typing
)]
pub async fn link(
    ctx: Context<'_>,
    #[description = "The booster member to link the role to"] member: Member,
    #[description = "The existing role to link to the booster"] role: Role,
) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| Error::Command("This command can only be used in guilds".to_string()))?;

    let admin_id = ctx.author().id;

    tracing::info!(
        admin_id = %admin_id,
        guild_id = %guild_id,
        command = "boosterrole.link",
        user_id = %member.user.id,
        role_id = %role.id,
        "Booster role link command invoked"
    );

    // Defer response to give us more time to process
    ctx.defer().await?;

    // Check if the role is a system role that shouldn't be linked
    if role.managed || role.id.get() == guild_id.get() {
        let embed = EmbedBuilder::error(
            "❌ Invalid Role",
            "Cannot link system roles, managed roles, or the @everyone role.",
        );

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    // Check if the member already has a linked role
    let existing_link = BoosterRoleLink::get(&ctx.data().db_pool, guild_id, member.user.id)
        .await
        .map_err(|e| Error::Database(e))?;

    // Create or update the link
    if let Err(e) = BoosterRoleLink::create(
        &ctx.data().db_pool,
        guild_id,
        member.user.id,
        role.id,
        admin_id,
    )
    .await
    {
        tracing::error!(
            admin_id = %admin_id,
            guild_id = %guild_id,
            user_id = %member.user.id,
            role_id = %role.id,
            error = ?e,
            "Failed to create booster role link"
        );

        let embed = EmbedBuilder::error(
            "❌ Link Creation Failed",
            "Failed to link the role to the booster. Please try again.",
        );

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    // Assign the role to the member if they don't already have it
    if !member.roles.contains(&role.id) {
        if let Err(e) = member.add_role(&ctx.serenity_context().http, role.id).await {
            tracing::warn!(
                admin_id = %admin_id,
                guild_id = %guild_id,
                user_id = %member.user.id,
                role_id = %role.id,
                error = ?e,
                "Failed to assign linked role to member"
            );

            // Don't fail the command if role assignment fails, the link is still created
        }
    }

    // Create success response
    let action_text = if existing_link.is_some() {
        "updated"
    } else {
        "created"
    };

    let embed = serenity::CreateEmbed::new()
        .title("✅ Booster Role Link Created!")
        .description(format!(
            "Successfully {} role link:\n\n**Member:** {}\n**Role:** {}\n**Linked by:** {}",
            action_text,
            member.mention(),
            role.mention(),
            ctx.author().mention()
        ))
        .color(EmbedColor::Success.value())
        .thumbnail(member.avatar_url().unwrap_or_default())
        .footer(serenity::CreateEmbedFooter::new(format!(
            "Linked by {}",
            ctx.author().name
        )))
        .timestamp(serenity::Timestamp::now());

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    tracing::info!(
        admin_id = %admin_id,
        guild_id = %guild_id,
        user_id = %member.user.id,
        role_id = %role.id,
        "Booster role link created successfully"
    );

    Ok(())
}
