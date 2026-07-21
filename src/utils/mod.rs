pub mod color_generator;
pub mod color_parser;
pub mod content_filter;
pub mod embed_builder;
pub mod error;
pub mod image_processor;
pub mod moderation;
pub mod performance;
pub mod response;
pub mod role_manager;
pub mod settings_error;
pub mod settings_rate_limiter;

pub use color_generator::ColorGenerator;
pub use color_parser::ColorParser;
pub use embed_builder::{EmbedBuilder, EmbedColor};
pub use error::{BotError, BotResult};
#[allow(unused_imports)] // Re-exports for later moderation command suites
pub use moderation::{
    case_created_embed, case_view_embed, load_staff_role_ids, member_is_staff,
    moderation_error_embed, moderation_warning_embed, normalize_reason, prepare_reason,
    require_guild_staff, validate_reason, ModerationError, MAX_REASON_LEN,
};
pub use response::ResponseHelper;
pub use role_manager::RoleManager;
pub use settings_error::SettingsError;
pub use settings_rate_limiter::SettingsRateLimiter;
