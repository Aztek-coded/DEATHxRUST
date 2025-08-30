pub mod color;
pub mod dominant;
pub mod filter;
pub mod link;
pub mod list;

use crate::bot::{Context, Error};
use color::color;
use dominant::dominant;
use filter::filter;
use link::link;
use list::list;

/// Booster role management commands for server boosters and administrators
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Booster",
    description_localized(
        "en-US",
        "Manage custom booster roles with colors, links, and filters"
    ),
    subcommands("color", "dominant", "link", "filter", "list"),
    aliases("br", "booster")
)]
pub async fn boosterrole(ctx: Context<'_>) -> Result<(), Error> {
    let embed = crate::utils::EmbedBuilder::info(
        "🎨 Booster Role Commands",
        "**Available subcommands:**\n\n\
        `/boosterrole color <color> <name>` - Create/update your custom role\n\
        `/boosterrole dominant` - Set role color to your avatar's dominant color\n\
        `/boosterrole link <user> <role>` - Link existing role to booster (Admin)\n\
        `/boosterrole filter add <word>` - Add word to blacklist (Admin)\n\
        `/boosterrole filter remove <word>` - Remove word from blacklist (Admin)\n\
        `/boosterrole filter list` - View blacklisted words (Admin)\n\
        `/boosterrole list` - View all booster roles (Admin)\n\n\
        **Aliases:** `!br dom`, `!br avatar`, `!br auto`, `!booster dominant`"
    );

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}