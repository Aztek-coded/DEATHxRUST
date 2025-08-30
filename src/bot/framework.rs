use crate::bot::{Data, Error, Framework};
use crate::commands::{boosterrole, cache_status, help, info, ping, prefix, test_responses};
use crate::config::Settings;
use crate::data::init_database;
use crate::handlers::BoostHandler;
use crate::utils::{EmbedBuilder, ResponseHelper};
use serenity::all::{Context, FullEvent, GuildId};
use std::sync::Arc;

/// Create and configure the Poise framework
pub async fn create_framework(settings: Settings) -> Framework {
    let mut commands = vec![
        ping::ping(),
        help::help(),
        info::info(),
        prefix::prefix(),
        cache_status::cache_status(),
        boosterrole::boosterrole(),
    ];
    
    #[cfg(debug_assertions)]
    commands.push(test_responses::test_responses());
    
    let options = poise::FrameworkOptions {
        commands,
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: None,
            dynamic_prefix: Some(|ctx| {
                Box::pin(async move {
                    let default_prefix = ctx.data.settings.command_prefix.clone();

                    let result = if let Some(guild_id) = ctx.guild_id {
                        ctx.data
                            .get_guild_prefix(guild_id.get())
                            .await
                            .ok()
                            .flatten()
                            .or(Some(default_prefix))
                    } else {
                        Some(default_prefix)
                    };

                    Ok(result)
                })
            }),
            edit_tracker: Some(std::sync::Arc::new(poise::EditTracker::for_timespan(
                std::time::Duration::from_secs(3600),
            ))),
            additional_prefixes: vec![
                poise::Prefix::Literal("death"),
                poise::Prefix::Literal("d!"),
            ],
            ..Default::default()
        },
        // The global error handler for all commands
        on_error: |error| {
            Box::pin(async move {
                match error {
                    poise::FrameworkError::Setup { error, .. } => {
                        println!("Failed to start bot: {:?}", error)
                    }
                    poise::FrameworkError::Command { error, ctx, .. } => {
                        println!(
                            "Error in command `{}`: {:?}",
                            ctx.command().qualified_name,
                            error
                        );

                        let (error_title, error_description) = match error {
                            Error::Serenity(e) => ("Discord API Error", format!("{}", e)),
                            Error::Command(e) => ("Command Error", format!("{}", e)),
                            Error::Config(e) => ("Configuration Error", format!("{}", e)),
                            Error::Database(e) => ("Database Error", format!("{}", e)),
                        };

                        // Send error as embed - maintain embed-only policy
                        let error_embed = EmbedBuilder::error(error_title, &error_description);
                        if let Err(e) = ResponseHelper::send_embed(ctx, error_embed).await {
                            println!("Failed to send error embed: {:?}", e);
                            // Try a simpler embed format if the first fails
                            let simple_embed = EmbedBuilder::error(
                                "Error",
                                "An error occurred processing your command.",
                            );
                            if let Err(e) = ResponseHelper::send_embed(ctx, simple_embed).await {
                                println!("Failed to send fallback error embed: {:?}", e);
                                // Log but don't fall back to text - maintain embed-only policy
                            }
                        }
                    }
                    poise::FrameworkError::ArgumentParse { error, ctx, .. } => {
                        println!(
                            "Argument parse error in `{}`: {}",
                            ctx.command().qualified_name,
                            error
                        );

                        // Handle argument parsing errors with embeds
                        let error_embed = EmbedBuilder::error(
                            "Invalid Arguments",
                            &format!(
                                "{}.\n\nUse `/help {}` for usage information.",
                                error,
                                ctx.command().name
                            ),
                        );

                        if let Err(e) = ResponseHelper::send_embed(ctx, error_embed).await {
                            println!("Failed to send argument error embed: {:?}", e);
                        }
                    }
                    poise::FrameworkError::CommandCheckFailed { error, ctx, .. } => {
                        println!(
                            "Command check failed for `{}`: {:?}",
                            ctx.command().qualified_name,
                            error
                        );

                        let error_embed = EmbedBuilder::error(
                            "Command Not Allowed",
                            "You don't have permission to use this command or it can't be used here."
                        );

                        if let Err(e) = ResponseHelper::send_embed(ctx, error_embed).await {
                            println!("Failed to send permission error embed: {:?}", e);
                        }
                    }
                    error => {
                        // For any other framework errors, try to send a generic embed
                        println!("Other framework error: {:?}", error);

                        // We can't get context for some errors, so we'll just log them
                        // poise::builtins::on_error would send plain text, so we avoid it
                    }
                }
            })
        },
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };

    Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);

                // Register slash commands
                let guild_id = settings.development_guild_id.map(GuildId::new);

                let commands = &framework.options().commands;

                if settings.auto_sync_commands {
                    println!("🔄 Syncing {} slash commands to guild...", commands.len());
                    poise::builtins::register_in_guild(&ctx.http, commands, guild_id.unwrap())
                        .await?;
                } else {
                    println!(
                        "🚀 Registering {} slash commands globally...",
                        commands.len()
                    );
                    poise::builtins::register_globally(&ctx.http, commands).await?;
                }

                println!("✅ Commands registered successfully!");
                println!("📋 Registered commands:");
                for cmd in commands {
                    let subcommands = if cmd.subcommands.is_empty() {
                        String::new()
                    } else {
                        let sub_names: Vec<String> =
                            cmd.subcommands.iter().map(|s| s.name.clone()).collect();
                        format!(" [subcommands: {}]", sub_names.join(", "))
                    };
                    println!(
                        "   /{} - {}{}",
                        cmd.name,
                        cmd.description.as_deref().unwrap_or("No description"),
                        subcommands
                    );
                }

                println!("🗄️ Initializing database...");
                let db_pool = init_database("data/bot.db").await?;
                println!("✅ Database initialized successfully!");

                Ok(Data::new(settings, db_pool))
            })
        })
        .options(options)
        .build()
}

/// Handle events that aren't commands
async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    // Create boost handler for this event
    let boost_handler = BoostHandler::new(Arc::new(data.db_pool.clone()));

    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            println!("🤖 {} is connected and ready!", data_about_bot.user.name);

            // Handle ready event for boost handler
            boost_handler.on_ready(ctx, data_about_bot).await;
        }
        FullEvent::GuildMemberUpdate {
            old_if_available: _,
            new: _,
            event,
        } => {
            // Handle member updates for boost status changes - simplified approach
            boost_handler.handle_boost_change(ctx, event).await;
        }
        FullEvent::GuildRoleDelete {
            guild_id,
            removed_role_id,
            removed_role_data_if_available,
        } => {
            // Handle role deletions to clean up database
            boost_handler
                .on_guild_role_delete(
                    *guild_id,
                    *removed_role_id,
                    removed_role_data_if_available.clone(),
                )
                .await;
        }
        _ => {}
    }
    Ok(())
}
