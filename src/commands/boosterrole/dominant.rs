use crate::bot::{Context, Error};
use crate::utils::EmbedBuilder;
use poise::serenity_prelude::{self as serenity, Colour, CreateEmbed, EditRole, Member};
use tracing::{debug, error, info, warn};

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Booster",
    description_localized("en-US", "Set your booster role color to your avatar's dominant color")
)]
pub async fn dominant(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(Error::Command("Not in a guild".to_string()))?;
    let member = ctx.author_member().await.ok_or(Error::Command("Not in a guild".to_string()))?;
    
    info!("Boosterrole dominant command invoked by user {} in guild {}", 
          ctx.author().id, guild_id);
    
    if !is_booster(&member) {
        warn!("Non-booster {} attempted dominant color command", ctx.author().id);
        let embed = EmbedBuilder::error(
            "❌ Booster Required",
            "This command is only available to server boosters."
        );
        ctx.send(poise::CreateReply::default().embed(embed).ephemeral(true))
            .await?;
        return Ok(());
    }
    
    debug!("User {} confirmed as booster", ctx.author().id);
    
    ctx.defer().await?;
    
    let avatar_url = ctx.author().avatar_url().ok_or_else(|| {
        Error::Command("You need to have an avatar set to use this command".to_string())
    })?;
    
    debug!("Fetching avatar for user {}: {}", ctx.author().id, avatar_url);
    
    let dominant_color = match extract_dominant_color(&avatar_url).await {
        Ok(color) => {
            debug!("Dominant color extracted: #{:06X}", color);
            color
        }
        Err(e) => {
            error!("Avatar processing failed for user {}: {}", ctx.author().id, e);
            let embed = EmbedBuilder::error(
                "❌ Processing Failed",
                &format!("Failed to process your avatar: {}", e)
            );
            ctx.send(poise::CreateReply::default().embed(embed))
                .await?;
            return Ok(());
        }
    };
    
    let booster_role = find_or_create_booster_role(ctx, &member).await?;
    
    let color = Colour::from(dominant_color);
    
    match guild_id.edit_role(&ctx.http(), booster_role, EditRole::new().colour(color)).await {
        Ok(_) => {
            info!("Updated booster role color for user {} to #{:06X}", 
                  ctx.author().id, dominant_color);
            
            let embed = create_success_embed(dominant_color, color);
            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        }
        Err(e) => {
            error!("Failed to update role for user {}: {}", ctx.author().id, e);
            let embed = EmbedBuilder::error(
                "❌ Update Failed",
                &format!("Failed to update your role color: {}", e)
            );
            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        }
    }
    
    Ok(())
}

fn is_booster(member: &Member) -> bool {
    member.premium_since.is_some()
}

async fn find_or_create_booster_role(
    ctx: Context<'_>,
    member: &Member,
) -> Result<serenity::RoleId, Error> {
    use sqlx::Row;
    
    let guild_id = ctx.guild_id().ok_or(Error::Command("Not in a guild".to_string()))?;
    let user_id = member.user.id;
    
    let pool = &ctx.data().db_pool;
    
    let existing = sqlx::query(
        "SELECT role_id FROM booster_roles WHERE guild_id = ? AND user_id = ?"
    )
    .bind(guild_id.get() as i64)
    .bind(user_id.get() as i64)
    .fetch_optional(pool)
    .await?;
    
    if let Some(row) = existing {
        let role_id: i64 = row.get("role_id");
        return Ok(serenity::RoleId::new(role_id as u64));
    }
    
    let role_name = format!("{}'s Booster Role", member.user.name);
    let new_role = guild_id
        .create_role(&ctx.http(), EditRole::new()
            .name(&role_name)
            .mentionable(false)
            .hoist(false))
        .await?;
    
    sqlx::query(
        "INSERT INTO booster_roles (guild_id, user_id, role_id) VALUES (?, ?, ?)"
    )
    .bind(guild_id.get() as i64)
    .bind(user_id.get() as i64)
    .bind(new_role.id.get() as i64)
    .execute(pool)
    .await?;
    
    member.add_role(&ctx.http(), new_role.id).await?;
    
    Ok(new_role.id)
}

async fn extract_dominant_color(avatar_url: &str) -> Result<u32, Error> {
    use crate::utils::image_processor;
    
    let image_data = image_processor::fetch_avatar(avatar_url).await?;
    let color = image_processor::extract_dominant_color(&image_data)?;
    
    Ok(color)
}

fn create_success_embed(hex_color: u32, discord_color: Colour) -> CreateEmbed {
    let r = (hex_color >> 16) & 0xFF;
    let g = (hex_color >> 8) & 0xFF;
    let b = hex_color & 0xFF;
    
    CreateEmbed::default()
        .title("✅ Color Updated")
        .description(format!(
            "Your booster role color has been set to your avatar's dominant color!\n\n\
            **Hex:** #{:06X}\n\
            **RGB:** ({}, {}, {})",
            hex_color, r, g, b
        ))
        .color(discord_color)
        .thumbnail("https://via.placeholder.com/150/".to_string() + &format!("{:06X}", hex_color))
}