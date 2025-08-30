use crate::bot::{Context, Error};
use crate::data::models::RoleNameBlacklist;
use crate::utils::{EmbedBuilder, EmbedColor};
use poise::serenity_prelude as serenity;

/// Manage role name blacklist filters (Administrator only)
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    description_localized(
        "en-US",
        "Manage blacklisted words that cannot be used in booster role names"
    ),
    subcommands("add", "remove", "list")
)]
pub async fn filter(ctx: Context<'_>) -> Result<(), Error> {
    let embed = EmbedBuilder::info(
        "🚫 Role Name Filter Commands",
        "**Available subcommands:**\n\n\
        `/boosterrole filter add <word>` - Add word to blacklist\n\
        `/boosterrole filter remove <word>` - Remove word from blacklist\n\
        `/boosterrole filter list` - View all blacklisted words"
    );

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Add a word to the role name blacklist
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    description_localized(
        "en-US",
        "Add a word to the blacklist that cannot be used in booster role names"
    )
)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "The word to add to the blacklist"] word: String,
) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| Error::Command("This command can only be used in guilds".to_string()))?;

    let admin_id = ctx.author().id;

    tracing::info!(
        admin_id = %admin_id,
        guild_id = %guild_id,
        command = "boosterrole.filter.add",
        word = %word,
        "Blacklist add command invoked"
    );

    // Validate word length and content
    if word.trim().is_empty() {
        let embed = EmbedBuilder::error(
            "❌ Invalid Word",
            "Cannot add empty words to the blacklist."
        );

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    if word.len() > 50 {
        let embed = EmbedBuilder::error(
            "❌ Word Too Long",
            "Blacklisted words cannot be longer than 50 characters."
        );

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    // Add word to blacklist
    match RoleNameBlacklist::add_word(&ctx.data().db_pool, guild_id, &word.trim(), admin_id).await {
        Ok(true) => {
            let embed = serenity::CreateEmbed::new()
                .title("✅ Word Added to Blacklist")
                .description(format!(
                    "boosterrole name blacklisted: **{}**",
                    word.trim()
                ))
                .color(EmbedColor::Success.value())
                .footer(serenity::CreateEmbedFooter::new(format!(
                    "Added by {}",
                    ctx.author().name
                )))
                .timestamp(serenity::Timestamp::now());

            ctx.send(poise::CreateReply::default().embed(embed)).await?;

            tracing::info!(
                admin_id = %admin_id,
                guild_id = %guild_id,
                word = %word.trim(),
                "Word successfully added to blacklist"
            );
        }
        Ok(false) => {
            let embed = EmbedBuilder::warning(
                "⚠️ Word Already Exists",
                &format!("The word **{}** is already in the blacklist.", word.trim())
            );

            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        }
        Err(e) => {
            tracing::error!(
                admin_id = %admin_id,
                guild_id = %guild_id,
                word = %word,
                error = ?e,
                "Failed to add word to blacklist"
            );

            let embed = EmbedBuilder::error(
                "❌ Database Error",
                "Failed to add the word to the blacklist. Please try again."
            );

            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        }
    }

    Ok(())
}

/// Remove a word from the role name blacklist
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    description_localized(
        "en-US",
        "Remove a word from the blacklist so it can be used in booster role names"
    )
)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "The word to remove from the blacklist"] word: String,
) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| Error::Command("This command can only be used in guilds".to_string()))?;

    let admin_id = ctx.author().id;

    tracing::info!(
        admin_id = %admin_id,
        guild_id = %guild_id,
        command = "boosterrole.filter.remove",
        word = %word,
        "Blacklist remove command invoked"
    );

    // Remove word from blacklist
    match RoleNameBlacklist::remove_word(&ctx.data().db_pool, guild_id, &word.trim()).await {
        Ok(true) => {
            let embed = serenity::CreateEmbed::new()
                .title("✅ Word Removed from Blacklist")
                .description(format!(
                    "The word **{}** has been removed from the role name blacklist.\n\nBooster roles can now use this word.",
                    word.trim()
                ))
                .color(EmbedColor::Success.value())
                .footer(serenity::CreateEmbedFooter::new(format!(
                    "Removed by {}",
                    ctx.author().name
                )))
                .timestamp(serenity::Timestamp::now());

            ctx.send(poise::CreateReply::default().embed(embed)).await?;

            tracing::info!(
                admin_id = %admin_id,
                guild_id = %guild_id,
                word = %word.trim(),
                "Word successfully removed from blacklist"
            );
        }
        Ok(false) => {
            let embed = EmbedBuilder::warning(
                "⚠️ Word Not Found",
                &format!("The word **{}** is not in the blacklist.", word.trim())
            );

            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        }
        Err(e) => {
            tracing::error!(
                admin_id = %admin_id,
                guild_id = %guild_id,
                word = %word,
                error = ?e,
                "Failed to remove word from blacklist"
            );

            let embed = EmbedBuilder::error(
                "❌ Database Error",
                "Failed to remove the word from the blacklist. Please try again."
            );

            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        }
    }

    Ok(())
}

