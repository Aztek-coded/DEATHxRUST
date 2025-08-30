use death_bot::commands::{help, info, ping, prefix};
use death_bot::config::Settings;
use serenity::all::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    let settings = Settings::from_env()?;

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let (global, guild_id) = parse_arguments(&args)?;

    println!("🚀 Discord Slash Command Deployment Tool (Poise)");
    println!("=================================================");

    if global {
        println!("🌍 Deploying commands globally");
        println!("⏳ Note: Global commands can take up to 1 hour to appear");
    } else if let Some(guild_id) = guild_id {
        println!("🏰 Deploying commands to guild: {}", guild_id);
        println!("⚡ Guild commands are available immediately");
    } else {
        println!("📝 Using configuration from environment variables");
        if let Some(guild_id) = settings.development_guild_id {
            println!("🏰 Target guild: {}", guild_id);
        } else {
            println!("🌍 Target: Global");
        }
    }

    println!();

    // Create Discord HTTP client
    let client = Client::builder(&settings.discord_token, GatewayIntents::empty()).await?;
    let http = &client.http;

    // Get Poise commands
    let commands = vec![ping::ping(), help::help(), info::info(), prefix::prefix()];

    println!("📦 Prepared {} commands for deployment", commands.len());

    // Determine deployment target
    let deployment_guild_id = if global {
        None
    } else {
        guild_id.or(settings.development_guild_id)
    };

    // Deploy commands using Poise built-in functions
    match deploy_commands(http, &commands, deployment_guild_id).await {
        Ok(deployed_count) => {
            println!(
                "🎉 Successfully deployed {} slash commands!",
                deployed_count
            );

            if deployment_guild_id.is_some() {
                println!("⚡ Commands should be available immediately in the target guild");
            } else {
                println!("🕐 Global commands may take up to 1 hour to propagate to all servers");
            }

            // List deployed commands
            println!("\n📋 Deployed commands:");
            for command in &commands {
                println!(
                    "  /{} - {}",
                    command.name,
                    command.description.as_deref().unwrap_or("No description")
                );
            }
        }
        Err(e) => {
            eprintln!("❌ Deployment failed: {}", e);
            std::process::exit(1);
        }
    }

    println!("\n✨ Deployment complete!");
    Ok(())
}

async fn deploy_commands(
    http: &serenity::http::Http,
    commands: &[poise::Command<death_bot::bot::Data, death_bot::bot::Error>],
    guild_id: Option<u64>,
) -> Result<usize, Box<dyn std::error::Error>> {
    println!("🔄 Starting deployment...");

    if let Some(guild_id) = guild_id {
        println!("🏰 Deploying to guild: {}", guild_id);
        let guild_id = GuildId::new(guild_id);
        poise::builtins::register_in_guild(http, commands, guild_id).await?;
    } else {
        println!("🌍 Deploying globally");
        poise::builtins::register_globally(http, commands).await?;
    }

    println!("✅ Commands registered successfully");
    Ok(commands.len())
}

fn parse_arguments(args: &[String]) -> Result<(bool, Option<u64>), Box<dyn std::error::Error>> {
    let mut global = false;
    let mut guild_id = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--global" | "-g" => {
                global = true;
            }
            "--guild" | "-u" => {
                if i + 1 >= args.len() {
                    return Err("Guild ID required after --guild flag".into());
                }
                guild_id = Some(args[i + 1].parse::<u64>()?);
                i += 1; // Skip next argument as it's the guild ID
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            arg if arg.starts_with('-') => {
                return Err(format!("Unknown argument: {}", arg).into());
            }
            _ => {
                return Err(format!("Unexpected argument: {}", args[i]).into());
            }
        }
        i += 1;
    }

    Ok((global, guild_id))
}

fn print_help() {
    println!("Discord Slash Command Deployment Tool (Poise)");
    println!();
    println!("USAGE:");
    println!("    cargo run --bin deploy_commands [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -g, --global           Deploy commands globally (takes up to 1 hour)");
    println!("    -u, --guild <GUILD_ID> Deploy commands to specific guild (immediate)");
    println!("    -h, --help            Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    cargo run --bin deploy_commands --global");
    println!("    cargo run --bin deploy_commands --guild 123456789012345678");
    println!("    cargo run --bin deploy_commands  # Uses environment config");
    println!();
    println!("ENVIRONMENT VARIABLES:");
    println!("    DISCORD_TOKEN           Required: Your bot token");
    println!("    DEVELOPMENT_GUILD_ID    Optional: Default guild for deployment");
    println!("    AUTO_SYNC_COMMANDS      Optional: Auto-sync commands on bot start");
    println!();
    println!("NOTE: With Poise, commands are typically registered automatically");
    println!("      when the bot starts. This tool is for manual deployment only.");
}
