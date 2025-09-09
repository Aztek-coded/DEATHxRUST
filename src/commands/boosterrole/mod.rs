pub mod award;
pub mod base;
pub mod cleanup;
pub mod color;
pub mod dominant;
pub mod filter;
pub mod icon;
pub mod limit;
pub mod link;
pub mod list;
pub mod random;
pub mod remove;
pub mod rename;
pub mod share;

use crate::bot::{Context, Error};
use award::award;
use base::base;
use cleanup::cleanup;
use color::color;
use dominant::dominant;
use filter::filter;
use icon::icon;
use limit::limit;
use link::link;
use list::list;
use random::random;
use remove::remove;
use rename::rename;
use share::share;

/// Booster role management commands for server boosters and administrators
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Booster",
    description_localized("en-US", "Comprehensive booster role management with custom colors, filters, and admin controls"),
    subcommands("color", "dominant", "rename", "link", "filter", "list", "cleanup", "limit", "award", "icon", "random", "remove", "base", "share"),
    aliases("br", "booster"),
    broadcast_typing
)]
pub async fn boosterrole(ctx: Context<'_>) -> Result<(), Error> {
    let embed = crate::utils::EmbedBuilder::info(
        "🎨 Booster Role Commands",
        "**Booster Commands:**\n\
        `/boosterrole color <color> <name>` - Create/update your custom role\n\
        `/boosterrole dominant` - Set role color to your avatar's dominant color\n\
        `/boosterrole rename <name>` - Rename your booster role (1hr cooldown)\n\
        `/boosterrole icon <url>` - Set custom icon for your role\n\
        `/boosterrole random [style]` - Generate random color for your role\n\
        `/boosterrole remove` - Delete your custom booster role\n\n\
        **Sharing Commands:**\n\
        `/boosterrole share role <user>` - Share your role with another member\n\
        `/boosterrole share remove <role>` - Remove yourself from shared role\n\n\
        **Admin Commands:**\n\
        `/boosterrole link <user> <role>` - Link existing role to booster\n\
        `/boosterrole cleanup [dry_run]` - Remove orphaned booster roles\n\
        `/boosterrole limit [max]` - Set/view max booster roles allowed\n\
        `/boosterrole base [role]` - Set base role for hierarchy positioning\n\
        `/boosterrole award set <role>` - Set role to award new boosters\n\
        `/boosterrole award unset` - Remove award role\n\
        `/boosterrole award view` - View current award role\n\
        `/boosterrole filter add <word>` - Add word to blacklist\n\
        `/boosterrole filter remove <word>` - Remove word from blacklist\n\
        `/boosterrole filter list` - View blacklisted words\n\
        `/boosterrole share max <num>` - Set max members per shared role\n\
        `/boosterrole share limit <num>` - Set max shared roles per member\n\
        `/boosterrole share list` - View all role shares\n\
        `/boosterrole list` - View all booster roles\n\n\
        **Aliases:** `!br`, `!booster`",
    );

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