/// List all blacklisted words for role names
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    description_localized(
        "en-US",
        "View all words that are blacklisted from booster role names"
    ),
    aliases("ls")
)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| Error::Command("This command can only be used in guilds".to_string()))?;

    let admin_id = ctx.author().id;

    tracing::info!(
        admin_id = %admin_id,
        guild_id = %guild_id,
        command = "boosterrole.filter.list",
        "Blacklist list command invoked"
    );

    // Get all blacklisted words
    let blacklisted_words = match RoleNameBlacklist::get_all_for_guild(&ctx.data().db_pool, guild_id).await {
        Ok(words) => words,
        Err(e) => {
            tracing::error!(
                admin_id = %admin_id,
                guild_id = %guild_id,
                error = ?e,
                "Failed to fetch blacklisted words"
            );

            let embed = EmbedBuilder::error(
                "❌ Database Error",
                "Failed to fetch the blacklist. Please try again."
            );

            ctx.send(poise::CreateReply::default().embed(embed)).await?;
            return Ok(());
        }
    };

    if blacklisted_words.is_empty() {
        let embed = EmbedBuilder::primary(
            "📝 Role Name Blacklist",
            "No words are currently blacklisted for role names.\n\nUse `/boosterrole filter add <word>` to add words to the blacklist."
        );

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    // Create paginated response for large lists
    const WORDS_PER_PAGE: usize = 20;
    let total_pages = (blacklisted_words.len() + WORDS_PER_PAGE - 1) / WORDS_PER_PAGE;
    let current_page = 1; // For now, just show the first page

    let start_idx = (current_page - 1) * WORDS_PER_PAGE;
    let end_idx = std::cmp::min(start_idx + WORDS_PER_PAGE, blacklisted_words.len());
    let page_words = &blacklisted_words[start_idx..end_idx];

    let word_list = page_words
        .iter()
        .enumerate()
        .map(|(i, word)| format!("{}. **{}**", start_idx + i + 1, word))
        .collect::<Vec<_>>()
        .join("\n");

    let embed = serenity::CreateEmbed::new()
        .title("🚫 Role Name Blacklist")
        .description(format!(
            "**Blacklisted words ({} total):**\n\n{}\n\n*Role names containing these words will be rejected.*",
            blacklisted_words.len(),
            word_list
        ))
        .color(EmbedColor::Warning.value())
        .footer(serenity::CreateEmbedFooter::new(format!(
            "Page {} of {} • Requested by {}",
            current_page,
            total_pages,
            ctx.author().name
        )))
        .timestamp(serenity::Timestamp::now());

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    tracing::info!(
        admin_id = %admin_id,
        guild_id = %guild_id,
        word_count = blacklisted_words.len(),
        "Blacklist displayed successfully"
    );

    Ok(())
}