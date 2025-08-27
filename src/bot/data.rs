use crate::config::Settings;

/// Bot application data that will be accessible in all commands
#[derive(Debug)]
pub struct Data {
    pub settings: Settings,
}

impl Data {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }
}

/// Custom error type for the bot
#[derive(Debug)]
pub enum Error {
    Serenity(serenity::Error),
    Config(String),
    Command(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Serenity(e) => write!(f, "Serenity error: {}", e),
            Error::Config(e) => write!(f, "Configuration error: {}", e),
            Error::Command(e) => write!(f, "Command error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<serenity::Error> for Error {
    fn from(error: serenity::Error) -> Self {
        Self::Serenity(error)
    }
}

/// Type aliases for easier usage throughout the codebase
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;
pub type Framework = poise::Framework<Data, Error>;