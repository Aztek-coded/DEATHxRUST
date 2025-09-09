use crate::bot::{Context, Error};
use crate::data::models::{BoosterRole, BoosterRoleLink};
use crate::utils::{ColorGenerator, ResponseHelper};
use serenity::all::{EditRole, Permissions, RoleId};
use tracing::{info, instrument};

/// Generate a random color for your booster role
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Booster Roles",
    required_bot_permissions = "MANAGE_ROLES",
    description_localized("en-US", "Generate a random color for your booster role"),
    aliases("rand", "randomize")
)]
#[instrument(
    skip(ctx),
    fields(
        user_id = %ctx.author().id,
        guild_id = ?ctx.guild_id(),
        command = "boosterrole.random"
    )
)]
pub async fn random(
    ctx: Context<'_>,
    #[description = "Color style: vibrant, pastel, dark, or palette"] 
    style: Option<String>,
) -> Result<(), Error> {
    info!("Random color command invoked");
    
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
    
    // Check if user has a linked role (admin-assigned)
    let data = ctx.data();
    if let Some(_linked_role) = BoosterRoleLink::get(&data.db_pool, guild_id, user_id).await? {
        ResponseHelper::send_error(
            ctx,
            "Role is Linked",
            "Your booster role is managed by an administrator and cannot be modified."
        ).await?;
        return Ok(());
    }
    
    // Generate random color based on style
    let color = match style.as_deref() {
        Some("pastel") => ColorGenerator::generate_pastel(),
        Some("dark") => ColorGenerator::generate_dark(),
        Some("palette") => ColorGenerator::generate_from_palette(),
        _ => ColorGenerator::generate_random(), // Default to vibrant
    };
    
    let hex_color = ColorGenerator::to_hex_string(color);
    
    // Get or create booster role
    let existing_role = BoosterRole::get(&data.db_pool, guild_id, user_id).await?;
    
    let (role_id, role_name) = if let Some(role) = existing_role {
        // Update existing role
        let role_id = RoleId::new(role.role_id as u64);
        
        // Update role color
        guild_id.edit_role(&ctx.http(), role_id, EditRole::new().colour(color.0 as u64)).await?;
        
        // Update database
        BoosterRole::update_color(&data.db_pool, guild_id, user_id, &hex_color, None).await?;
        
        (role_id, role.role_name)
    } else {
        // Create new role with random color
        let default_name = format!("{}'s Booster Role", member.user.name);
        
        // Create the role
        let new_role = guild_id.create_role(
            &ctx.http(),
            serenity::all::EditRole::new()
                .name(&default_name)
                .colour(color.0 as u64)
                .permissions(Permissions::empty())
                .mentionable(false)
                .hoist(false)
        ).await?;
        
        // Add role to member
        member.add_role(&ctx.http(), new_role.id).await?;
        
        // Position the role above base role if configured
        if let Some(base_role_id) = crate::data::models::GuildBoosterBaseRole::get(&data.db_pool, guild_id).await? {
            let base_position = {
                let guild = guild_id.to_guild_cached(&ctx.serenity_context().cache)
                    .ok_or(Error::Command("Guild not found in cache".to_string()))?;
                guild.roles.get(&base_role_id).map(|r| r.position)
            };
            
            if let Some(pos) = base_position {
                let new_position = pos + 1;
                guild_id.edit_role(
                    &ctx.http(),
                    new_role.id,
                    EditRole::new().position(new_position as u16)
                ).await?;
            }
        }
        
        // Store in database
        BoosterRole::create(
            &data.db_pool,
            guild_id,
            user_id,
            new_role.id,
            &default_name,
            &hex_color,
            None,
        ).await?;
        
        (new_role.id, default_name)
    };
    
    info!(
        user_id = %user_id,
        guild_id = %guild_id,
        role_id = %role_id,
        color = %hex_color,
        "Random color applied to booster role"
    );
    
    // Send success response with color preview
    let description = format!(
        "Your booster role **{}** has been updated with a random color!\n\n\
        **New Color:** {}\n\
        **Style:** {}",
        role_name,
        hex_color,
        style.as_deref().unwrap_or("vibrant")
    );
    
    let embed = crate::utils::EmbedBuilder::success(
        "🎲 Random Color Applied",
        &description
    );
    
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    
    Ok(())
}