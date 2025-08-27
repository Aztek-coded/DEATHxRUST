use crate::bot::{Data, Error, Framework};
use crate::commands::{ping, info, help};
use crate::config::Settings;
use serenity::all::{GuildId, Context, FullEvent};

/// Create and configure the Poise framework
pub async fn create_framework(settings: Settings) -> Framework {
    let options = poise::FrameworkOptions {
        commands: vec![
            ping::ping(),
            help::help(),
            info::info(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(settings.command_prefix.clone()),
            edit_tracker: Some(std::sync::Arc::new(poise::EditTracker::for_timespan(
                std::time::Duration::from_secs(3600),
            ))),
            additional_prefixes: vec![
                poise::Prefix::Literal("death"),
                poise::Prefix::Literal("d!"),
            ],
            ..Default::default()
        },
        /// The global error handler for all commands
        on_error: |error| {
            Box::pin(async move {
                match error {
                    poise::FrameworkError::Setup { error, .. } => {
                        println!("Failed to start bot: {:?}", error)
                    }
                    poise::FrameworkError::Command { error, ctx, .. } => {
                        println!("Error in command `{}`: {:?}", ctx.command().qualified_name, error);
                        
                        let error_message = match error {
                            Error::Serenity(e) => format!("Discord API error: {}", e),
                            Error::Command(e) => format!("Command error: {}", e),
                            Error::Config(e) => format!("Configuration error: {}", e),
                        };
                        
                        if let Err(e) = ctx.say(format!("❌ An error occurred: {}", error_message)).await {
                            println!("Failed to send error message: {:?}", e);
                        }
                    }
                    error => {
                        if let Err(e) = poise::builtins::on_error(error).await {
                            println!("Error while handling error: {}", e)
                        }
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
                
                if settings.auto_sync_commands {
                    println!("🔄 Syncing slash commands...");
                    poise::builtins::register_in_guild(&ctx.http, &framework.options().commands, guild_id.unwrap()).await?;
                } else {
                    println!("🚀 Registering slash commands...");
                    poise::builtins::register_globally(&ctx.http, &framework.options().commands).await?;
                }
                
                println!("✅ Commands registered successfully!");
                
                Ok(Data::new(settings))
            })
        })
        .options(options)
        .build()
}

/// Handle events that aren't commands
async fn event_handler(
    _ctx: &Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            println!("🤖 {} is connected and ready!", data_about_bot.user.name);
        }
        _ => {}
    }
    Ok(())
}