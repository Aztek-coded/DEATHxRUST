use thiserror::Error;

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Invalid nickname format: {0}")]
    InvalidNickname(String),

    #[error("Channel not accessible: {0}")]
    ChannelPermissionDenied(String),

    #[error("Role hierarchy conflict: {0}")]
    RoleHierarchyError(String),

    #[error("Settings limit exceeded: {0}")]
    LimitExceeded(String),

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Discord API error: {0}")]
    Discord(#[from] serenity::Error),
}

impl From<SettingsError> for crate::bot::Error {
    fn from(err: SettingsError) -> Self {
        crate::bot::Error::Command(err.to_string())
    }
}