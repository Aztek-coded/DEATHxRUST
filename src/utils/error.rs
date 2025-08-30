use std::fmt;

#[derive(Debug)]
pub enum BotError {
    Config(String),
    Discord(serenity::Error),
    Io(std::io::Error),
    Command(String),
    InvalidColor(String),
    Other(String),
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BotError::Config(msg) => write!(f, "Configuration error: {}", msg),
            BotError::Discord(err) => write!(f, "Discord error: {}", err),
            BotError::Io(err) => write!(f, "IO error: {}", err),
            BotError::Command(msg) => write!(f, "Command error: {}", msg),
            BotError::InvalidColor(color) => write!(f, "Invalid color format: '{}'", color),
            BotError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for BotError {}

impl From<serenity::Error> for BotError {
    fn from(err: serenity::Error) -> Self {
        BotError::Discord(err)
    }
}

impl From<std::io::Error> for BotError {
    fn from(err: std::io::Error) -> Self {
        BotError::Io(err)
    }
}

pub type BotResult<T> = Result<T, BotError>;
