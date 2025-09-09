use crate::bot::{Context, Error};
use crate::data::models::{
    GuildAutoNickname, GuildJoinLogChannel, GuildPremiumRole, GuildStaffRole,
};
use crate::utils::EmbedColor;
use serenity::all::{CreateEmbed, Timestamp};
use tokio::join;

#[poise::command(slash_command, prefix_command)]
pub async fn config(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in guild")?;
    let pool = &ctx.data().db_pool;

    let (staff_roles, auto_nick, join_log, premium_role) = join!(
        GuildStaffRole::list(pool, guild_id),
        GuildAutoNickname::get(pool, guild_id),
        GuildJoinLogChannel::get(pool, guild_id),
        GuildPremiumRole::get(pool, guild_id)
    );

    let staff_display = match staff_roles {
        Ok(roles) if !roles.is_empty() => roles
            .iter()
            .map(|r| format!("<@&{}>", r.role_id))
            .collect::<Vec<_>>()
            .join(", "),
        _ => "None configured".to_string(),
    };

    let auto_nick_display = match auto_nick {
        Ok(Some(an)) => format!("`{}`", an.nickname_template),
        _ => "Disabled".to_string(),
    };

    let join_log_display = match join_log {
        Ok(Some(jl)) => format!("<#{}>", jl.channel_id),
        _ => "Disabled".to_string(),
    };

    let premium_role_display = match premium_role {
        Ok(Some(pr)) => format!("<@&{}>", pr.role_id),
        _ => "None configured".to_string(),
    };

    let embed = CreateEmbed::new()
        .title("⚙️ Current Guild Settings")
        .color(EmbedColor::Primary.value())
        .field("Staff Roles", staff_display, false)
        .field("Auto-Nickname Template", auto_nick_display, false)
        .field("Join/Leave Logs", join_log_display, false)
        .field("Premium Role", premium_role_display, false)
        .timestamp(Timestamp::now());

    ctx.send(poise::CreateReply::default().embed(embed))
        .await?;
    Ok(())
}