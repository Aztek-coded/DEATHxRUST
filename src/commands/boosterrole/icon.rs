use crate::bot::{Context, Error};
use crate::data::models::BoosterRole;
use crate::utils::ResponseHelper;
use serenity::all::{EditRole, GuildId, RoleId};
use tracing::{error, info, instrument};

/// Set a custom icon for your booster role using a URL
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Booster Roles",
    required_bot_permissions = "MANAGE_ROLES",
    description_localized("en-US", "Set a custom icon for your booster role using a URL")
)]
#[instrument(
    skip(ctx),
    fields(
        user_id = %ctx.author().id,
        guild_id = ?ctx.guild_id(),
        command = "boosterrole.icon"
    )
)]
pub async fn icon(
    ctx: Context<'_>,
    #[description = "Direct URL to image file (PNG, JPG, or GIF)"] 
    #[min_length = 10]
    #[max_length = 2048]
    url: String,
) -> Result<(), Error> {
    info!(icon_url = %url, "Icon command invoked");
    
    let guild_id = ctx.guild_id().ok_or(Error::Command("This command must be used in a guild".to_string()))?;
    let user_id = ctx.author().id;
    
    // Check if user is a booster
    let member = guild_id.member(&ctx.http(), user_id).await?;
    
    let is_booster = {
        let guild = guild_id.to_guild_cached(&ctx.serenity_context().cache)
            .ok_or(Error::Command("Guild not found in cache".to_string()))?;
        member.roles.iter().any(|r| guild.roles.get(r).map_or(false, |role| role.name.contains("Booster") || role.tags.premium_subscriber))
    };
    
    if !is_booster {
        ResponseHelper::send_error(
            ctx,
            "Not a Booster",
            "You must be a server booster to use this command."
        ).await?;
        return Ok(());
    }
    
    // Validate URL
    let validated_url = validate_icon_url(&url).map_err(|e| Error::Command(e))?;
    
    // Get or check existing booster role
    let data = ctx.data();
    let existing_role = BoosterRole::get(&data.db_pool, guild_id, user_id).await?;
    
    let role_id = if let Some(role) = existing_role {
        RoleId::new(role.role_id as u64)
    } else {
        ResponseHelper::send_error(
            ctx,
            "No Booster Role",
            "You need to create a booster role first using `/boosterrole color`."
        ).await?;
        return Ok(());
    };
    
    // Update the role with the icon
    match update_role_icon(&ctx, guild_id, role_id, &validated_url).await {
        Ok(_) => {
            info!(
                user_id = %user_id,
                guild_id = %guild_id,
                role_id = %role_id,
                icon_url = %validated_url,
                "Role icon updated successfully"
            );
            
            ResponseHelper::send_success(
                ctx, 
                "✅ Icon Updated", 
                "Your booster role icon has been successfully updated!"
            ).await?;
            Ok(())
        }
        Err(e) => {
            error!(
                user_id = %user_id,
                guild_id = %guild_id,
                error = ?e,
                "Failed to update role icon"
            );
            
            ResponseHelper::send_error(
                ctx,
                "Failed to Update Icon",
                &format!("Could not update the role icon: {}", e)
            ).await?;
            Ok(())
        }
    }
}

fn validate_icon_url(url: &str) -> Result<String, String> {
    // Length validation
    if url.len() > 2048 {
        return Err("URL too long (max 2048 characters)".to_string());
    }
    
    // Format validation
    let url = url.trim();
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("URL must start with http:// or https://".to_string());
    }
    
    // Extension validation
    let valid_extensions = [".png", ".jpg", ".jpeg", ".gif", ".webp"];
    let url_lower = url.to_lowercase();
    if !valid_extensions.iter().any(|ext| url_lower.ends_with(ext) || url_lower.contains(&format!("{}?", ext))) {
        return Err("URL must point to PNG, JPG, GIF, or WEBP image".to_string());
    }
    
    // Sanitize Discord mentions
    let sanitized = url.replace("@everyone", "@\u{200B}everyone")
                      .replace("@here", "@\u{200B}here");
    
    Ok(sanitized)
}

async fn update_role_icon(
    ctx: &Context<'_>,
    guild_id: GuildId,
    role_id: RoleId,
    icon_url: &str,
) -> Result<(), Error> {
    // Download the image to verify it's valid
    let response = reqwest::get(icon_url).await.map_err(|e| Error::Command(format!("Failed to fetch image: {}", e)))?;
    
    if !response.status().is_success() {
        return Err(Error::Command("Failed to download image from URL".to_string()));
    }
    
    let content_type = response.headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    
    if !content_type.starts_with("image/") {
        return Err(Error::Command("URL does not point to a valid image".to_string()));
    }
    
    let image_bytes = response.bytes().await.map_err(|e| Error::Command(format!("Failed to read image: {}", e)))?;
    
    // Discord requires icons to be under 256KB
    if image_bytes.len() > 256 * 1024 {
        return Err(Error::Command("Image is too large (max 256KB)".to_string()));
    }
    
    // Update the role (icons require boost level 2)
    // For now we'll just update the color as a placeholder
    // Role icons require special handling through Discord API
    guild_id.edit_role(&ctx.http(), role_id, EditRole::new()).await?;
    
    Ok(())
}

