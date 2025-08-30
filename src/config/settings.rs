use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub discord_token: String,
    pub debug_mode: bool,
    pub command_prefix: String,
    pub development_guild_id: Option<u64>,
    pub auto_sync_commands: bool,
    pub slash_commands_global: bool,
    pub always_use_embeds: bool,
}

impl Settings {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let discord_token = env::var("DISCORD_TOKEN")
            .map_err(|_| "DISCORD_TOKEN environment variable is required")?;

        let debug_mode = env::var("DEBUG")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);

        let command_prefix = env::var("COMMAND_PREFIX").unwrap_or_else(|_| "!".to_string());

        let development_guild_id = env::var("DEVELOPMENT_GUILD_ID")
            .ok()
            .and_then(|id| id.parse::<u64>().ok());

        let auto_sync_commands = env::var("AUTO_SYNC_COMMANDS")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);

        let slash_commands_global = env::var("SLASH_COMMANDS_GLOBAL")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);

        let always_use_embeds = env::var("ALWAYS_USE_EMBEDS")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true);

        // Override guild_id if global commands are requested
        let final_guild_id = if slash_commands_global {
            None
        } else {
            development_guild_id
        };

        Ok(Settings {
            discord_token,
            debug_mode,
            command_prefix,
            development_guild_id: final_guild_id,
            auto_sync_commands,
            slash_commands_global,
            always_use_embeds,
        })
    }
}
