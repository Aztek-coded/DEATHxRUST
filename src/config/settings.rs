use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub discord_token: String,
    pub debug_mode: bool,
}

impl Settings {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let discord_token = env::var("DISCORD_TOKEN")
            .map_err(|_| "DISCORD_TOKEN environment variable is required")?;
        
        let debug_mode = env::var("DEBUG")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);

        Ok(Settings {
            discord_token,
            debug_mode,
        })
    }
}