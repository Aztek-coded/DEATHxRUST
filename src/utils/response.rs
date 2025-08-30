use crate::bot::{Context, Error};
use crate::utils::embed_builder::{EmbedBuilder, EmbedColor};
use poise::serenity_prelude::CreateEmbed;
use poise::{CreateReply, ReplyHandle};

pub struct ResponseHelper;

impl ResponseHelper {
    pub async fn send_success(
        ctx: Context<'_>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<ReplyHandle, Error> {
        let embed = EmbedBuilder::success(title, description);
        Self::send_embed(ctx, embed).await
    }

    pub async fn send_error(
        ctx: Context<'_>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<ReplyHandle, Error> {
        let embed = EmbedBuilder::error(title, description);
        Self::send_embed(ctx, embed).await
    }

    pub async fn send_warning(
        ctx: Context<'_>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<ReplyHandle, Error> {
        let embed = EmbedBuilder::warning(title, description);
        Self::send_embed(ctx, embed).await
    }

    pub async fn send_info(
        ctx: Context<'_>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<ReplyHandle, Error> {
        let embed = EmbedBuilder::info(title, description);
        Self::send_embed(ctx, embed).await
    }

    pub async fn send_primary(
        ctx: Context<'_>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<ReplyHandle, Error> {
        let embed = EmbedBuilder::primary(title, description);
        Self::send_embed(ctx, embed).await
    }

    pub async fn send_embed(ctx: Context<'_>, embed: CreateEmbed) -> Result<ReplyHandle, Error> {
        ctx.send(CreateReply::default().embed(embed))
            .await
            .map_err(Error::from)
    }

    pub async fn send_custom(
        ctx: Context<'_>,
        title: impl Into<String>,
        description: impl Into<String>,
        color: EmbedColor,
    ) -> Result<ReplyHandle, Error> {
        let embed = EmbedBuilder::custom(title, description, color);
        Self::send_embed(ctx, embed).await
    }

    pub async fn send_text_as_embed(
        ctx: Context<'_>,
        text: impl Into<String>,
    ) -> Result<ReplyHandle, Error> {
        let embed = EmbedBuilder::simple_text_to_embed(text);
        Self::send_embed(ctx, embed).await
    }

    pub async fn edit_to_embed(
        reply: &ReplyHandle<'_>,
        ctx: Context<'_>,
        embed: CreateEmbed,
    ) -> Result<(), Error> {
        reply
            .edit(ctx, CreateReply::default().embed(embed))
            .await
            .map_err(Error::from)
    }

    pub async fn edit_to_success(
        reply: &ReplyHandle<'_>,
        ctx: Context<'_>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<(), Error> {
        let embed = EmbedBuilder::success(title, description);
        Self::edit_to_embed(reply, ctx, embed).await
    }

    pub async fn edit_to_error(
        reply: &ReplyHandle<'_>,
        ctx: Context<'_>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<(), Error> {
        let embed = EmbedBuilder::error(title, description);
        Self::edit_to_embed(reply, ctx, embed).await
    }

    pub async fn defer_with_embed(ctx: Context<'_>) -> Result<ReplyHandle, Error> {
        let embed = EmbedBuilder::info("Processing", "Please wait while I process your request...");
        ctx.send(CreateReply::default().embed(embed))
            .await
            .map_err(Error::from)
    }

    /// Send an embed with fallback to simpler embed if the initial one fails
    /// This ensures embed-only responses even if permissions are limited
    pub async fn send_embed_guaranteed(
        ctx: Context<'_>,
        embed: CreateEmbed,
    ) -> Result<ReplyHandle, Error> {
        match ctx.send(CreateReply::default().embed(embed)).await {
            Ok(handle) => Ok(handle),
            Err(_) => {
                // Try with a minimal embed if the original fails
                let fallback_embed = EmbedBuilder::info("Response", "Command processed.");
                Self::send_embed(ctx, fallback_embed).await
            }
        }
    }

    /// Force all responses to be embeds - will not fall back to plain text
    pub async fn send_embed_only(
        ctx: Context<'_>,
        title: impl Into<String>,
        description: impl Into<String>,
        color: EmbedColor,
    ) -> Result<ReplyHandle, Error> {
        let embed = EmbedBuilder::custom(title, description, color);
        Self::send_embed_guaranteed(ctx, embed).await
    }
}

pub trait ContextExt {
    async fn say_embed(&self, text: impl Into<String>) -> Result<ReplyHandle, Error>;
}

impl ContextExt for Context<'_> {
    async fn say_embed(&self, text: impl Into<String>) -> Result<ReplyHandle, Error> {
        ResponseHelper::send_text_as_embed(*self, text).await
    }
}
