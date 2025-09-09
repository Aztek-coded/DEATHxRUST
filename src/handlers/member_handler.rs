use crate::data::models::{GuildAutoNickname, GuildJoinLogChannel};
use crate::utils::EmbedColor;
use serenity::model::mention::Mentionable;
use serenity::all::{
    ChannelId, Context, CreateEmbed, CreateMessage, EditMember, GuildId, Member, User,
};
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct MemberHandler {
    pub db_pool: Arc<SqlitePool>,
}

impl MemberHandler {
    pub fn new(db_pool: Arc<SqlitePool>) -> Self {
        Self { db_pool }
    }

    pub async fn handle_member_join(&self, ctx: &Context, new_member: &Member) {
        let guild_id = new_member.guild_id;
        let user_id = new_member.user.id;

        // Apply auto-nickname if configured
        if let Err(e) = self.apply_auto_nickname(ctx, new_member).await {
            tracing::error!(
                guild_id = %guild_id,
                user_id = %user_id,
                error = ?e,
                "Failed to apply auto-nickname"
            );
        }

        // Send join log if configured
        if let Err(e) = self.send_join_log(ctx, new_member).await {
            tracing::error!(
                guild_id = %guild_id,
                user_id = %user_id,
                error = ?e,
                "Failed to send join log"
            );
        }
    }

    pub async fn handle_member_leave(&self, ctx: &Context, guild_id: GuildId, user: &User) {
        // Send leave log if configured
        if let Err(e) = self.send_leave_log(ctx, guild_id, user).await {
            tracing::error!(
                guild_id = %guild_id,
                user_id = %user.id,
                error = ?e,
                "Failed to send leave log"
            );
        }
    }

    async fn apply_auto_nickname(
        &self,
        ctx: &Context,
        member: &Member,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let auto_nick = GuildAutoNickname::get(&self.db_pool, member.guild_id).await?;

        if let Some(nick_config) = auto_nick {
            let nickname = self.parse_nickname_template(
                &nick_config.nickname_template,
                &member.user.name,
                member.user.discriminator.map(|d| d.get()),
            );

            // Apply nickname with graceful degradation
            let mut edited_member = member.clone();
            match edited_member
                .edit(&ctx.http, EditMember::new().nickname(&nickname))
                .await
            {
                Ok(_) => {
                    tracing::info!(
                        guild_id = %member.guild_id,
                        user_id = %member.user.id,
                        nickname = %nickname,
                        "Applied auto-nickname to new member"
                    );
                }
                Err(serenity::Error::Http(ref e))
                    if e.status_code() == Some(serenity::http::StatusCode::FORBIDDEN) =>
                {
                    tracing::warn!(
                        guild_id = %member.guild_id,
                        user_id = %member.user.id,
                        "Cannot apply auto-nickname: missing permissions"
                    );
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(())
    }

    async fn send_join_log(
        &self,
        ctx: &Context,
        member: &Member,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let join_log = GuildJoinLogChannel::get(&self.db_pool, member.guild_id).await?;

        if let Some(log_config) = join_log {
            let channel_id = ChannelId::new(log_config.channel_id as u64);

            // Get member count
            let member_count = ctx
                .cache
                .guild(member.guild_id)
                .map(|g| g.member_count)
                .unwrap_or(0);

            let embed = CreateEmbed::new()
                .title("📥 Member Joined")
                .color(EmbedColor::Success.value())
                .thumbnail(
                    member
                        .user
                        .avatar_url()
                        .unwrap_or_else(|| member.user.default_avatar_url()),
                )
                .field(
                    "User",
                    format!("{} ({})", member.user.mention(), member.user.tag()),
                    false,
                )
                .field(
                    "Account Created",
                    format!("<t:{}:R>", member.user.created_at().unix_timestamp()),
                    true,
                )
                .field("Member Count", member_count.to_string(), true)
                .field("User ID", member.user.id.to_string(), true)
                .timestamp(serenity::model::Timestamp::now());

            channel_id
                .send_message(&ctx.http, CreateMessage::new().embed(embed))
                .await?;

            tracing::info!(
                guild_id = %member.guild_id,
                user_id = %member.user.id,
                channel_id = %channel_id,
                "Sent join log message"
            );
        }

        Ok(())
    }

    async fn send_leave_log(
        &self,
        ctx: &Context,
        guild_id: GuildId,
        user: &User,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let leave_log = GuildJoinLogChannel::get(&self.db_pool, guild_id).await?;

        if let Some(log_config) = leave_log {
            let channel_id = ChannelId::new(log_config.channel_id as u64);

            // Get member count
            let member_count = ctx
                .cache
                .guild(guild_id)
                .map(|g| g.member_count)
                .unwrap_or(0);

            let embed = CreateEmbed::new()
                .title("📤 Member Left")
                .color(EmbedColor::Error.value())
                .thumbnail(user.avatar_url().unwrap_or_else(|| user.default_avatar_url()))
                .field(
                    "User",
                    format!("{} ({})", user.mention(), user.tag()),
                    false,
                )
                .field("Member Count", member_count.to_string(), true)
                .field("User ID", user.id.to_string(), true)
                .timestamp(serenity::model::Timestamp::now());

            channel_id
                .send_message(&ctx.http, CreateMessage::new().embed(embed))
                .await?;

            tracing::info!(
                guild_id = %guild_id,
                user_id = %user.id,
                channel_id = %channel_id,
                "Sent leave log message"
            );
        }

        Ok(())
    }

    fn parse_nickname_template(
        &self,
        template: &str,
        username: &str,
        discriminator: Option<u16>,
    ) -> String {
        let mut result = template.to_string();
        result = result.replace("{username}", username);
        if let Some(disc) = discriminator {
            result = result.replace("{discriminator}", &disc.to_string());
        } else {
            result = result.replace("{discriminator}", "");
        }
        // Truncate to 32 characters if needed
        if result.len() > 32 {
            result.truncate(32);
        }
        result
    }
}