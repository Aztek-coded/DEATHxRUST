# Bleed Bot Commands Reference

> **Parity inventory** for DEATHxRUST vs Bleed. Not a full implementation plan.
> Source: [bleed.bot/commands](https://bleed.bot/commands) (`https://bucket.bleed.bot/commands.json`).
> Bleed catalog snapshot: **2026-04-04** UTC · **1144** commands · **36** categories.
> Guideline notes: `.grok/enhancements/bleed-bot-commands-enhancements.md`.

**Legend**

- ✅ Implemented in DEATHxRUST
- ⚠️ Partially implemented
- ❌ Not implemented
- **DEATHxRUST only** = present in this bot, not listed the same way in Bleed

---

## Implementation summary (DEATHxRUST)

### Core bot commands
- **Implemented:** 5/5 (DEATHxRUST-only helpers)
  - ✅ `ping`
  - ✅ `help`
  - ✅ `info`
  - ✅ `cache_status` (dev only)
  - ✅ `test_responses` (dev only)

### Bleed parity (from official catalog)
- **Catalog size:** 1144 commands across 36 categories
- **Implemented (exact name match):** 27
- **Partial:** 6
- **Not implemented:** 1111
- **Overall Bleed parity (exact match):** ~2.6% of catalog names

### Implemented suites
- ✅ **Prefix** — `prefix`, `prefix set`, `prefix remove` (+ our `prefix reset`); ❌ `prefix self` (Tier 2)
- ✅ **Booster roles** — full `boosterrole` suite under `src/commands/boosterrole/`
- ✅ **Settings (high priority)** — `settings`, `config`, `staff`, `autonick`, `joinlogs`, `premiumrole`, `baserole` via `boosterrole base`
- ❌ **Settings (blocked)** — jail/mute/modlog/music/Last.fm-related settings need larger systems

### Notes
- Permissions sometimes differ from Bleed (e.g. Manage Guild instead of Administrator).
- Some DEATHxRUST subcommands are more explicit than Bleed (`boosterrole filter add/remove`).
- Use a focused `.grok/roadmaps/<slug>-roadmap.md` before implementing a new suite.
- For any command, open a feature roadmap; do not treat this file as the build plan.

---

## Category index

| Category | Cog | Commands | Implemented* |
|----------|-----|----------|--------------|
| Moderation | `moderation` | 146 | 0 |
| Server | `servers` | 302 | 33 |
| Antinuke | `antinuke` | 15 | 0 |
| Antiraid | `antiraid` | 8 | 0 |
| Information | `information` | 84 | 0 |
| Fun | `fun` | 50 | 0 |
| Miscellaneous | `misc` | 52 | 0 |
| Last.fm | `lastfm` | 62 | 0 |
| Music | `music` | 28 | 0 |
| Spotify | `spotify` | 20 | 0 |
| Voicemaster | `voicemaster` | 29 | 0 |
| Tickets | `tickets` | 26 | 0 |
| Levels | `levels` | 24 | 0 |
| Giveaways | `giveaways` | 21 | 0 |
| Autorole | `autorole` | 18 | 0 |
| Reaction | `reaction` | 24 | 0 |
| Logs | `logs` | 7 | 0 |
| Snipe | `snipe` | 5 | 0 |
| Starboard | `starboard` | 15 | 0 |
| Clownboard | `clownboard` | 15 | 0 |
| Roleplay | `roleplay` | 63 | 0 |
| Manipulation | `manipulation` | 36 | 0 |
| Counters | `counters` | 5 | 0 |
| Timers | `timers` | 6 | 0 |
| Bump Reminder | `bumpreminder` | 9 | 0 |
| Fortnite | `fortnite` | 8 | 0 |
| Crypto | `crypto` | 4 | 0 |
| Instagram | `instagram` | 9 | 0 |
| X | `twitter` | 7 | 0 |
| TikTok | `tiktok` | 7 | 0 |
| Youtube | `youtube` | 6 | 0 |
| Soundcloud | `soundcloud` | 6 | 0 |
| Twitch | `twitch` | 6 | 0 |
| Reddit | `reddit` | 7 | 0 |
| Pinterest | `pinterest` | 8 | 0 |
| Kick | `kick` | 6 | 0 |

\* Implemented/partial by exact command name match only.

---

## DEATHxRUST-only / extra commands

| Command | Arguments | Permissions | Status | Notes |
|---------|-----------|-------------|--------|-------|
| `prefix reset` | none | Manage Guild | ✅ Our addition | Reset the guild prefix to default (same as remove) |
| `boosterrole filter add` | word | Manage Guild | ✅ Implemented (Bleed may use `boosterrole filter <word>`) | Add a word to the booster role name blacklist |
| `boosterrole filter remove` | word | Manage Guild | ✅ Implemented | Remove a word from the booster role name blacklist |
| `boosterrole share role` | member | Booster | ✅ Implemented as share role subcommand | Share your booster role with a member |
| `ping` | none | none | ✅ DEATHxRUST core | Test bot responsiveness |
| `help` | optional command | none | ✅ DEATHxRUST core | Show bot help information |
| `info` | none | none | ✅ DEATHxRUST core | Display bot information and statistics |
| `cache_status` | none | Administrator | ✅ DEATHxRUST dev | View database cache statistics (debug builds) |
| `test_responses` | subcommands | Administrator | ✅ DEATHxRUST dev | Development response testing (debug builds) |

---

## Full Bleed command catalog

Format per command:

- Status + name
- Description (when provided by Bleed)
- Arguments, aliases

## Moderation (`moderation`)

Easily moderate and manage your server with bleed.

**Count:** 146 · ✅ 0 · ⚠️ 0 · ❌ 146

### `ban` ❌
- **Arguments:** `member`, `delete_history`, `reason`
- **Aliases:** _none_

### `caselog` ❌
- **Arguments:** `case_id`
- **Aliases:** `case`

### `clearinvites` ❌
Remove all existing invites in guild
- **Arguments:** `params`
- **Aliases:** _none_

### `drag` ❌
Drag member(s) to the specified Voice Channel
- **Arguments:** `members`, `channel`
- **Aliases:** `d`

### `dump` ❌
Dumps all members of a role
- **Arguments:** `role`, `flags`
- **Aliases:** _none_

### `forcenickname` ❌
Force a members current nickname
- **Arguments:** `member`, `name_to_set`
- **Aliases:** `forcenick`, `freezenick`, `fn`

### `hardban` ❌
Keep a member banned
- **Arguments:** `member`, `reason`
- **Aliases:** `hb`

### `hide` ❌
Hide a channel from a role or member
- **Arguments:** `channel`, `role_or_member`
- **Aliases:** _none_

### `history` ❌
View a list of every punishment recorded
- **Arguments:** `member`, `command`
- **Aliases:** _none_

### `imute` ❌
- **Arguments:** `member`, `reason`
- **Aliases:** _none_

### `iunmute` ❌
- **Arguments:** `member`, `reason`
- **Aliases:** `imagemute`

### `jail` ❌
- **Arguments:** `member`, `duration`, `reason`
- **Aliases:** _none_

### `jaillist` ❌
View a list of every current jailed member
- **Arguments:** _none_
- **Aliases:** `currentlyjailed`, `jl`

### `lockdown` ❌
Lockdown a channel
- **Arguments:** `channel`, `reason`
- **Aliases:** `lock`

### `moderationhistory` ❌
View moderation actions from a staff member
- **Arguments:** `member`, `command`
- **Aliases:** `modhistory`, `mhistory`

### `modstats` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `moveall` ❌
Move all members in current channel to another channel
- **Arguments:** `channel`
- **Aliases:** _none_

### `mute` ❌
- **Arguments:** `member`, `duration`, `reason`
- **Aliases:** _none_

### `naughty` ❌
Temporarily make a channel NSFW for 30 seconds
- **Arguments:** `channel`
- **Aliases:** _none_

### `newmembers` ❌
View list of recently joined members
- **Arguments:** `count`
- **Aliases:** `newusers`

### `notes` ❌
View notes on a member
- **Arguments:** `member`
- **Aliases:** `note`

### `nuke` ❌
- **Arguments:** _none_
- **Aliases:** `clone`

### `permissions` ❌
Check permissions for member or myself
- **Arguments:** `member`, `channel`
- **Aliases:** `perms`

### `proof` ❌
- **Arguments:** _none_
- **Aliases:** `evidence`

### `purge` ❌
Deletes the specified amount of messages from the current channel
- **Arguments:** `member`, `search`
- **Aliases:** `prune`, `c`

### `raid` ❌
Remove all members that joined in the time provided in the event of a raid
- **Arguments:** `time`, `action`, `reason`
- **Aliases:** _none_

### `reason` ❌
Updates the reason on a case log
- **Arguments:** `case_id`, `reason`
- **Aliases:** _none_

### `recentban` ❌
Chunk ban recently joined members
- **Arguments:** `count`, `reason`
- **Aliases:** `chunkban`

### `remind` ❌
Get reminders for a duration set about whatever you choose
- **Arguments:** `reminder`
- **Aliases:** `reminder`

### `reminders` ❌
View a list of your reminders
- **Arguments:** _none_
- **Aliases:** _none_

### `rename` ❌
Assigns the mentioned user a new nickname in the guild
- **Arguments:** `member`, `newnick`
- **Aliases:** `nick`, `nickname`

### `restrictcommand` ❌
Only allows people with a certain role to use command
- **Arguments:** `cmd`, `role`
- **Aliases:** `restrictcmd`, `restrict`, `rc`

### `revokefiles` ❌
Removes/assigns the permission to attach files & embed links in the current channel
- **Arguments:** _none_
- **Aliases:** _none_

### `rmute` ❌
- **Arguments:** `member`, `reason`
- **Aliases:** `reactionmute`

### `role` ❌
Modify a member's roles
- **Arguments:** `member`, `role`
- **Aliases:** `r`

### `runmute` ❌
- **Arguments:** `member`, `reason`
- **Aliases:** _none_

### `setup` ❌
Start process for setting up the moderation system
- **Arguments:** _none_
- **Aliases:** `setme`

### `setupmute` ❌
Sets up muted roles and channel permissions
- **Arguments:** _none_
- **Aliases:** _none_

### `slowmode` ❌
Restricts members to sending one message per interval
- **Arguments:** _none_
- **Aliases:** _none_

### `softban` ❌
Softbans the mentioned user and deleting 1 day of messages
- **Arguments:** `member`, `delete_history`, `reason`
- **Aliases:** _none_

### `stickyrole` ❌
Reapplies a role on join
- **Arguments:** `member`, `role`
- **Aliases:** `sr`

### `stripstaff` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `talk` ❌
Toggle a channel to text for a role
- **Arguments:** `channel`, `role`
- **Aliases:** _none_

### `tempban` ❌
- **Arguments:** `member`, `duration`, `reason`
- **Aliases:** _none_

### `temprole` ❌
- **Arguments:** `member`, `duration`, `role`
- **Aliases:** _none_

### `thread` ❌
Commands to manage threads and forum posts
- **Arguments:** _none_
- **Aliases:** _none_

### `timeout` ❌
Mutes the provided member using Discords timeout feature
- **Arguments:** `member`, `duration`, `reason`
- **Aliases:** _none_

### `topic` ❌
Change the current channel topic
- **Arguments:** `text`
- **Aliases:** _none_

### `unban` ❌
Unbans the mentioned user
- **Arguments:** `user`, `reason`
- **Aliases:** _none_

### `unbanall` ❌
Unbans every member in a guild
- **Arguments:** _none_
- **Aliases:** `massunban`

### `unhide` ❌
Unhide a channel from a role or member
- **Arguments:** `channel`, `role_or_member`
- **Aliases:** _none_

### `unjail` ❌
Unjails the mentioned user
- **Arguments:** `member`, `reason`
- **Aliases:** _none_

### `unlock` ❌
Unlock a channel
- **Arguments:** `channel`, `reason`
- **Aliases:** `unlockdown`

### `unmute` ❌
- **Arguments:** `member`, `reason`
- **Aliases:** _none_

### `untimeout` ❌
Removes a timeout for a member
- **Arguments:** `member`, `reason`
- **Aliases:** _none_

### `warn` ❌
Warns the mentioned user and private messages them the warning
- **Arguments:** `member`, `reason`
- **Aliases:** _none_

### `warnings` ❌
View warnings for a member
- **Arguments:** `member`
- **Aliases:** `warns`

### `ban purge` ❌
- **Arguments:** `delete_history`
- **Aliases:** _none_

### `ban recent` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `forcenickname list` ❌
View a list of all forced nicknames
- **Arguments:** _none_
- **Aliases:** `view`

### `hardban list` ❌
View list of hardbanned members
- **Arguments:** _none_
- **Aliases:** _none_

### `history remove` ❌
Remove a punishment from a member
- **Arguments:** `member`, `case_id`
- **Aliases:** `delete`, `del`

### `history removeall` ❌
Remove all punishments from a member
- **Arguments:** `member`
- **Aliases:** `deleteall`, `delall`

### `history view` ❌
View an ID's case log
- **Arguments:** `case_id`
- **Aliases:** _none_

### `lockdown all` ❌
Locks all channels
- **Arguments:** `reason`
- **Aliases:** _none_

### `lockdown ignore` ❌
Blocks a channel from being altered when using the "unlock all" command
- **Arguments:** _none_
- **Aliases:** _none_

### `lockdown role` ❌
- **Arguments:** `role`
- **Aliases:** _none_

### `notes add` ❌
Add a note for a member
- **Arguments:** `member`, `note`
- **Aliases:** _none_

### `notes clear` ❌
Clears all notes for a member
- **Arguments:** `member`
- **Aliases:** `cl`

### `notes remove` ❌
Removes a note for a member
- **Arguments:** `member`, `id`
- **Aliases:** `delete`, `del`

### `nuke add` ❌
- **Arguments:** `channel`, `interval`, `message`
- **Aliases:** _none_

### `nuke archive` ❌
- **Arguments:** `channel`, `setting`
- **Aliases:** _none_

### `nuke list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `nuke remove` ❌
- **Arguments:** `channel`
- **Aliases:** _none_

### `nuke view` ❌
- **Arguments:** `channel`
- **Aliases:** _none_

### `proof add` ❌
- **Arguments:** `case_id`, `media_url`
- **Aliases:** _none_

### `proof list` ❌
- **Arguments:** `case_id`
- **Aliases:** _none_

### `proof remove` ❌
- **Arguments:** `case_id`, `index`
- **Aliases:** _none_

### `proof set` ❌
- **Arguments:** `case_id`, `explanation`
- **Aliases:** _none_

### `proof view` ❌
- **Arguments:** `case_id`
- **Aliases:** _none_

### `purge activity` ❌
Purge activity messages from chat
- **Arguments:** `search`
- **Aliases:** `activities`

### `purge after` ❌
Purge messages after a given message ID
- **Arguments:** `messagelink`
- **Aliases:** _none_

### `purge before` ❌
Purge messages before a given message ID
- **Arguments:** `messagelink`
- **Aliases:** _none_

### `purge between` ❌
Purge between two messages
- **Arguments:** `start_id`, `finish_id`
- **Aliases:** `bt`

### `purge bots` ❌
Purge messages from bots in chat
- **Arguments:** `search`
- **Aliases:** _none_

### `purge contains` ❌
Purges messages containing given substring
- **Arguments:** `substring`
- **Aliases:** _none_

### `purge embeds` ❌
Purge embeds from chat
- **Arguments:** `search`
- **Aliases:** `embed`

### `purge emoji` ❌
Purge emojis from chat
- **Arguments:** `search`
- **Aliases:** `emojis`

### `purge emotes` ❌
Purge emotes from chat
- **Arguments:** `search`
- **Aliases:** `emote`

### `purge endswith` ❌
Purge messages that ends with a given substring
- **Arguments:** `substring`
- **Aliases:** _none_

### `purge files` ❌
Purge files/attachments from chat
- **Arguments:** `search`
- **Aliases:** _none_

### `purge humans` ❌
Purge messages from humans in chat
- **Arguments:** `search`
- **Aliases:** _none_

### `purge images` ❌
Purge images (including links) from chat
- **Arguments:** `search`
- **Aliases:** _none_

### `purge links` ❌
Purge messages containing links
- **Arguments:** `search`
- **Aliases:** _none_

### `purge mentions` ❌
Purge mentions for a member from chat
- **Arguments:** `member`, `search`
- **Aliases:** _none_

### `purge reactions` ❌
Purge reactions from messages in chat
- **Arguments:** `search`
- **Aliases:** _none_

### `purge startswith` ❌
Purge messages that start with a given substring
- **Arguments:** `substring`
- **Aliases:** _none_

### `purge stickers` ❌
Purge stickers from chat
- **Arguments:** `search`
- **Aliases:** _none_

### `purge upto` ❌
Purge messages up to a message link
- **Arguments:** `messagelink`
- **Aliases:** _none_

### `purge webhooks` ❌
Purge messages from webhooks in chat
- **Arguments:** `search`
- **Aliases:** _none_

### `remind list` ❌
View a list of your reminders
- **Arguments:** _none_
- **Aliases:** _none_

### `remind remove` ❌
Remove a reminder
- **Arguments:** `id`
- **Aliases:** `delete`, `del`

### `restrictcommand add` ❌
Allows the specified role exclusive permission to use a command
- **Arguments:** `cmd`, `role`
- **Aliases:** _none_

### `restrictcommand list` ❌
View a list of every restricted command
- **Arguments:** `role_or_command`
- **Aliases:** _none_

### `restrictcommand remove` ❌
Removes the specified roles exclusive permission to use a command
- **Arguments:** `cmd`, `role`
- **Aliases:** `del`, `delete`

### `restrictcommand reset` ❌
Removes every restricted command
- **Arguments:** _none_
- **Aliases:** `clear`

### `revokefiles off` ❌
Disables permissions to attach files & embed links in a channel
- **Arguments:** `channel`
- **Aliases:** _none_

### `revokefiles on` ❌
Enable permissions to attach files & embed links in a channel
- **Arguments:** `channel`
- **Aliases:** _none_

### `role add` ❌
Adds role to a member
- **Arguments:** `member`, `role`
- **Aliases:** `set`, `give`

### `role bots` ❌
Add a role to all bots
- **Arguments:** `role`
- **Aliases:** _none_

### `role cancel` ❌
Cancels a mass role task running
- **Arguments:** _none_
- **Aliases:** `kill`

### `role color` ❌
Set a color for a role
- **Arguments:** `colour`, `role`
- **Aliases:** `colour`

### `role create` ❌
Creates a role with optional color
- **Arguments:** `color`, `role`
- **Aliases:** `make`

### `role delete` ❌
Deletes a role
- **Arguments:** `role`
- **Aliases:** `del`

### `role edit` ❌
Change a role name
- **Arguments:** `role`, `name`
- **Aliases:** `editname`, `rename`

### `role has` ❌
Add a role to members with a specific role
- **Arguments:** `role`, `assign_role`
- **Aliases:** _none_

### `role hoist` ❌
Toggle hoisting a role
- **Arguments:** `role`
- **Aliases:** _none_

### `role humans` ❌
Add a role to all humans
- **Arguments:** `role`, `flags`
- **Aliases:** _none_

### `role icon` ❌
Set an icon for a role
- **Arguments:** `url`, `role`
- **Aliases:** _none_

### `role mentionable` ❌
Toggle mentioning a role
- **Arguments:** `role`
- **Aliases:** `mention`

### `role remove` ❌
Removes role from a member
- **Arguments:** `member`, `role`
- **Aliases:** `rmv`, `take`

### `role restore` ❌
Restore roles to a member
- **Arguments:** `member`
- **Aliases:** _none_

### `role topcolor` ❌
Changes your highest roles color
- **Arguments:** `colour`, `member`
- **Aliases:** `topcolour`, `tc`

### `slowmode off` ❌
Disables slowmode in a channel
- **Arguments:** `channel`
- **Aliases:** _none_

### `slowmode on` ❌
Enable slowmode in a channel
- **Arguments:** `channel`, `delay_time`
- **Aliases:** _none_

### `stickyrole add` ❌
Reapplies a role on join
- **Arguments:** `member`, `role`
- **Aliases:** _none_

### `stickyrole list` ❌
View a list of every sticky role
- **Arguments:** _none_
- **Aliases:** _none_

### `stickyrole remove` ❌
Removes sticky role on join
- **Arguments:** `member`, `role`
- **Aliases:** `del`, `clear`

### `temprole list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `thread add` ❌
- **Arguments:** `thread`, `member`
- **Aliases:** _none_

### `thread lock` ❌
Lock a thread or forum post
- **Arguments:** `thread`, `reason`
- **Aliases:** `close`

### `thread remove` ❌
- **Arguments:** `thread`, `member`
- **Aliases:** _none_

### `thread rename` ❌
- **Arguments:** `thread`, `new_name`
- **Aliases:** _none_

### `thread unlock` ❌
Unlock a thread or forum post
- **Arguments:** `thread`, `reason`
- **Aliases:** `reopen`

### `thread watch` ❌
- **Arguments:** `thread`
- **Aliases:** _none_

### `timeout list` ❌
View list of timed out members
- **Arguments:** _none_
- **Aliases:** _none_

### `unbanall cancel` ❌
Cancels a unban all task running
- **Arguments:** _none_
- **Aliases:** `kill`

### `unlock all` ❌
Unlocks every channel
- **Arguments:** `reason`
- **Aliases:** _none_

### `lockdown ignore add` ❌
Set an ignored lockdown channel
- **Arguments:** `channel`
- **Aliases:** `create`

### `lockdown ignore list` ❌
View all ignored lockdown channels
- **Arguments:** _none_
- **Aliases:** _none_

### `lockdown ignore remove` ❌
Remove an ignored lockdown channel
- **Arguments:** `channel`
- **Aliases:** `delete`, `del`

### `role bots remove` ❌
Remove a role from all bots
- **Arguments:** `role`
- **Aliases:** _none_

### `role color gradient` ❌
Set a gradient color for a role
- **Arguments:** `colour`, `second_colour`, `role`
- **Aliases:** `g`, `grad`

### `role has remove` ❌
Remove a role from members with a specific role
- **Arguments:** `role`, `remove_role`
- **Aliases:** _none_

### `role humans remove` ❌
Remove a role from all humans
- **Arguments:** `role`
- **Aliases:** _none_

### `thread watch list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

---

## Server (`servers`)

Useful and essential commands for server management.

**Count:** 302 · ✅ 27 · ⚠️ 6 · ❌ 269

### `alias` ❌
Create your own shortcuts for commands
- **Arguments:** _none_
- **Aliases:** `shortcut`

### `autoresponder` ❌
- **Arguments:** _none_
- **Aliases:** `arp`

### `badge` ❌
- **Arguments:** `setting`
- **Aliases:** `gt`

### `boosterrole` ⚠️
- **Arguments:** `color`, `second_color`, `name`
- **Aliases:** `boostrole`, `br`

### `boosts` ❌
Set up boost messages in one or multiple channels
- **Arguments:** _none_
- **Aliases:** `boost`

### `copydisabled` ❌
Copy disabled modules, events, filters and commands to another channel
- **Arguments:** `old_channel`, `new_channel`
- **Aliases:** `cd`

### `customize` ❌
- **Arguments:** _none_
- **Aliases:** `customization`

### `disablecommand` ❌
Disable a command in a channel
- **Arguments:** `channel_or_member`, `command`
- **Aliases:** `dcmd`

### `disableevent` ❌
Disable a bot event in a channel
- **Arguments:** `channel`, `event`
- **Aliases:** `de`

### `disablemodule` ❌
Disable a module in a channel
- **Arguments:** `channel`, `module`
- **Aliases:** `dm`

### `enablecommand` ❌
Enable a previously disabled command in a channel
- **Arguments:** `channel_or_member`, `command`
- **Aliases:** `ecmd`

### `enableevent` ❌
Enable a bot event in a channel
- **Arguments:** `channel`, `event`
- **Aliases:** `ee`

### `enablemodule` ❌
Enable a module in a channel
- **Arguments:** `channel`, `module`
- **Aliases:** `em`

### `extractemotes` ❌
Sends all of your servers emojis in a zip file
- **Arguments:** _none_
- **Aliases:** _none_

### `extractstickers` ❌
Sends all of your servers stickers in a zip file
- **Arguments:** _none_
- **Aliases:** _none_

### `fakepermissions` ❌
Set up fake permissions for role through the bot!
- **Arguments:** _none_
- **Aliases:** `fakeperms`, `fp`

### `filter` ❌
View a variety of options to help clean chat
- **Arguments:** _none_
- **Aliases:** _none_

### `firstmessage` ❌
Get a link for the first message in a channel
- **Arguments:** `channel`
- **Aliases:** `firstmsg`

### `goodbye` ❌
Set up a goodbye message in one or multiple channels
- **Arguments:** _none_
- **Aliases:** _none_

### `ignore` ❌
- **Arguments:** `member_or_channel`
- **Aliases:** _none_

### `imgonly` ❌
Set up image + caption only channels
- **Arguments:** _none_
- **Aliases:** `gallery`, `imageonly`

### `invoke` ❌
Change punishment messages for DM or command response
- **Arguments:** _none_
- **Aliases:** _none_

### `pagination` ❌
Set up multiple embeds on one message
- **Arguments:** _none_
- **Aliases:** `pn`, `pages`

### `pin` ❌
Pin the most recent message or by URL
- **Arguments:** `message`
- **Aliases:** _none_

### `pins` ❌
Pin archival system commands
- **Arguments:** _none_
- **Aliases:** _none_

### `prefix` ✅
View guild prefix
- **Arguments:** _none_
- **Aliases:** _none_

### `reposter` ❌
- **Arguments:** `option`
- **Aliases:** _none_

### `setbanner` ❌
Set a new guild banner
- **Arguments:** `url`
- **Aliases:** _none_

### `seticon` ❌
Set a new guild icon
- **Arguments:** `url`
- **Aliases:** `setavatar`

### `setsplashbackground` ❌
Set a new guild splash background
- **Arguments:** `url`
- **Aliases:** `setsplash`

### `settings` ✅
Server configuration - visit https://bleed.bot/help for all commands
- **Arguments:** _none_
- **Aliases:** `bind`

### `stickymessage` ❌
Set up a sticky message in one or multiple channels
- **Arguments:** _none_
- **Aliases:** `sticky`, `sm`

### `suggest` ❌
- **Arguments:** `suggestion`
- **Aliases:** `suggestions`, `suggestion`

### `unpin` ❌
Unpin a message
- **Arguments:** `message`
- **Aliases:** _none_

### `webhook` ❌
Set up webhooks in your server
- **Arguments:** _none_
- **Aliases:** _none_

### `welcome` ❌
Set up a welcome message in one or multiple channels
- **Arguments:** _none_
- **Aliases:** `welc`

### `alias add` ❌
Create an alias for command
- **Arguments:** `shortcut`, `command`
- **Aliases:** `create`

### `alias list` ❌
List every alias for all commands
- **Arguments:** _none_
- **Aliases:** _none_

### `alias remove` ❌
Remove an alias for command
- **Arguments:** `shortcut`
- **Aliases:** `del`, `delete`

### `alias removeall` ❌
Remove an alias for command
- **Arguments:** `command`
- **Aliases:** `delall`, `deleteeall`

### `alias reset` ❌
Reset every alias for all commands
- **Arguments:** _none_
- **Aliases:** `clear`

### `alias view` ❌
View command execution for alias
- **Arguments:** `shortcut`
- **Aliases:** `show`

### `autoresponder add` ❌
Create a reply for a trigger word
- **Arguments:** `args`
- **Aliases:** `create`

### `autoresponder exclusive` ❌
- **Arguments:** `role_or_channel`, `trigger`
- **Aliases:** _none_

### `autoresponder list` ❌
View auto-reply triggers in guild
- **Arguments:** _none_
- **Aliases:** _none_

### `autoresponder remove` ❌
Remove a reply for a trigger word
- **Arguments:** `trigger`
- **Aliases:** `del`, `delete`

### `autoresponder reset` ❌
Remove every auto response
- **Arguments:** _none_
- **Aliases:** `clear`

### `autoresponder role` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `autoresponder update` ❌
Update a reply for a trigger word
- **Arguments:** `args`
- **Aliases:** _none_

### `autoresponder variables` ❌
View a list of available variables
- **Arguments:** _none_
- **Aliases:** `vars`

### `badge channel` ❌
- **Arguments:** `channel`
- **Aliases:** _none_

### `badge message` ❌
- **Arguments:** `message`
- **Aliases:** _none_

### `badge role` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `badge sync` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `boosterrole award` ✅
Reward a member a specific role upon boost
- **Arguments:** `role`
- **Aliases:** _none_

### `boosterrole base` ✅
Set the base role for where boost roles will go under
- **Arguments:** `role`
- **Aliases:** `baseid`

### `boosterrole cleanup` ✅
Clean up unused booster roles
- **Arguments:** _none_
- **Aliases:** `purge`, `truncate`

### `boosterrole color` ✅
Get your own custom booster color role
- **Arguments:** `color`, `second_color`, `name`
- **Aliases:** `colour`

### `boosterrole dominant` ✅
Set booster roles color to the most dominant color in avatar
- **Arguments:** _none_
- **Aliases:** _none_

### `boosterrole filter` ✅
Blacklist words for booster role names
- **Arguments:** `word`
- **Aliases:** _none_

### `boosterrole icon` ✅
Set an icon for booster role
- **Arguments:** `url`
- **Aliases:** _none_

### `boosterrole limit` ✅
Set limit for booster roles
- **Arguments:** `limit`
- **Aliases:** _none_

### `boosterrole link` ✅
- **Arguments:** `member`, `role`
- **Aliases:** _none_

### `boosterrole list` ✅
View all booster roles
- **Arguments:** _none_
- **Aliases:** `view`

### `boosterrole random` ✅
Set a booster role with a random hex code
- **Arguments:** _none_
- **Aliases:** `randomhex`

### `boosterrole remove` ✅
Remove custom color booster role
- **Arguments:** _none_
- **Aliases:** `delete`, `del`

### `boosterrole rename` ✅
Edit your booster roles name
- **Arguments:** `new_name`
- **Aliases:** `name`

### `boosterrole share` ⚠️
- **Arguments:** `member`
- **Aliases:** _none_

### `boosts add` ❌
Add a boost message to a channel
- **Arguments:** `channel`, `message`
- **Aliases:** `create`

### `boosts list` ❌
View all boost messages
- **Arguments:** _none_
- **Aliases:** _none_

### `boosts remove` ❌
Remove a boost message from a channel
- **Arguments:** `channel`
- **Aliases:** `delete`, `del`

### `boosts variables` ❌
View all available variables for boost messages
- **Arguments:** _none_
- **Aliases:** `vars`

### `boosts view` ❌
View a boost message for a channel
- **Arguments:** `channel`
- **Aliases:** `check`

### `customize avatar` ❌
- **Arguments:** `url`
- **Aliases:** `pfp`, `av`

### `customize banner` ❌
- **Arguments:** `url`
- **Aliases:** _none_

### `customize bio` ❌
- **Arguments:** `text`
- **Aliases:** _none_

### `disablecommand all` ❌
Disable a command in every channel
- **Arguments:** `command`
- **Aliases:** _none_

### `disablecommand list` ❌
View a list of disabled commands in guild
- **Arguments:** _none_
- **Aliases:** _none_

### `disableevent all` ❌
Disable a bot event in every channel
- **Arguments:** `event`
- **Aliases:** _none_

### `disableevent list` ❌
View a list of disabled bot events in guild
- **Arguments:** _none_
- **Aliases:** _none_

### `disablemodule all` ❌
Disable a module in every channel
- **Arguments:** `module`
- **Aliases:** _none_

### `disablemodule list` ❌
View a list of disabled modules in guild
- **Arguments:** _none_
- **Aliases:** _none_

### `enablecommand all` ❌
Enable a command in every channel
- **Arguments:** `command`
- **Aliases:** _none_

### `enableevent all` ❌
Enables a bot event in every channel
- **Arguments:** `event`
- **Aliases:** _none_

### `enablemodule all` ❌
Enables a module in every channel
- **Arguments:** `module`
- **Aliases:** _none_

### `fakepermissions add` ❌
Grant a fake permission to a role
- **Arguments:** `role`, `permission`
- **Aliases:** `grant`

### `fakepermissions list` ❌
List all fake permissions
- **Arguments:** `role`
- **Aliases:** _none_

### `fakepermissions remove` ❌
Remove a fake permission from a role
- **Arguments:** `role`, `permission`
- **Aliases:** `delete`, `del`

### `fakepermissions reset` ❌
Resets all fake permissions
- **Arguments:** _none_
- **Aliases:** `clear`

### `filter add` ❌
- **Arguments:** `word`
- **Aliases:** _none_

### `filter caps` ❌
Delete messages that contain too many uppercase characters
- **Arguments:** `channel`, `setting`, `parameters`
- **Aliases:** _none_

### `filter emoji` ❌
Delete any message exceeding the threshold for emojis
- **Arguments:** `channel`, `setting`, `parameters`
- **Aliases:** `emojis`

### `filter exempt` ❌
Exempt roles from the word filter
- **Arguments:** `role`
- **Aliases:** _none_

### `filter invites` ❌
Delete any message that contains a server link
- **Arguments:** `channel`, `setting`, `parameters`
- **Aliases:** `serverinvites`, `discordinvites`

### `filter links` ❌
Delete any message that contains a link
- **Arguments:** `channel`, `setting`, `parameters`
- **Aliases:** _none_

### `filter list` ❌
View a list of filtered words in guild
- **Arguments:** _none_
- **Aliases:** _none_

### `filter massmention` ❌
Delete any message exceeding the threshold for user mentions
- **Arguments:** `channel`, `setting`, `parameters`
- **Aliases:** _none_

### `filter musicfiles` ❌
Delete any message that contains a music file
- **Arguments:** `channel`, `setting`, `parameters`
- **Aliases:** _none_

### `filter regex` ❌
- **Arguments:** `pattern`
- **Aliases:** _none_

### `filter remove` ❌
- **Arguments:** `word`
- **Aliases:** _none_

### `filter reset` ❌
Reset all legacy filtered words
- **Arguments:** _none_
- **Aliases:** `clear`

### `filter snipe` ❌
- **Arguments:** `type`
- **Aliases:** _none_

### `filter spam` ❌
Delete messages from users that send messages too fast
- **Arguments:** `channel`, `setting`, `parameters`
- **Aliases:** `antispam`

### `filter spoilers` ❌
Delete any message exceeding the threshold for spoilers
- **Arguments:** `channel`, `setting`, `parameters`
- **Aliases:** _none_

### `filter whitelist` ❌
- **Arguments:** `word`
- **Aliases:** _none_

### `filter wordmigrate` ❌
Migrate your filtered words to Discords Automod
- **Arguments:** _none_
- **Aliases:** _none_

### `goodbye add` ❌
Add a goodbye message for a channel
- **Arguments:** `channel`, `message`
- **Aliases:** `create`

### `goodbye list` ❌
View all goodbye messages
- **Arguments:** _none_
- **Aliases:** _none_

### `goodbye remove` ❌
Remove a goodbye message from a channel
- **Arguments:** `channel`
- **Aliases:** `delete`, `del`

### `goodbye variables` ❌
View all available variables for goodbye messages
- **Arguments:** _none_
- **Aliases:** `vars`

### `goodbye view` ❌
View goodbye message for a channel
- **Arguments:** `channel`
- **Aliases:** `check`

### `ignore add` ❌
Ignore a member or channel
- **Arguments:** `member_or_channel`
- **Aliases:** _none_

### `ignore list` ❌
View a list of ignored members or channels
- **Arguments:** _none_
- **Aliases:** _none_

### `ignore remove` ❌
Remove ignoring for a member or channel
- **Arguments:** `member_or_channel`
- **Aliases:** `del`, `delete`

### `imgonly add` ❌
Set a gallery channel
- **Arguments:** `channel`
- **Aliases:** `create`

### `imgonly list` ❌
View all gallery channels
- **Arguments:** _none_
- **Aliases:** _none_

### `imgonly remove` ❌
Remove a gallery channel
- **Arguments:** `channel`
- **Aliases:** `delete`, `del`

### `invoke ban` ❌
Change ban message for DM or command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke hardban` ❌
Change hardban message for DM or command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke imute` ❌
Change imute message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke iunmute` ❌
Change iunmute message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke jail` ❌
Change jail message for DM or command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke kick` ❌
Change kick message for DM or command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke mute` ❌
Change mute message for DM or command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke rmute` ❌
Change rmute message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke runmute` ❌
Change runmute message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke softban` ❌
Change softban message for DM or command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke tempban` ❌
Change tempban message for DM or command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke timeout` ❌
Change timeout message for DM or command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke unban` ❌
Change unban message for DM or command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke unjail` ❌
Change unjail message for DM or command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke unmute` ❌
Change unmute message for DM or command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke untimeout` ❌
Change untimeout message for DM or command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke warn` ❌
Change warn message for DM or command response
- **Arguments:** _none_
- **Aliases:** _none_

### `pagination add` ❌
Add a page to a pagination embed
- **Arguments:** `messagelink`, `embed_code`
- **Aliases:** _none_

### `pagination delete` ❌
Delete a pagination embed entirely
- **Arguments:** `messagelink`
- **Aliases:** _none_

### `pagination list` ❌
View all existing pagination embeds
- **Arguments:** _none_
- **Aliases:** _none_

### `pagination remove` ❌
Remove a page from a pagination embed
- **Arguments:** `messagelink`, `id`
- **Aliases:** _none_

### `pagination reset` ❌
Remove every existing pagination in guild
- **Arguments:** _none_
- **Aliases:** `clear`

### `pagination restorereactions` ❌
Restore reactions to an existing pagination
- **Arguments:** `messagelink`
- **Aliases:** `rr`

### `pagination set` ❌
Set up an existing embed to be paginated
- **Arguments:** `messagelink`
- **Aliases:** _none_

### `pagination update` ❌
Update an existing page on pagination embed
- **Arguments:** `messagelink`, `id`, `embed_code`
- **Aliases:** _none_

### `pins archive` ❌
Archive the pins in the current channel
- **Arguments:** _none_
- **Aliases:** _none_

### `pins channel` ❌
Set the pin archival channel
- **Arguments:** `channel`
- **Aliases:** _none_

### `pins config` ❌
View the pin archival config
- **Arguments:** _none_
- **Aliases:** _none_

### `pins reset` ❌
Reset the pin archival config
- **Arguments:** _none_
- **Aliases:** _none_

### `pins set` ❌
Enable or disable the pin archival system
- **Arguments:** `option`
- **Aliases:** _none_

### `pins unpin` ❌
Enable or disable the unpinning of messages during archival
- **Arguments:** `option`
- **Aliases:** _none_

### `prefix remove` ✅
Remove command prefix for server
- **Arguments:** _none_
- **Aliases:** `delete`, `del`, `clear`

### `prefix self` ❌
Set personal prefix across all servers with bleed
- **Arguments:** `prefix`
- **Aliases:** _none_

### `prefix set` ✅
Set command prefix for server
- **Arguments:** `prefix`
- **Aliases:** `add`

### `reposter delete` ❌
- **Arguments:** `option`
- **Aliases:** _none_

### `reposter embed` ❌
- **Arguments:** `option`
- **Aliases:** _none_

### `reposter prefix` ❌
- **Arguments:** `option`
- **Aliases:** _none_

### `reposter strict` ❌
- **Arguments:** `option`
- **Aliases:** _none_

### `reposter suppress` ❌
- **Arguments:** `option`
- **Aliases:** _none_

### `settings autonick` ⚠️
- **Arguments:** `nick`
- **Aliases:** _none_

### `settings autoplay` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `settings baserole` ✅
Set the base role for where boost roles will go under
- **Arguments:** `role`
- **Aliases:** `baseid`

### `settings config` ✅
View settings configuration for guild
- **Arguments:** _none_
- **Aliases:** `list`, `configuration`

### `settings disablecustomfms` ❌
Disable custom Now Playing commands
- **Arguments:** `yes_or_no`
- **Aliases:** `disablefms`

### `settings dj` ❌
- **Arguments:** `role`
- **Aliases:** _none_

### `settings googlesafetylevel` ❌
Enable or disable safety level for Google commands
- **Arguments:** `yes_or_no`
- **Aliases:** `googlesafety`, `safetylevel`, `gs`

### `settings imuted` ❌
- **Arguments:** `role`
- **Aliases:** `imute`

### `settings jail` ❌
- **Arguments:** `channel`
- **Aliases:** _none_

### `settings jailmsg` ❌
- **Arguments:** `message`
- **Aliases:** `jailmessage`

### `settings jailrole` ❌
Set the default role for the Jail system
- **Arguments:** `role`
- **Aliases:** `jailed`, `jailedrole`

### `settings jailroles` ❌
Enable or disable removal of roles for jail
- **Arguments:** `yes_or_no`
- **Aliases:** _none_

### `settings joinlogs` ⚠️
Set a channel to log join/leaves in a server
- **Arguments:** `channel`
- **Aliases:** `joinlog`, `jl`

### `settings modlog` ❌
Set mod logs for punishments in guild
- **Arguments:** `channel`
- **Aliases:** `jaillog`

### `settings muted` ❌
- **Arguments:** `role`
- **Aliases:** `textmute`, `mute`

### `settings premiumrole` ⚠️
Set the Premium Members role for Server Subscriptions
- **Arguments:** `role`
- **Aliases:** `premiumid`, `pr`

### `settings reset` ❌
- **Arguments:** _none_
- **Aliases:** `clear`

### `settings resetcases` ❌
Reset jail-log cases
- **Arguments:** _none_
- **Aliases:** _none_

### `settings rmuted` ❌
- **Arguments:** `role`
- **Aliases:** `rmute`

### `settings staff` ⚠️
Set staff role(s)
- **Arguments:** `role`
- **Aliases:** _none_

### `stickymessage add` ❌
Add a sticky message to a channel
- **Arguments:** `channel`, `message`
- **Aliases:** _none_

### `stickymessage list` ❌
View all sticky messages
- **Arguments:** _none_
- **Aliases:** _none_

### `stickymessage remove` ❌
Remove a sticky message from a channel
- **Arguments:** `channel`
- **Aliases:** `delete`, `del`

### `stickymessage view` ❌
View the sticky message for a channel
- **Arguments:** `channel`
- **Aliases:** _none_

### `suggest approve` ❌
- **Arguments:** `id`
- **Aliases:** _none_

### `suggest config` ❌
- **Arguments:** _none_
- **Aliases:** `configuration`

### `suggest consider` ❌
- **Arguments:** `id`
- **Aliases:** _none_

### `suggest deny` ❌
- **Arguments:** `id`
- **Aliases:** `decline`

### `suggest ignore` ❌
- **Arguments:** `member_or_role`
- **Aliases:** _none_

### `suggest lock` ❌
- **Arguments:** _none_
- **Aliases:** `disable`, `off`

### `suggest progress` ❌
- **Arguments:** `id`
- **Aliases:** `working`

### `suggest reactions` ❌
- **Arguments:** `upvote`, `downvote`
- **Aliases:** _none_

### `suggest reply` ❌
- **Arguments:** `id`, `comment`
- **Aliases:** _none_

### `suggest reset` ❌
- **Arguments:** `id`
- **Aliases:** `pending`

### `suggest review` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `suggest set` ❌
- **Arguments:** `channel`
- **Aliases:** `channel`

### `suggest threads` ❌
- **Arguments:** `setting`
- **Aliases:** `thread`

### `suggest unlock` ❌
- **Arguments:** _none_
- **Aliases:** `enable`, `on`

### `webhook create` ❌
Create webhook to forward messages to
- **Arguments:** `name`
- **Aliases:** _none_

### `webhook delete` ❌
Delete webhook for a channel
- **Arguments:** `identifier`
- **Aliases:** `remove`, `del`

### `webhook edit` ❌
Send message to existing channel webhook
- **Arguments:** `messagelink`, `message_or_embed_code`
- **Aliases:** `editmessage`

### `webhook list` ❌
List all available webhooks in the server
- **Arguments:** _none_
- **Aliases:** _none_

### `webhook lock` ❌
Lock your webhook from being accessed by others
- **Arguments:** `identifier`
- **Aliases:** _none_

### `webhook send` ❌
Send message to existing channel webhook
- **Arguments:** `identifier`, `message_or_embed_code`
- **Aliases:** `message`

### `webhook unlock` ❌
Unlock your webhook from being accessed by others
- **Arguments:** `identifier`
- **Aliases:** _none_

### `welcome add` ❌
Add a welcome message for a channel
- **Arguments:** `channel`, `message`
- **Aliases:** `create`, `set`

### `welcome list` ❌
View all welcome messages
- **Arguments:** _none_
- **Aliases:** _none_

### `welcome remove` ❌
Remove a welcome message from a channel
- **Arguments:** `channel`
- **Aliases:** `delete`, `del`

### `welcome variables` ❌
View all available variables for welcome messages
- **Arguments:** _none_
- **Aliases:** `vars`

### `welcome view` ❌
View welcome message for a channel
- **Arguments:** `channel`
- **Aliases:** `check`

### `autoresponder exclusive list` ❌
- **Arguments:** `trigger`
- **Aliases:** _none_

### `autoresponder list tickets` ❌
View ticket auto-reply triggers in guild
- **Arguments:** _none_
- **Aliases:** _none_

### `autoresponder role add` ❌
- **Arguments:** `role`, `trigger`
- **Aliases:** _none_

### `autoresponder role remove` ❌
- **Arguments:** `role`, `trigger`
- **Aliases:** _none_

### `badge message view` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `badge role add` ❌
- **Arguments:** `role`
- **Aliases:** _none_

### `badge role list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `badge role remove` ❌
- **Arguments:** `role`
- **Aliases:** _none_

### `boosterrole award unset` ✅
Remove the reward role
- **Arguments:** _none_
- **Aliases:** `delete`, `remove`

### `boosterrole award view` ✅
View the current award role
- **Arguments:** _none_
- **Aliases:** _none_

### `boosterrole filter list` ✅
Blacklist words for booster role names
- **Arguments:** _none_
- **Aliases:** _none_

### `boosterrole share limit` ✅
- **Arguments:** `number`
- **Aliases:** _none_

### `boosterrole share list` ✅
- **Arguments:** _none_
- **Aliases:** _none_

### `boosterrole share max` ✅
- **Arguments:** `number`
- **Aliases:** _none_

### `boosterrole share remove` ✅
- **Arguments:** `role`
- **Aliases:** _none_

### `filter caps exempt` ❌
Exempt roles from the caps filter
- **Arguments:** `role`
- **Aliases:** _none_

### `filter emoji exempt` ❌
Exempt roles from the emoji filter
- **Arguments:** `role`
- **Aliases:** `emojis`

### `filter exempt list` ❌
View list of roles exempted from nicknames filter
- **Arguments:** _none_
- **Aliases:** _none_

### `filter invites exempt` ❌
Exempt roles from the invite filter
- **Arguments:** `role`
- **Aliases:** `serverinvites`, `discordinvites`

### `filter links exempt` ❌
Exempt roles from the links filter
- **Arguments:** `role`
- **Aliases:** _none_

### `filter links whitelist` ❌
Whitelist links from the links filter
- **Arguments:** `channel`, `url`
- **Aliases:** `wl`

### `filter massmention exempt` ❌
Exempt roles from the mass mention filter
- **Arguments:** `role`
- **Aliases:** _none_

### `filter musicfiles exempt` ❌
Exempt roles from the music files filter
- **Arguments:** `role`
- **Aliases:** _none_

### `filter spam exempt` ❌
Exempt roles from the antispam filter
- **Arguments:** `role`
- **Aliases:** `antispam`

### `filter spoilers exempt` ❌
Exempt roles from the spoilers filter
- **Arguments:** `role`
- **Aliases:** _none_

### `invoke ban dm` ❌
Change ban message for Direct Messages
- **Arguments:** `message`
- **Aliases:** _none_

### `invoke ban message` ❌
Change ban message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke hardban message` ❌
Change hardban message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke imute message` ❌
Change imute message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke iunmute message` ❌
Change iunmute message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke jail dm` ❌
Change jail message for Direct Messages
- **Arguments:** `message`
- **Aliases:** _none_

### `invoke jail message` ❌
Change jail message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke kick dm` ❌
Change kick message for Direct Messages
- **Arguments:** `message`
- **Aliases:** _none_

### `invoke kick message` ❌
Change kick message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke mute dm` ❌
Change mute message for Direct Messages
- **Arguments:** `message`
- **Aliases:** _none_

### `invoke mute message` ❌
Change mute message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke rmute message` ❌
Change rmute message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke runmute message` ❌
Change runmute message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke softban dm` ❌
Change softban message for Direct Messages
- **Arguments:** `message`
- **Aliases:** _none_

### `invoke softban message` ❌
Change softban message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke tempban dm` ❌
Change tempban message for Direct Messages
- **Arguments:** `message`
- **Aliases:** _none_

### `invoke tempban message` ❌
Change tempban message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke timeout dm` ❌
Change timeout message for Direct Messages
- **Arguments:** `message`
- **Aliases:** _none_

### `invoke timeout message` ❌
Change timeout message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke unban dm` ❌
Change unban message for Direct Messages
- **Arguments:** `message`
- **Aliases:** _none_

### `invoke unban message` ❌
Change unban message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke unjail dm` ❌
Change unjail message for Direct Messages
- **Arguments:** `message`
- **Aliases:** _none_

### `invoke unjail message` ❌
Change unjail message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke unmute dm` ❌
Change unmute message for Direct Messages
- **Arguments:** `message`
- **Aliases:** _none_

### `invoke unmute message` ❌
Change unmute message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke untimeout dm` ❌
Change untimeout message for Direct Messages
- **Arguments:** `message`
- **Aliases:** _none_

### `invoke untimeout message` ❌
Change untimeout message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `invoke warn dm` ❌
Change warn message for Direct Messages
- **Arguments:** `message`
- **Aliases:** _none_

### `invoke warn message` ❌
Change warn message for command response
- **Arguments:** `message`
- **Aliases:** `msg`

### `settings staff list` ✅
View a list of all staff roles
- **Arguments:** _none_
- **Aliases:** _none_

### `suggest ignore list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `suggest review channel` ❌
- **Arguments:** `channel`
- **Aliases:** _none_

### `autoresponder role add list` ❌
- **Arguments:** `trigger`
- **Aliases:** _none_

### `autoresponder role remove list` ❌
- **Arguments:** `trigger`
- **Aliases:** _none_

### `filter caps exempt list` ❌
View list of roles exempted from caps filter
- **Arguments:** _none_
- **Aliases:** _none_

### `filter emoji exempt list` ❌
View list of roles exempted from emoji filter
- **Arguments:** _none_
- **Aliases:** _none_

### `filter invites exempt list` ❌
View list of roles exempted from invites filter
- **Arguments:** _none_
- **Aliases:** _none_

### `filter links exempt list` ❌
View list of roles exempted from links filter
- **Arguments:** _none_
- **Aliases:** _none_

### `filter massmention exempt list` ❌
View list of roles exempted from massmention filter
- **Arguments:** _none_
- **Aliases:** _none_

### `filter musicfiles exempt list` ❌
View list of roles exempted from musicfiles filter
- **Arguments:** _none_
- **Aliases:** _none_

### `filter spam exempt list` ❌
View list of roles exempted from spam filter
- **Arguments:** _none_
- **Aliases:** _none_

### `filter spoilers exempt list` ❌
View list of roles exempted from spoilers filter
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke ban dm view` ❌
View the ban message for Direct Messages
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke ban message view` ❌
View the ban message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke hardban message view` ❌
View the hardban message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke imute message view` ❌
View the imute message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke iunmute message view` ❌
View the iunmute message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke jail dm view` ❌
View the jail message for Direct Messages
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke jail message view` ❌
View the jail message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke kick dm view` ❌
View the kick message for Direct Messages
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke kick message view` ❌
View the kick message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke mute dm view` ❌
View the mute message for Direct Messages
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke mute message view` ❌
View the mute message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke rmute message view` ❌
View the rmute message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke runmute message view` ❌
View the runmute message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke softban dm view` ❌
View the softban message for Direct Messages
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke softban message view` ❌
View the softban message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke tempban dm view` ❌
View the tempban message for Direct Messages
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke tempban message view` ❌
View the tempban message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke timeout dm view` ❌
View the timeout message for Direct Messages
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke timeout message view` ❌
View the timeout message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke unban dm view` ❌
View the unban message for Direct Messages
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke unban message view` ❌
View the unban message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke unjail dm view` ❌
View the unjail message for Direct Messages
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke unjail message view` ❌
View the unjail message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke unmute dm view` ❌
View the unmute message for Direct Messages
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke unmute message view` ❌
View the unmute message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke untimeout dm view` ❌
View the untimeout message for Direct Messages
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke untimeout message view` ❌
View the untimeout message for command response
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke warn dm view` ❌
View the warn message for Direct Messages
- **Arguments:** _none_
- **Aliases:** _none_

### `invoke warn message view` ❌
View the warn message for command response
- **Arguments:** _none_
- **Aliases:** _none_

---

## Antinuke (`antinuke`)

Easily prevent your server from malicious attacks and griefing, with a customizable threshold set by you

**Count:** 15 · ✅ 0 · ⚠️ 0 · ❌ 15

### `antinuke` ❌
Antinuke to protect your server
- **Arguments:** _none_
- **Aliases:** `an`

### `antinuke admin` ❌
Give a user permissions to edit antinuke settings
- **Arguments:** `member`
- **Aliases:** _none_

### `antinuke admins` ❌
View all antinuke admins
- **Arguments:** _none_
- **Aliases:** _none_

### `antinuke ban` ❌
Prevent mass member ban
- **Arguments:** `status`, `parameters`
- **Aliases:** _none_

### `antinuke botadd` ❌
Prevent new bot additions
- **Arguments:** `status`
- **Aliases:** _none_

### `antinuke channel` ❌
Prevent mass channel create and delete
- **Arguments:** `status`, `parameters`
- **Aliases:** _none_

### `antinuke config` ❌
View server configuration for Antinuke
- **Arguments:** _none_
- **Aliases:** `configuration`, `settings`

### `antinuke emoji` ❌
Prevent mass emoji delete Warning: This module may be unstable due to Discords rate limit
- **Arguments:** `status`, `parameters`
- **Aliases:** _none_

### `antinuke kick` ❌
Prevent mass member kick
- **Arguments:** `status`, `parameters`
- **Aliases:** _none_

### `antinuke list` ❌
View all enabled modules along with whitelisted members & bots
- **Arguments:** _none_
- **Aliases:** _none_

### `antinuke permissions` ❌
Watch dangerous permissions being granted or removed
- **Arguments:** `typee`, `permission`, `flags`
- **Aliases:** `perms`

### `antinuke role` ❌
Prevent mass role delete
- **Arguments:** `status`, `parameters`
- **Aliases:** _none_

### `antinuke vanity` ❌
Punish users that change the server vanity
- **Arguments:** `status`, `parameters`
- **Aliases:** `vanityurl`

### `antinuke webhook` ❌
Prevent mass webhook creation
- **Arguments:** `status`, `parameters`
- **Aliases:** _none_

### `antinuke whitelist` ❌
Whitelist a member from triggering antinuke or a bot to join
- **Arguments:** `member`
- **Aliases:** _none_

---

## Antiraid (`antiraid`)

Protect against targeted bot raids on your server, with our mass join, avatar and account age anti-raid filters.

**Count:** 8 · ✅ 0 · ⚠️ 0 · ❌ 8

### `antiraid` ❌
Configure protection against potential raids
- **Arguments:** _none_
- **Aliases:** _none_

### `antiraid avatar` ❌
Punish accounts without a profile picture
- **Arguments:** `setting`, `flags`
- **Aliases:** `defaultpfp`

### `antiraid config` ❌
View server antiraid configuration
- **Arguments:** _none_
- **Aliases:** `configuration`

### `antiraid massjoin` ❌
Protect server against mass bot raids
- **Arguments:** `setting`, `flags`
- **Aliases:** _none_

### `antiraid newaccounts` ❌
Punish new registered accounts
- **Arguments:** `setting`, `flags`
- **Aliases:** `age`, `newaccount`

### `antiraid state` ❌
Turn off server's raid state
- **Arguments:** _none_
- **Aliases:** `raidstate`, `removeraid`

### `antiraid whitelist` ❌
Create a one-time whitelist to allow a user to join
- **Arguments:** `member`
- **Aliases:** _none_

### `antiraid whitelist view` ❌
View all current antinuke whitelists
- **Arguments:** _none_
- **Aliases:** `list`

---

## Information (`information`)

Useful commands, like highlights and other social media platforms commands.

**Count:** 84 · ✅ 0 · ⚠️ 0 · ❌ 84

### `avatar` ❌
Get avatar of a member or yourself
- **Arguments:** `user`
- **Aliases:** `av`, `avi`, `pfp`, `ab`, `ag`

### `banner` ❌
Get the banner of a member or yourself
- **Arguments:** `user`
- **Aliases:** `userbanner`

### `birthday` ❌
View your birthday or somebody elses
- **Arguments:** `member`
- **Aliases:** `bday`

### `boosters` ❌
View all recent server boosters
- **Arguments:** _none_
- **Aliases:** _none_

### `bots` ❌
View all bots in the server
- **Arguments:** _none_
- **Aliases:** _none_

### `cashapp` ❌
Retrieve simple CashApp profile information
- **Arguments:** `username`
- **Aliases:** _none_

### `channelinfo` ❌
View information about a channel
- **Arguments:** `channel`
- **Aliases:** `cinfo`, `ci`

### `compress` ❌
Compress image to lower quality
- **Arguments:** `ratio`, `url`
- **Aliases:** _none_

### `define` ❌
Get definition of a word
- **Arguments:** `word`
- **Aliases:** _none_

### `donate` ❌
Donate to the bot's hosting expenses
- **Arguments:** _none_
- **Aliases:** `donation`, `support`

### `emoji` ❌
Returns a large emoji or server emote
- **Arguments:** `emoji`
- **Aliases:** `emote`

### `emotes` ❌
View all emotes in the server
- **Arguments:** _none_
- **Aliases:** `emojis`

### `github` ❌
Gets profile information on the given Github user
- **Arguments:** `username`
- **Aliases:** `git`

### `guildbanner` ❌
Returns banner icon
- **Arguments:** `guild_id`
- **Aliases:** `gbanner`

### `guildicon` ❌
Returns guild icon
- **Arguments:** `guild_id`
- **Aliases:** `servericon`, `gicon`, `sicon`

### `hex` ❌
Grab the most dominant color from an image
- **Arguments:** `url_or_attachment_or_member`
- **Aliases:** `dominant`

### `highlight` ❌
Set notifications for when a keyword is said
- **Arguments:** _none_
- **Aliases:** _none_

### `invert` ❌
Invert an image's colors
- **Arguments:** `url`
- **Aliases:** _none_

### `inviteinfo` ❌
View basic invite code information
- **Arguments:** `code`
- **Aliases:** `ii`

### `membercount` ❌
View server member count
- **Arguments:** _none_
- **Aliases:** `mc`, `serverstats`, `serverstatistics`

### `members` ❌
View members in a role
- **Arguments:** `role`
- **Aliases:** `inrole`

### `minecraft` ❌
Gets minecraft profile information
- **Arguments:** `username`
- **Aliases:** `namemc`

### `osu` ❌
Retrieve simple OSU! profile information
- **Arguments:** `username`, `game`
- **Aliases:** _none_

### `roblox` ❌
Gets profile information on the given Roblox user
- **Arguments:** `username`
- **Aliases:** `rblx`

### `roleinfo` ❌
View information about a role
- **Arguments:** `role`
- **Aliases:** `rinfo`, `ri`

### `roles` ❌
View all roles in the server
- **Arguments:** _none_
- **Aliases:** _none_

### `rotate` ❌
Rotate an image by a provided degree
- **Arguments:** `degree`, `url`
- **Aliases:** _none_

### `screenshot` ❌
Get an image of a website
- **Arguments:** `url`
- **Aliases:** `ss`

### `seen` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `serveravatar` ❌
Get the server avatar of a member or yourself
- **Arguments:** `user`
- **Aliases:** `sav`, `savi`, `spfp`, `serverav`, `gav`, `guildav`

### `serverbanner` ❌
Get the server banner of a member or yourself
- **Arguments:** `user`
- **Aliases:** `sbanner`

### `serverinfo` ❌
View information about a server
- **Arguments:** `guild_id`
- **Aliases:** `guildinfo`, `sinfo`, `si`, `ginfo`

### `snapchat` ❌
Get bitmoji and QR scan code for user
- **Arguments:** `username`
- **Aliases:** `snap`

### `snapchatstory` ❌
Gets all current stories for the given Snapchat user
- **Arguments:** `username`
- **Aliases:** `snapstory`

### `splash` ❌
Returns splash background
- **Arguments:** `guild_id`
- **Aliases:** _none_

### `status` ❌
Get a link to the status page
- **Arguments:** _none_
- **Aliases:** _none_

### `steam` ❌
- **Arguments:** `id_or_id64`
- **Aliases:** _none_

### `sticker` ❌
Modfy or add stickers to your server!
- **Arguments:** _none_
- **Aliases:** _none_

### `telegram` ❌
Gets profile information on the given Telegram user or group
- **Arguments:** `username`
- **Aliases:** `tele`

### `timezone` ❌
View your current time or somebody elses
- **Arguments:** `member`
- **Aliases:** `tz`, `time`

### `urbandictionary` ❌
Gets the definition of a word/slang from Urban Dictionary
- **Arguments:** `word`
- **Aliases:** `ud`, `urban`

### `userinfo` ❌
View information about a member or yourself
- **Arguments:** `member`
- **Aliases:** `whois`, `uinfo`, `info`, `ui`

### `valorant` ❌
Get valorant player information
- **Arguments:** `user`
- **Aliases:** `val`

### `weather` ❌
Gets simple weather from OpenWeatherMap
- **Arguments:** `city`
- **Aliases:** _none_

### `xbox` ❌
Gets profile information on the given Xbox gamertag
- **Arguments:** `gamertag`
- **Aliases:** _none_

### `birthday celebrate` ❌
- **Arguments:** `role`
- **Aliases:** _none_

### `birthday channel` ❌
- **Arguments:** `channel`
- **Aliases:** _none_

### `birthday config` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `birthday list` ❌
View a list of every member's birthday
- **Arguments:** _none_
- **Aliases:** `view`

### `birthday lock` ❌
- **Arguments:** _none_
- **Aliases:** `disable`, `off`

### `birthday role` ❌
- **Arguments:** `role`
- **Aliases:** _none_

### `birthday set` ❌
- **Arguments:** `date`
- **Aliases:** _none_

### `birthday unlock` ❌
- **Arguments:** _none_
- **Aliases:** `enable`, `on`

### `boosters lost` ❌
View list of most recent lost boosters
- **Arguments:** _none_
- **Aliases:** _none_

### `emoji add` ❌
Downloads emote and adds to server
- **Arguments:** `emoji`, `characters`
- **Aliases:** `copy`

### `emoji addmany` ❌
Bulk add emotes to the current server
- **Arguments:** `emotes`
- **Aliases:** `am`

### `emoji information` ❌
View the most recent emote used
- **Arguments:** `message_link`
- **Aliases:** `info`

### `emoji remove` ❌
Removes emote from server
- **Arguments:** `emoji`
- **Aliases:** `delete`, `del`

### `emoji removeduplicates` ❌
Remove duplicates of emotes
- **Arguments:** _none_
- **Aliases:** `rmdups`

### `emoji removemany` ❌
Bulk remove emotes from the current server
- **Arguments:** `emotes`
- **Aliases:** `rm`, `deletemany`, `dm`

### `emoji rename` ❌
Renames emote to the new name provided
- **Arguments:** `emoji`, `new_name`
- **Aliases:** `editname`

### `emoji stats` ❌
Show top ten most used emotes
- **Arguments:** _none_
- **Aliases:** _none_

### `highlight add` ❌
Add a highlighted keyword
- **Arguments:** `keyword`
- **Aliases:** _none_

### `highlight ignore` ❌
Ignore notifications from members or a channel or a role
- **Arguments:** `member_or_channel_or_role`
- **Aliases:** _none_

### `highlight list` ❌
List all keywords set in a server
- **Arguments:** _none_
- **Aliases:** _none_

### `highlight remove` ❌
Remove a highlighted keyword
- **Arguments:** `keyword`
- **Aliases:** `delete`, `del`

### `highlight reset` ❌
- **Arguments:** _none_
- **Aliases:** `clear`

### `roblox check` ❌
- **Arguments:** `username`, `asset_id_or_name`
- **Aliases:** _none_

### `roblox devex` ❌
- **Arguments:** `robux`
- **Aliases:** _none_

### `roblox fromdiscord` ❌
Get Roblox account from Discord account
- **Arguments:** `user_id`
- **Aliases:** _none_

### `roblox inventory` ❌
- **Arguments:** `username`
- **Aliases:** `rap`

### `roblox item` ❌
- **Arguments:** `query`
- **Aliases:** _none_

### `roblox outfits` ❌
View all outfits for a user
- **Arguments:** `username`
- **Aliases:** `outfit`

### `roblox template` ❌
Download asset for an item
- **Arguments:** `asset_id`
- **Aliases:** `asset`, `a`

### `roblox todiscord` ❌
Get Discord account from Roblox account
- **Arguments:** `username`
- **Aliases:** _none_

### `sticker add` ❌
Downloads sticker and adds to server
- **Arguments:** `url`, `name`
- **Aliases:** `copy`, `create`, `steal`

### `sticker cleanup` ❌
Cleans server sticker names
- **Arguments:** _none_
- **Aliases:** _none_

### `sticker remove` ❌
Removes sticker from server
- **Arguments:** `name`
- **Aliases:** `delete`, `del`

### `sticker rename` ❌
Rename the attached sticker to given new name
- **Arguments:** `new_name`
- **Aliases:** `editname`

### `sticker tag` ❌
Add server vanity to stickers
- **Arguments:** _none_
- **Aliases:** _none_

### `timezone list` ❌
View a list of every member's timezone
- **Arguments:** _none_
- **Aliases:** `view`

### `timezone set` ❌
Set your timezone
- **Arguments:** `location`
- **Aliases:** _none_

### `birthday celebrate list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `highlight ignore list` ❌
List all ignored members, channel & roles
- **Arguments:** _none_
- **Aliases:** _none_

---

## Fun (`fun`)

Various fun commands and more uncategorized bleed commands for you.

**Count:** 50 · ✅ 0 · ⚠️ 0 · ❌ 50

### `anime` ❌
Search MyAnimeList for anime information
- **Arguments:** `search`
- **Aliases:** _none_

### `blacktea` ❌
Find a word with 3 letters!
- **Arguments:** _none_
- **Aliases:** _none_

### `book` ❌
Shows information of a book from Goodreads
- **Arguments:** `search`
- **Aliases:** `goodreads`

### `character` ❌
Search MyAnimeList for character information
- **Arguments:** `search`
- **Aliases:** _none_

### `duckduckgo` ❌
Search the DuckDuckGo search engine
- **Arguments:** `search`
- **Aliases:** `ddg`

### `duckduckgoimage` ❌
Search duckduckgo for an image
- **Arguments:** `search`
- **Aliases:** `ddgim`, `ddgimg`

### `game` ❌
Returns information about the specified game title given
- **Arguments:** `title`
- **Aliases:** `gamesearch`

### `giphy` ❌
Search Giphy for gif results
- **Arguments:** `keyword`
- **Aliases:** `gif`

### `google` ❌
Search the largest search engine on the internet
- **Arguments:** `search`
- **Aliases:** `g`

### `image` ❌
Search Google for an image
- **Arguments:** `search`
- **Aliases:** `im`, `img`

### `juul` ❌
Share a juul with your friends!
- **Arguments:** _none_
- **Aliases:** _none_

### `lego` ❌
Legofy an image
- **Arguments:** `url`, `flags`
- **Aliases:** `legoify`, `legofy`

### `lyrics` ❌
Gets lyrics for the given song
- **Arguments:** `query`
- **Aliases:** `lyric`, `lyr`

### `makegif` ❌
Convert videos into a GIF
- **Arguments:** `url`, `quality`, `fps`, `fast_forward`
- **Aliases:** `m2g`

### `manga` ❌
Search MyAnimeList for manga information
- **Arguments:** `search`
- **Aliases:** _none_

### `movie` ❌
Returns information about the specified movie title given
- **Arguments:** `title`
- **Aliases:** `kino`

### `ocr` ❌
Detects text in an image
- **Arguments:** `url`
- **Aliases:** _none_

### `ocrtr` ❌
Detects text in an image and translates to desired language
- **Arguments:** `url`, `to_language`
- **Aliases:** `ocrtranslate`

### `quote` ❌
Quote a message
- **Arguments:** `text`
- **Aliases:** _none_

### `reverseimage` ❌
Reverse Images on Google
- **Arguments:** `url`
- **Aliases:** `rimage`, `rimg`

### `steal` ❌
View the most recent emote used
- **Arguments:** `message_link`
- **Aliases:** _none_

### `tags` ❌
- **Arguments:** `tag_name`
- **Aliases:** `tag`, `t`

### `tenor` ❌
Search Tenor for gif results
- **Arguments:** `keyword`
- **Aliases:** _none_

### `tictactoe` ❌
Play tic-tac-toe with somebody!
- **Arguments:** `member`
- **Aliases:** `ttt`

### `tone` ❌
Run Google Perspective on text
- **Arguments:** `text`
- **Aliases:** `perspective`

### `translate` ❌
- **Arguments:** `to_language`, `from_language`, `text`
- **Aliases:** `tr`

### `transparent` ❌
Remove background from an image
- **Arguments:** `url`, `flags`
- **Aliases:** `tp`

### `tts` ❌
Sends a .mp3 file of text speech
- **Arguments:** `speaker`, `text`
- **Aliases:** `texttospeech`

### `tvshow` ❌
Returns information about the specified TV show title given
- **Arguments:** `title`
- **Aliases:** `show`

### `wolfram` ❌
Gets basic information about a query, like Google search
- **Arguments:** `query`
- **Aliases:** `wolframalpha`, `w`, `wa`, `calc`

### `blacktea end` ❌
- **Arguments:** _none_
- **Aliases:** `quit`, `stop`, `cancel`

### `juul flavor` ❌
Change the servers juul's flavor
- **Arguments:** `flavor`
- **Aliases:** `pod`

### `juul hit` ❌
Hit the servers juul
- **Arguments:** _none_
- **Aliases:** _none_

### `juul pass` ❌
Pass the servers juul to someone else
- **Arguments:** `member`
- **Aliases:** _none_

### `juul stats` ❌
Show the servers juul stats
- **Arguments:** _none_
- **Aliases:** _none_

### `juul steal` ❌
Steal the servers juul
- **Arguments:** _none_
- **Aliases:** _none_

### `juul toggle` ❌
Toggle the servers juul on or off
- **Arguments:** _none_
- **Aliases:** _none_

### `movie expand` ❌
Returns more information on a movie
- **Arguments:** `title`
- **Aliases:** _none_

### `tags add` ❌
Add a tag to guild
- **Arguments:** `tag_name`, `context`
- **Aliases:** `create`

### `tags author` ❌
View the author of a tag
- **Arguments:** `tag_name`
- **Aliases:** `owner`, `creator`

### `tags edit` ❌
Edit the contents of your tag
- **Arguments:** `tag_name`, `new_context`
- **Aliases:** `change`, `update`

### `tags list` ❌
View a list of every tag in guild
- **Arguments:** `member`
- **Aliases:** _none_

### `tags random` ❌
Return a random tag
- **Arguments:** _none_
- **Aliases:** _none_

### `tags remove` ❌
Remove a tag from guild
- **Arguments:** `tag_name`
- **Aliases:** `del`, `delete`

### `tags rename` ❌
Rename your tags name
- **Arguments:** `tag_name`, `new_name`
- **Aliases:** `editname`

### `tags reset` ❌
Reset every tag for this guild
- **Arguments:** _none_
- **Aliases:** _none_

### `tags search` ❌
Search for tags containing a keyword
- **Arguments:** `search`
- **Aliases:** `look`

### `tictactoe leaderboard` ❌
View the most tic-tac-toe wins
- **Arguments:** _none_
- **Aliases:** `lb`

### `tictactoe statistics` ❌
View tic-tac-toe statistics of a member
- **Arguments:** `member`
- **Aliases:** `stats`

### `tts channel` ❌
Speak in a voice chat with text-to-speech
- **Arguments:** `speaker`, `text`
- **Aliases:** `c`, `ch`

---

## Miscellaneous (`misc`)

Explore uncategorized commands for bleed.

**Count:** 52 · ✅ 0 · ⚠️ 0 · ❌ 52

### `addemote` ❌
Downloads emote and adds to server
- **Arguments:** `emoji`, `characters`
- **Aliases:** `adde`

### `afk` ❌
Set an AFK status for when you are mentioned
- **Arguments:** `status`
- **Aliases:** _none_

### `brainly` ❌
- **Arguments:** `url_or_attachment_or_text`
- **Aliases:** _none_

### `charinfo` ❌
Get information about a character/symbol..etc...
- **Arguments:** `characters`
- **Aliases:** _none_

### `chatgpt` ❌
Ask a question using the ChatGPT API
- **Arguments:** `question`
- **Aliases:** `ask`, `askgpt`, `gpt`

### `choose` ❌
Give me choices and I will pick for you
- **Arguments:** `choices`
- **Aliases:** _none_

### `cleargnames` ❌
Reset your guild's name history
- **Arguments:** _none_
- **Aliases:** `clearguildnames`

### `clearnames` ❌
Reset your name history
- **Arguments:** _none_
- **Aliases:** _none_

### `color` ❌
Show a hex codes color in a embed
- **Arguments:** `hex`
- **Aliases:** `colour`

### `createembed` ❌
Create your own embed
- **Arguments:** `embed_code`
- **Aliases:** `ce`

### `discog` ❌
Integrate your Discogs experience with bleed's commands. Get started with `discog login` to connect your account.
- **Arguments:** _none_
- **Aliases:** `discogs`

### `editembed` ❌
Edit an embed you created
- **Arguments:** `messagelink`, `embed_code`
- **Aliases:** `edite`

### `embed` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `embedcode` ❌
Copy an existing embeds code for creating an embed
- **Arguments:** `messagelink`
- **Aliases:** `copyembed`, `ec`

### `freaky` ❌
Freakify text
- **Arguments:** `text`
- **Aliases:** `freak`

### `futbol` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `fyp` ❌
Get a random TikTok video
- **Arguments:** _none_
- **Aliases:** _none_

### `gnames` ❌
View guild name changes
- **Arguments:** `guild_id`
- **Aliases:** `guildnames`, `servernames`, `snames`

### `help` ❌
View extended help for commands
- **Arguments:** `command`
- **Aliases:** `commands`, `h`

### `invites` ❌
View all active invites
- **Arguments:** _none_
- **Aliases:** _none_

### `jumbo` ❌
- **Arguments:** `emoji`
- **Aliases:** `e`, `enlarge`, `enlargen`

### `makemp3` ❌
Convert a video to an audio file (strictly cdn URLs)
- **Arguments:** `url`
- **Aliases:** `mp3`

### `mlb` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `names` ❌
View username and nickname history of a member or yourself
- **Arguments:** `member`
- **Aliases:** `namehistory`, `nicks`, `nh`

### `nba` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `nfl` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `nhl` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `poll` ❌
Create a short poll
- **Arguments:** `time`, `question`
- **Aliases:** _none_

### `quickpoll` ❌
Add up/down arrow to message initiating a poll
- **Arguments:** `msg`
- **Aliases:** `qp`

### `randomhex` ❌
Generate a random hex (color)
- **Arguments:** _none_
- **Aliases:** _none_

### `rps` ❌
Play Rock-paper-scissors with me!
- **Arguments:** `choice`
- **Aliases:** `rockpaperscissors`

### `run` ❌
- **Arguments:** `code`
- **Aliases:** `compile`, `exec`

### `shazam` ❌
Find a song by providing video or audio
- **Arguments:** `url`
- **Aliases:** _none_

### `soccer` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `timediff` ❌
Find the time difference between any two Discord IDs
- **Arguments:** `first_id`, `second_id`
- **Aliases:** `timedifference`, `tdiff`, `diff`, `td`

### `topcommands` ❌
View the most used commands
- **Arguments:** _none_
- **Aliases:** _none_

### `transcribe` ❌
Transcribe text by providing video or audio
- **Arguments:** `url`
- **Aliases:** `ts`

### `uwu` ❌
Uwuify text
- **Arguments:** `text`
- **Aliases:** `uwuify`

### `wikihow` ❌
How to...?
- **Arguments:** `query`
- **Aliases:** `whow`

### `wouldyourather` ❌
Would you rather?
- **Arguments:** `choose`
- **Aliases:** `wyr`

### `afk mentions` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `discog collections` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `discog login` ❌
- **Arguments:** _none_
- **Aliases:** `set`

### `discog logout` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `discog profile` ❌
- **Arguments:** `user`
- **Aliases:** _none_

### `discog search` ❌
- **Arguments:** `query`
- **Aliases:** _none_

### `discog wantlist` ❌
- **Arguments:** _none_
- **Aliases:** `wishlist`

### `embed copy` ❌
- **Arguments:** `messagelink`
- **Aliases:** _none_

### `embed create` ❌
- **Arguments:** `name`
- **Aliases:** `c`, `edit`

### `embed delete` ❌
- **Arguments:** `name`
- **Aliases:** `del`

### `embed list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `embed preview` ❌
- **Arguments:** `name`
- **Aliases:** `view`

---

## Last.fm (`lastfm`)

See leaderboards, statistics and your Last.fm data with commands.

**Count:** 62 · ✅ 0 · ⚠️ 0 · ❌ 62

### `itunes` ❌
Finds a song from the iTunes API
- **Arguments:** `song`
- **Aliases:** _none_

### `lastfm` ❌
- **Arguments:** _none_
- **Aliases:** `lf`, `lfm`

### `nowplaying` ❌
Shows your current song playing from Last.fm
- **Arguments:** `member`
- **Aliases:** `np`, `fm`, `now`

### `spotifyalbum` ❌
Finds album results from the Spotify API
- **Arguments:** `album`
- **Aliases:** `spalbum`

### `spotifytrack` ❌
Finds track results from the Spotify API
- **Arguments:** `track`
- **Aliases:** `sptrack`

### `lastfm collage` ❌
View a collage of your most listened to albums
- **Arguments:** `member`, `rows_x_cols`, `period`
- **Aliases:** `col`, `chart`, `art`

### `lastfm color` ❌
Set embed color for Last.fm commands
- **Arguments:** `hexc`
- **Aliases:** `embed`

### `lastfm count` ❌
View your total Last.fm scrobbles
- **Arguments:** `member`
- **Aliases:** `total`

### `lastfm crowns` ❌
View a list of your crowns
- **Arguments:** `member`
- **Aliases:** _none_

### `lastfm customcommand` ❌
Set your own custom Now Playing command
- **Arguments:** `substring`
- **Aliases:** `customnp`, `customfm`, `cc`

### `lastfm customreactions` ❌
Set personal upvote and downvote reaction for Now Playing
- **Arguments:** `upvote`, `downvote`
- **Aliases:** `customreact`, `customreaction`, `cr`

### `lastfm favorites` ❌
View yours or a member's liked tracks
- **Arguments:** `member`
- **Aliases:** `favs`, `likes`, `liked`, `loved`

### `lastfm globalboard` ❌
View the Last.fm globalboard (reactions)
- **Arguments:** _none_
- **Aliases:** `gboard`, `gb`

### `lastfm globalwhoknows` ❌
View the top listeners for an artist globally
- **Arguments:** `artist`
- **Aliases:** `globalwk`, `gwk`

### `lastfm globalwkalbum` ❌
View the top listeners for an album globally
- **Arguments:** `artist`
- **Aliases:** `globalwka`, `gwka`

### `lastfm globalwktrack` ❌
View the top listeners for a track globally
- **Arguments:** `artist`
- **Aliases:** `globalwkt`, `gwkt`

### `lastfm hide` ❌
Hide users from appearing on whoknows commands
- **Arguments:** `member`
- **Aliases:** _none_

### `lastfm itunes` ❌
Gives iTunes link for the current song playing
- **Arguments:** `member`
- **Aliases:** _none_

### `lastfm login` ❌
- **Arguments:** _none_
- **Aliases:** `set`

### `lastfm logout` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `lastfm lyrics` ❌
Gets lyrics from Musixmatch for current song playing
- **Arguments:** `member`
- **Aliases:** `lyr`

### `lastfm milestone` ❌
- **Arguments:** `number`
- **Aliases:** _none_

### `lastfm mode` ❌
Use a different embed for NP or create your own
- **Arguments:** `type`
- **Aliases:** _none_

### `lastfm mostcrowns` ❌
View a list of members with the most crowns
- **Arguments:** _none_
- **Aliases:** `allcrowns`, `crownsall`, `crownslb`

### `lastfm now` ❌
Shows your current song playing from Last.fm
- **Arguments:** `member`
- **Aliases:** `fm`, `nowplaying`, `np`

### `lastfm overview` ❌
See your statistics for an artist
- **Arguments:** `member`, `artistname`
- **Aliases:** `ov`

### `lastfm playing` ❌
See what song everyone is listening to in a server
- **Arguments:** _none_
- **Aliases:** _none_

### `lastfm plays` ❌
Check how many plays you have for an artist
- **Arguments:** `member`, `artist`
- **Aliases:** _none_

### `lastfm playsalbum` ❌
Check how many plays you have for an album
- **Arguments:** `member`, `artist_and_album`
- **Aliases:** `playsa`, `aplays`

### `lastfm playsall` ❌
Check how many plays you have for every song on an album
- **Arguments:** `member`, `artist_and_album`
- **Aliases:** _none_

### `lastfm playstrack` ❌
Check how many plays you have for a specific track
- **Arguments:** `member`, `artist_and_track`
- **Aliases:** `playst`, `tplays`

### `lastfm react` ❌
Set server upvote and downvote reaction for Now Playing
- **Arguments:** `upvote`, `downvote`
- **Aliases:** `reaction`, `reactions`

### `lastfm recent` ❌
View your recent tracks
- **Arguments:** `member`
- **Aliases:** `recenttracks`, `last`, `lp`

### `lastfm recentfor` ❌
View your recent tracks for an artist
- **Arguments:** `artist`
- **Aliases:** _none_

### `lastfm recommendation` ❌
Recommends a random artist from your library
- **Arguments:** `member`
- **Aliases:** `recommend`

### `lastfm score` ❌
View your Last.fm score and statistics
- **Arguments:** `member`
- **Aliases:** `stats`, `statistics`

### `lastfm scoreboard` ❌
View the Last.fm server scoreboard (reactions)
- **Arguments:** _none_
- **Aliases:** `leaderboard`, `sb`, `serverboard`

### `lastfm soundcloud` ❌
Gives Soundcloud link for the current song playing
- **Arguments:** `member`
- **Aliases:** `sc`

### `lastfm spotify` ❌
Gives Spotify link for the current song playing
- **Arguments:** `member`
- **Aliases:** `sp`

### `lastfm streak` ❌
View your current listening streak
- **Arguments:** `member`
- **Aliases:** _none_

### `lastfm taste` ❌
Compare your music taste between you and someone else
- **Arguments:** `member`, `period`
- **Aliases:** _none_

### `lastfm topalbums` ❌
View your most listened to albums
- **Arguments:** `member`, `period`
- **Aliases:** `tab`, `album`, `topalbum`, `albums`, `tl`

### `lastfm topartists` ❌
View your most listened to artists
- **Arguments:** `member`, `period`
- **Aliases:** `artists`, `artist`, `tar`, `topartist`, `ta`

### `lastfm toptenalbums` ❌
View your top ten albums for an artist
- **Arguments:** `member`, `artist`
- **Aliases:** `tta`

### `lastfm toptentracks` ❌
View your top ten tracks for an artist
- **Arguments:** `member`, `artist`
- **Aliases:** `ttt`

### `lastfm toptracks` ❌
View your most listened to tracks
- **Arguments:** `member`, `period`
- **Aliases:** `track`, `tracks`, `ttr`, `toptrack`, `tt`

### `lastfm update` ❌
Update your Last.fm library
- **Arguments:** `parameters`
- **Aliases:** `index`

### `lastfm url` ❌
Submit your own artworks for an album cover if you don't want the artwork from Last.fm
- **Arguments:** `url`, `album`
- **Aliases:** `changeartwork`

### `lastfm vote` ❌
Vote for submitted album artworks to display on the Now Playing command
- **Arguments:** `artist_and_album`
- **Aliases:** _none_

### `lastfm whois` ❌
View Last.fm profile information
- **Arguments:** `member`
- **Aliases:** `profile`

### `lastfm whoknows` ❌
View the top listeners for an artist in a guild
- **Arguments:** `artist`
- **Aliases:** `wk`

### `lastfm wkalbum` ❌
View the top listeners for an album by an artist
- **Arguments:** `album`
- **Aliases:** `wka`, `whoknowsalbum`

### `lastfm wktrack` ❌
View the top listeners for a specific song by an artist
- **Arguments:** `track`
- **Aliases:** `wkt`, `whoknowstrack`

### `lastfm youtube` ❌
Gives YouTube link for the current song playing
- **Arguments:** `member`
- **Aliases:** `yt`

### `lastfm customcommand blacklist` ❌
Blacklist users their own Now Playing command
- **Arguments:** `member`
- **Aliases:** `bl`

### `lastfm customcommand cleanup` ❌
Clean up custom commands from absent members
- **Arguments:** _none_
- **Aliases:** _none_

### `lastfm customcommand list` ❌
View list of custom commands for NP
- **Arguments:** _none_
- **Aliases:** _none_

### `lastfm customcommand public` ❌
Toggle public flag for a custom command
- **Arguments:** `substring`
- **Aliases:** _none_

### `lastfm customcommand remove` ❌
Remove a custom command for a member
- **Arguments:** `member`
- **Aliases:** _none_

### `lastfm customcommand reset` ❌
Resets all custom commands
- **Arguments:** _none_
- **Aliases:** `clear`

### `lastfm hide list` ❌
View the list of all hidden members
- **Arguments:** _none_
- **Aliases:** `view`

### `lastfm customcommand blacklist list` ❌
View list of blacklisted custom command users for NP
- **Arguments:** _none_
- **Aliases:** `view`, `check`

---

## Music (`music`)

Premium audio quality and commands for a superior music experience.

**Count:** 28 · ✅ 0 · ⚠️ 0 · ❌ 28

### `disconnect` ❌
- **Arguments:** _none_
- **Aliases:** `stop`, `dc`

### `fastforward` ❌
- **Arguments:** `position`
- **Aliases:** `seek`, `ff`

### `pause` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `play` ❌
- **Arguments:** `next`, `query`
- **Aliases:** `p`

### `preset` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `queue` ❌
View all tracks queued
- **Arguments:** _none_
- **Aliases:** `q`

### `repeat` ❌
- **Arguments:** `option`
- **Aliases:** `loop`

### `resume` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `rewind` ❌
- **Arguments:** `position`
- **Aliases:** `rw`

### `shuffle` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `skip` ❌
- **Arguments:** _none_
- **Aliases:** `next`, `sk`

### `volume` ❌
- **Arguments:** `volume`
- **Aliases:** `vol`

### `preset 8d` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `preset active` ❌
- **Arguments:** _none_
- **Aliases:** `list`

### `preset boost` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `preset chipmunk` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `preset flat` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `preset karaoke` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `preset metal` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `preset nightcore` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `preset piano` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `preset soft` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `preset vaporwave` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `preset vibrato` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `queue empty` ❌
Empty the queue
- **Arguments:** _none_
- **Aliases:** `clear`

### `queue move` ❌
Move a track to a different position in the queue
- **Arguments:** `position`, `new_position`
- **Aliases:** _none_

### `queue remove` ❌
Remove a track from the queue
- **Arguments:** `position`
- **Aliases:** `del`

### `queue shuffle` ❌
Shuffle the queue
- **Arguments:** _none_
- **Aliases:** _none_

---

## Spotify (`spotify`)

Control your music on Spotify through commands.

**Count:** 20 · ✅ 0 · ⚠️ 0 · ❌ 20

### `spotify` ❌
Control your music on Spotify through commands or search for a track. Get started with `spotify login` to connect your account.
- **Arguments:** `track`
- **Aliases:** `sp`

### `spotify device` ❌
Change the device that youre listening to Spotify with
- **Arguments:** _none_
- **Aliases:** `devices`

### `spotify like` ❌
Like your current playing song on Spotify
- **Arguments:** _none_
- **Aliases:** _none_

### `spotify login` ❌
Grant bleed access to your Spotify account
- **Arguments:** _none_
- **Aliases:** `register`

### `spotify logout` ❌
Disconnect your Spotify from our servers
- **Arguments:** _none_
- **Aliases:** `unlink`, `disconnect`

### `spotify next` ❌
Skip to the next song
- **Arguments:** _none_
- **Aliases:** `skip`, `ss`, `s`

### `spotify pause` ❌
Pause the current song
- **Arguments:** _none_
- **Aliases:** `stop`

### `spotify play` ❌
Immediately skip to the requested song
- **Arguments:** `query`
- **Aliases:** `p`

### `spotify previous` ❌
Skip to the previous song
- **Arguments:** _none_
- **Aliases:** `prev`, `back`

### `spotify queue` ❌
Queue a song
- **Arguments:** `query`
- **Aliases:** `q`

### `spotify repeat` ❌
Repeat the current song
- **Arguments:** `mode`
- **Aliases:** _none_

### `spotify resume` ❌
Resume the current song
- **Arguments:** _none_
- **Aliases:** `unpause`

### `spotify seek` ❌
Seek to position in current song
- **Arguments:** `seconds`
- **Aliases:** _none_

### `spotify shuffle` ❌
Toggle playback shuffle
- **Arguments:** `option`
- **Aliases:** _none_

### `spotify topartists` ❌
Show top artists for the specified time frame
- **Arguments:** `duration`
- **Aliases:** `artists`, `artist`, `tar`, `topartist`, `ta`

### `spotify toptracks` ❌
Show top tracks for the specified time frame
- **Arguments:** `duration`
- **Aliases:** `track`, `tracks`, `ttr`, `toptrack`, `tt`

### `spotify unlike` ❌
Unlike your current playing song on Spotify
- **Arguments:** _none_
- **Aliases:** _none_

### `spotify vc` ❌
Play your current track in a voice channel
- **Arguments:** _none_
- **Aliases:** _none_

### `spotify volume` ❌
Adjust current player volume
- **Arguments:** `percent`
- **Aliases:** `vol`

### `spotify device list` ❌
List all current devices connected to your Spotify account
- **Arguments:** _none_
- **Aliases:** _none_

---

## Voicemaster (`voicemaster`)

Temporary personalized voice channels for your server.

**Count:** 29 · ✅ 0 · ⚠️ 0 · ❌ 29

### `voicemaster` ❌
Make temporary voice channels in your server!
- **Arguments:** _none_
- **Aliases:** `voice`, `vm`, `vc`

### `voicemaster bitrate` ❌
Edit bitrate of your voice channel
- **Arguments:** `bitrate`
- **Aliases:** _none_

### `voicemaster category` ❌
Redirect voice channels to custom category
- **Arguments:** `channel`
- **Aliases:** _none_

### `voicemaster claim` ❌
Claim an inactive voice channel
- **Arguments:** _none_
- **Aliases:** _none_

### `voicemaster configuration` ❌
See current configuration for current Voice Channel
- **Arguments:** _none_
- **Aliases:** `show`, `view`, `config`, `info`

### `voicemaster default` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `voicemaster ghost` ❌
Hide your voice channel
- **Arguments:** _none_
- **Aliases:** `hide`

### `voicemaster join` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `voicemaster limit` ❌
Set a member limit to your voice channel
- **Arguments:** `limit`
- **Aliases:** _none_

### `voicemaster lock` ❌
Lock your voice channel
- **Arguments:** _none_
- **Aliases:** _none_

### `voicemaster music` ❌
Change your channel to a Music Only channel
- **Arguments:** `setting`
- **Aliases:** _none_

### `voicemaster name` ❌
Rename your voice channel
- **Arguments:** `new_name`
- **Aliases:** `rename`

### `voicemaster permit` ❌
Permit a member or role to join your VC
- **Arguments:** `member_or_role`
- **Aliases:** `allow`, `approve`

### `voicemaster reject` ❌
Reject a member or role from joining your VC
- **Arguments:** `member_or_role`
- **Aliases:** `remove`, `deny`, `kick`

### `voicemaster reset` ❌
Reset server configuration for VoiceMaster
- **Arguments:** _none_
- **Aliases:** `resetserver`

### `voicemaster role` ❌
Grant roles to members who join and remove from members leaving
- **Arguments:** `role`
- **Aliases:** `roles`

### `voicemaster sendinterface` ❌
Forcefully resend VoiceMaster interface
- **Arguments:** _none_
- **Aliases:** _none_

### `voicemaster setup` ❌
Begin VoiceMaster server configuration setup
- **Arguments:** _none_
- **Aliases:** _none_

### `voicemaster status` ❌
Set a status for your voice channel
- **Arguments:** `status`
- **Aliases:** _none_

### `voicemaster transfer` ❌
Transfer ownership of your channel to another member
- **Arguments:** `member`
- **Aliases:** _none_

### `voicemaster unghost` ❌
Unhide your voice channel
- **Arguments:** _none_
- **Aliases:** `unhide`

### `voicemaster unlock` ❌
Unlock your voice channel
- **Arguments:** _none_
- **Aliases:** _none_

### `voicemaster category private` ❌
Set the private category for VoiceMaster channels
- **Arguments:** `channel`
- **Aliases:** `priv`

### `voicemaster default bitrate` ❌
Edit default bitrate for new Voice Channels
- **Arguments:** `bitrate`
- **Aliases:** _none_

### `voicemaster default interface` ❌
Send interface to new Voice Channels
- **Arguments:** `setting`
- **Aliases:** _none_

### `voicemaster default name` ❌
Set default name for new Voice Channels
- **Arguments:** `name`
- **Aliases:** _none_

### `voicemaster default region` ❌
Edit default region for new Voice Channels
- **Arguments:** `region`
- **Aliases:** _none_

### `voicemaster default role` ❌
Set the default role for bleed to set permissions for
- **Arguments:** `role`
- **Aliases:** _none_

### `voicemaster join role` ❌
- **Arguments:** `role`
- **Aliases:** _none_

---

## Tickets (`tickets`)

Tickets preview.

**Count:** 26 · ✅ 0 · ⚠️ 0 · ❌ 26

### `tickets` ❌
Ticket system commands
- **Arguments:** _none_
- **Aliases:** `ticket`, `tix`

### `tickets allow` ❌
Allow a user or role to see the current ticket
- **Arguments:** `member_or_role`
- **Aliases:** `add`

### `tickets blacklist` ❌
Add or remove a member or role from the ticket blacklist
- **Arguments:** `member_or_role`
- **Aliases:** `bl`, `block`

### `tickets claim` ❌
Claim a ticket
- **Arguments:** `channel_and_reason`
- **Aliases:** _none_

### `tickets close` ❌
Close a ticket (use current channel if none specified)
- **Arguments:** `channel_and_reason`
- **Aliases:** _none_

### `tickets delete` ❌
Permanently delete a ticket and its channel
- **Arguments:** `channel_and_reason`
- **Aliases:** `del`

### `tickets deny` ❌
Deny a user or role from seeing the current ticket
- **Arguments:** `member_or_role`
- **Aliases:** `remove`

### `tickets forms` ❌
Manage reusable ticket forms
- **Arguments:** `name`
- **Aliases:** `form`, `frm`, `f`

### `tickets list` ❌
List all currently open tickets in this server
- **Arguments:** _none_
- **Aliases:** _none_

### `tickets move` ❌
Move a ticket to another option, or move a channel to a category
- **Arguments:** `channel_and_reason`
- **Aliases:** `migrate`

### `tickets options` ❌
Manage the options for ticket panels
- **Arguments:** _none_
- **Aliases:** `option`, `opts`, `opt`, `o`

### `tickets panels` ❌
Manage ticket panels
- **Arguments:** `name`
- **Aliases:** `panel`, `pan`, `pnl`, `p`

### `tickets profile` ❌
Manage your personal ticket claim profile
- **Arguments:** _none_
- **Aliases:** `pro`, `prof`

### `tickets profiles` ❌
Manage ticket profiles for this server
- **Arguments:** _none_
- **Aliases:** `profileadmin`, `ticketprofiles`

### `tickets reason` ❌
Update a stored ticket action reason
- **Arguments:** `action`, `target_and_reason`
- **Aliases:** _none_

### `tickets rename` ❌
Rename a channel
- **Arguments:** `channel_and_name`
- **Aliases:** `name`

### `tickets reopen` ❌
Reopen a ticket
- **Arguments:** `channel_and_reason`
- **Aliases:** _none_

### `tickets resend` ❌
Resend a ticket panel message
- **Arguments:** `channel`
- **Aliases:** `res`, `refresh`, `send`, `update`

### `tickets stats` ❌
Show tickets stats for this server
- **Arguments:** `target`
- **Aliases:** `counts`, `count`

### `tickets trainee` ❌
Manage trainee permissions for the current ticket
- **Arguments:** _none_
- **Aliases:** `trainees`

### `tickets transcript` ❌
Generate or update a ticket transcript
- **Arguments:** `channel_and_reason`
- **Aliases:** `trans`

### `tickets unclaim` ❌
Remove the current claimer from a ticket
- **Arguments:** `channel`
- **Aliases:** _none_

### `tickets allow list` ❌
List users and roles explicitly allowed to see the current ticket
- **Arguments:** _none_
- **Aliases:** _none_

### `tickets trainee grant` ❌
Grant trainee speak/claim/close permissions for the current ticket
- **Arguments:** `member_or_role`, `permissions`
- **Aliases:** _none_

### `tickets trainee list` ❌
List trainee permission overrides for the current ticket
- **Arguments:** _none_
- **Aliases:** _none_

### `tickets trainee revoke` ❌
Revoke trainee speak/claim/close permissions for the current ticket
- **Arguments:** `member_or_role`, `permissions`
- **Aliases:** `remove`

---

## Levels (`levels`)

Reward members with roles by leveling up in your server.

**Count:** 24 · ✅ 0 · ⚠️ 0 · ❌ 24

### `levels` ❌
View your level and experience
- **Arguments:** `member`
- **Aliases:** `level`, `rank`, `ranks`

### `removexp` ❌
Remove experience from a user
- **Arguments:** `user`, `amount`
- **Aliases:** _none_

### `setlevel` ❌
Set a user's level
- **Arguments:** `user`, `level`
- **Aliases:** `setrank`

### `setxp` ❌
Set a user's experience
- **Arguments:** `user`, `amount`
- **Aliases:** _none_

### `levels add` ❌
Create level role
- **Arguments:** `role`, `rank`
- **Aliases:** `create`

### `levels cleanup` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `levels config` ❌
View server configuration for Leveling system
- **Arguments:** _none_
- **Aliases:** `settings`, `configuration`

### `levels ignore` ❌
Ignore a channel or role for XP
- **Arguments:** `target`
- **Aliases:** _none_

### `levels leaderboard` ❌
View the highest ranking members
- **Arguments:** _none_
- **Aliases:** `top`

### `levels list` ❌
View all ignored channels and roles
- **Arguments:** _none_
- **Aliases:** _none_

### `levels lock` ❌
Disable leveling system
- **Arguments:** _none_
- **Aliases:** `off`, `disable`

### `levels message` ❌
Set custom level up message
- **Arguments:** `text`
- **Aliases:** _none_

### `levels messagemode` ❌
Set up where level up messages will be sent
- **Arguments:** `mode`
- **Aliases:** _none_

### `levels messages` ❌
Toggle level up messages for yourself
- **Arguments:** `setting`
- **Aliases:** _none_

### `levels remove` ❌
Remove level role
- **Arguments:** `rank`
- **Aliases:** `delete`, `del`

### `levels reset` ❌
Reset all members level and XP
- **Arguments:** _none_
- **Aliases:** _none_

### `levels roles` ❌
View all XP roles
- **Arguments:** _none_
- **Aliases:** `xproles`, `ranks`, `levels`

### `levels setrate` ❌
Set multiplier for XP gain
- **Arguments:** `multiplier`
- **Aliases:** `multiplier`

### `levels stackroles` ❌
Enable or disable stacking of roles
- **Arguments:** `option`
- **Aliases:** _none_

### `levels sync` ❌
Update your members level roles
- **Arguments:** _none_
- **Aliases:** _none_

### `levels unlock` ❌
Enable leveling system
- **Arguments:** _none_
- **Aliases:** `on`, `enable`

### `levels update` ❌
Update a level roles rank
- **Arguments:** `role`, `rank`
- **Aliases:** `updaterole`

### `levels leaderboard rename` ❌
Set the title of the leaderboard embeds
- **Arguments:** `text`
- **Aliases:** `name`

### `levels message view` ❌
View the level up message for the server
- **Arguments:** _none_
- **Aliases:** `check`

---

## Giveaways (`giveaways`)

Easily create events where members can win prizes.

**Count:** 21 · ✅ 0 · ⚠️ 0 · ❌ 21

### `giveaways` ❌
- **Arguments:** _none_
- **Aliases:** `giveaway`, `gw`

### `giveaways cancel` ❌
- **Arguments:** `message_link`
- **Aliases:** `delete`

### `giveaways edit` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `giveaways end` ❌
- **Arguments:** `message_link`
- **Aliases:** _none_

### `giveaways list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `giveaways reroll` ❌
- **Arguments:** `message_link`, `winners`
- **Aliases:** _none_

### `giveaways start` ❌
- **Arguments:** `channel`, `duration`, `winners`, `prize`
- **Aliases:** `s`

### `giveaways edit age` ❌
- **Arguments:** `message_link`, `days`
- **Aliases:** `accountage`

### `giveaways edit color` ❌
- **Arguments:** `message_link`, `color`
- **Aliases:** _none_

### `giveaways edit description` ❌
- **Arguments:** `message_link`, `text`
- **Aliases:** `desc`

### `giveaways edit duration` ❌
- **Arguments:** `message_link`, `date`
- **Aliases:** _none_

### `giveaways edit host` ❌
- **Arguments:** `message_link`, `members`
- **Aliases:** `hosts`

### `giveaways edit image` ❌
- **Arguments:** `message_link`, `url_or_attachment`
- **Aliases:** _none_

### `giveaways edit maxlevel` ❌
- **Arguments:** `message_link`, `level`
- **Aliases:** _none_

### `giveaways edit minlevel` ❌
- **Arguments:** `message_link`, `level`
- **Aliases:** _none_

### `giveaways edit prize` ❌
- **Arguments:** `message_link`, `prize`
- **Aliases:** `name`, `rename`, `title`

### `giveaways edit requiredroles` ❌
- **Arguments:** `message_link`, `roles`
- **Aliases:** _none_

### `giveaways edit roles` ❌
- **Arguments:** `message_link`, `roles`
- **Aliases:** _none_

### `giveaways edit stay` ❌
- **Arguments:** `message_link`, `days`
- **Aliases:** `minimumstay`

### `giveaways edit thumbnail` ❌
- **Arguments:** `message_link`, `url_or_attachment`
- **Aliases:** _none_

### `giveaways edit winners` ❌
- **Arguments:** `message_link`, `count`
- **Aliases:** _none_

---

## Autorole (`autorole`)

Set up auto roles, reaction & button roles for members in your server.

**Count:** 18 · ✅ 0 · ⚠️ 0 · ❌ 18

### `autorole` ❌
Set up automatic role assign on member join
- **Arguments:** _none_
- **Aliases:** `ar`

### `buttonrole` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `reactionrole` ❌
Set up self-assignable roles with reactions
- **Arguments:** _none_
- **Aliases:** `rr`

### `autorole add` ❌
Adds a autorole and assigns on join to member
- **Arguments:** `role`
- **Aliases:** `create`

### `autorole list` ❌
View a list of every auto role
- **Arguments:** _none_
- **Aliases:** `all`

### `autorole remove` ❌
Removes a autorole and stops assigning on join
- **Arguments:** `role`
- **Aliases:** `delete`, `del`

### `autorole reset` ❌
Clears every autorole for guild
- **Arguments:** _none_
- **Aliases:** `clear`

### `buttonrole add` ❌
- **Arguments:** `message_link`, `role`, `style`, `emoji`, `label`
- **Aliases:** _none_

### `buttonrole list` ❌
- **Arguments:** _none_
- **Aliases:** `all`

### `buttonrole remove` ❌
- **Arguments:** `message_link`, `index`
- **Aliases:** _none_

### `buttonrole removeall` ❌
- **Arguments:** `messagelink`
- **Aliases:** `deleteall`, `delall`

### `buttonrole reset` ❌
- **Arguments:** _none_
- **Aliases:** `clear`

### `reactionrole add` ❌
Adds a reaction role to a message
- **Arguments:** `messagelink`, `reaction`, `role`
- **Aliases:** `create`

### `reactionrole list` ❌
View a list of every reaction role
- **Arguments:** _none_
- **Aliases:** `all`

### `reactionrole remove` ❌
Removes a reaction role from a message
- **Arguments:** `messagelink`, `reaction`
- **Aliases:** `delete`, `del`

### `reactionrole removeall` ❌
Removes all reaction roles from a message
- **Arguments:** `messagelink`
- **Aliases:** `deleteall`, `delall`

### `reactionrole reset` ❌
Clears every reaction role from guild
- **Arguments:** _none_
- **Aliases:** `clear`

### `reactionrole restore` ❌
Choose whether reaction roles restore when members rejoin
- **Arguments:** `option`
- **Aliases:** `rejoin`

---

## Reaction (`reaction`)

Set up reaction triggers on messages based on keywords.

**Count:** 24 · ✅ 0 · ⚠️ 0 · ❌ 24

### `noselfreact` ❌
Prevent self reactions on messages
- **Arguments:** _none_
- **Aliases:** `nsr`

### `previousreact` ❌
- **Arguments:** `emoji`, `trigger_word`
- **Aliases:** `pr`, `previousreaction`

### `reaction` ❌
Add a reaction(s) to a message
- **Arguments:** `message_link`, `emoji_or_emote`
- **Aliases:** `reactiontrigger`, `rt`, `react`

### `noselfreact bypass` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `noselfreact emoji` ❌
- **Arguments:** `emoji_or_emote`
- **Aliases:** _none_

### `noselfreact exempt` ❌
- **Arguments:** `member_or_channel_role`
- **Aliases:** `exclude`

### `noselfreact punishment` ❌
- **Arguments:** `punishment`
- **Aliases:** _none_

### `noselfreact toggle` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `previousreact add` ❌
Adds a reaction trigger to guild
- **Arguments:** `emoji`, `trigger_word`
- **Aliases:** _none_

### `previousreact clear` ❌
Removes every previous reaction trigger in guild
- **Arguments:** _none_
- **Aliases:** `reset`

### `previousreact delete` ❌
Removes a previous reaction trigger in guild
- **Arguments:** `emote`, `trigger_word`
- **Aliases:** `remove`, `del`

### `previousreact deleteall` ❌
Removes every reaction trigger for a specific word
- **Arguments:** `trigger_word`
- **Aliases:** `removeall`, `delall`

### `previousreact list` ❌
View a list of every previous reaction trigger in guild
- **Arguments:** _none_
- **Aliases:** _none_

### `previousreact owner` ❌
Gets the author of a previous reaction trigger
- **Arguments:** `trigger_word`
- **Aliases:** `author`, `creator`

### `reaction add` ❌
Adds a reaction trigger to guild
- **Arguments:** `emoji`, `trigger_word`
- **Aliases:** _none_

### `reaction clear` ❌
Removes every reaction trigger in guild
- **Arguments:** _none_
- **Aliases:** `reset`

### `reaction delete` ❌
Removes a reaction trigger in guild
- **Arguments:** `emoji`, `trigger_word`
- **Aliases:** `remove`, `del`

### `reaction deleteall` ❌
Removes every reaction trigger for a specific word
- **Arguments:** `trigger_word`
- **Aliases:** `removeall`, `delall`

### `reaction list` ❌
View a list of every reaction trigger in guild
- **Arguments:** _none_
- **Aliases:** _none_

### `reaction messages` ❌
Add or remove auto reaction on messages
- **Arguments:** `channel`, `first`, `second`, `third`
- **Aliases:** `message`, `msg`

### `reaction owner` ❌
Gets the author of a trigger word
- **Arguments:** `trigger_word`
- **Aliases:** `author`, `creator`

### `noselfreact emoji list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `noselfreact exempt list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `reaction messages list` ❌
List auto reactions for all channels
- **Arguments:** _none_
- **Aliases:** _none_

---

## Logs (`logs`)

Log various types of events in your server.

**Count:** 7 · ✅ 0 · ⚠️ 0 · ❌ 7

### `log` ❌
- **Arguments:** _none_
- **Aliases:** `logging`, `logger`, `logs`

### `log add` ❌
- **Arguments:** `channel`, `event`
- **Aliases:** _none_

### `log color` ❌
- **Arguments:** `channel`, `event`, `color`
- **Aliases:** _none_

### `log ignore` ❌
- **Arguments:** `member_or_channel`
- **Aliases:** _none_

### `log remove` ❌
- **Arguments:** `channel`, `event`
- **Aliases:** _none_

### `log color list` ❌
- **Arguments:** `channel`
- **Aliases:** _none_

### `log ignore list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

---

## Snipe (`snipe`)

Snipe various types of content from members.

**Count:** 5 · ✅ 0 · ⚠️ 0 · ❌ 5

### `clearsnipe` ❌
Clear all results for reactions, edits and messages
- **Arguments:** _none_
- **Aliases:** `clearsnipes`, `cs`

### `editsnipe` ❌
Snipe the latest message that was edited
- **Arguments:** `index`
- **Aliases:** `es`

### `reactionhistory` ❌
See logged reactions for a message
- **Arguments:** `messagelink`
- **Aliases:** `rh`

### `reactionsnipe` ❌
Snipe the latest reaction that was removed
- **Arguments:** _none_
- **Aliases:** `rs`

### `snipe` ❌
Snipe the latest message that was deleted
- **Arguments:** `index`
- **Aliases:** `s`

---

## Starboard (`starboard`)

Highlight the best moments in your server to a channel.

**Count:** 15 · ✅ 0 · ⚠️ 0 · ❌ 15

### `starboard` ❌
Showcase the best messages in your server
- **Arguments:** _none_
- **Aliases:** `star`

### `starboard attachments` ❌
Allow attachments to appear on Starboard posts
- **Arguments:** `setting`
- **Aliases:** _none_

### `starboard color` ❌
Set default color for starboard posts
- **Arguments:** `color`
- **Aliases:** _none_

### `starboard config` ❌
View the settings for starboard in guild
- **Arguments:** _none_
- **Aliases:** `configuration`

### `starboard emoji` ❌
Sets the emoji that triggers the starboard messages
- **Arguments:** `emoji`
- **Aliases:** _none_

### `starboard ignore` ❌
- **Arguments:** `channel_or_member_or_role`
- **Aliases:** _none_

### `starboard jumpurl` ❌
Allow the jump URL to appear on a Starboard post
- **Arguments:** `setting`
- **Aliases:** _none_

### `starboard lock` ❌
Disables/locks starboard from operating
- **Arguments:** _none_
- **Aliases:** `disable`, `off`

### `starboard reset` ❌
Resets guild's configuration for starboard
- **Arguments:** _none_
- **Aliases:** `delete`, `del`

### `starboard selfstar` ❌
Allow an author to star their own message
- **Arguments:** `setting`
- **Aliases:** _none_

### `starboard set` ❌
Sets the channel where starboard messages will be sent to
- **Arguments:** `channel`
- **Aliases:** `channel`

### `starboard threshold` ❌
Sets the default amount stars needed to post
- **Arguments:** `threshold`
- **Aliases:** _none_

### `starboard timestamp` ❌
Allow a timestamp to appear on a Starboard post
- **Arguments:** `setting`
- **Aliases:** _none_

### `starboard unlock` ❌
Enables/unlocks starboard from operating
- **Arguments:** _none_
- **Aliases:** `enable`, `on`

### `starboard ignore list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

---

## Clownboard (`clownboard`)

Highlight the best moments in your server to a channel.

**Count:** 15 · ✅ 0 · ⚠️ 0 · ❌ 15

### `clownboard` ❌
Showcase the worst messages in your server
- **Arguments:** _none_
- **Aliases:** `clown`

### `clownboard attachments` ❌
Allow attachments to appear on a Clownboard post
- **Arguments:** `setting`
- **Aliases:** _none_

### `clownboard color` ❌
Set default color for clownboard posts
- **Arguments:** `color`
- **Aliases:** _none_

### `clownboard config` ❌
View the settings for clownboard in guild
- **Arguments:** _none_
- **Aliases:** `configuration`

### `clownboard emoji` ❌
Sets the emoji that triggers the clownboard messages
- **Arguments:** `emoji`
- **Aliases:** _none_

### `clownboard ignore` ❌
- **Arguments:** `channel_or_member_or_role`
- **Aliases:** _none_

### `clownboard jumpurl` ❌
Allow the jump URL to appear on a Clownboard post
- **Arguments:** `setting`
- **Aliases:** _none_

### `clownboard lock` ❌
Disables/locks clownboard from operating
- **Arguments:** _none_
- **Aliases:** `disable`, `off`

### `clownboard reset` ❌
Resets guild's configuration for clownboard
- **Arguments:** _none_
- **Aliases:** `delete`, `del`

### `clownboard selfstar` ❌
Allow a author to star their own message
- **Arguments:** `setting`
- **Aliases:** _none_

### `clownboard set` ❌
Sets the channel where clownboard messages will be sent to
- **Arguments:** `channel`
- **Aliases:** `channel`

### `clownboard threshold` ❌
Sets the default amount clowns needed to post
- **Arguments:** `threshold`
- **Aliases:** _none_

### `clownboard timestamp` ❌
Allow a timestamp to appear on a Clownboard post
- **Arguments:** `setting`
- **Aliases:** _none_

### `clownboard unlock` ❌
Enables/unlocks clownboard from operating
- **Arguments:** _none_
- **Aliases:** `enable`, `on`

### `clownboard ignore list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

---

## Roleplay (`roleplay`)

Interactive roleplay commands for members in your server.

**Count:** 63 · ✅ 0 · ⚠️ 0 · ❌ 63

### `airkiss` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `angrystare` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `bite` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `bleh` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `brofist` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `celebrate` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `cheers` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `clap` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `confused` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `cool` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `cry` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `cuddle` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `dance` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `drool` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `evillaugh` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `facepalm` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `handhold` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `happy` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `headbang` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `hug` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `kiss` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `laugh` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `lick` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `love` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `mad` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `nervous` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `nom` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `nuzzle` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `nyah` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `pat` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `peek` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `pinch` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `poke` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `pout` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `punch` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `roleplay` ❌
- **Arguments:** `option`
- **Aliases:** _none_

### `sad` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `scared` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `shout` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `shrug` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `shy` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `sigh` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `sip` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `slap` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `sleep` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `slowclap` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `smack` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `smile` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `smug` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `sneeze` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `sorry` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `stare` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `surprised` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `sweat` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `thumbsup` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `tickle` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `tired` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `wave` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `wink` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `woah` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `yawn` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `yay` ❌
- **Arguments:** `member`
- **Aliases:** _none_

### `yes` ❌
- **Arguments:** `member`
- **Aliases:** _none_

---

## Manipulation (`manipulation`)

Image manipulation for photos, videos and GIFs.

**Count:** 36 · ✅ 0 · ⚠️ 0 · ❌ 36

### `media` ❌
- **Arguments:** _none_
- **Aliases:** `m`

### `media billboard` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media bloom` ❌
- **Arguments:** `url_or_attachment`, `radius`, `brightness`, `sharpness`
- **Aliases:** _none_

### `media blur` ❌
- **Arguments:** `url_or_attachment`, `strength`
- **Aliases:** _none_

### `media book` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media caption` ❌
- **Arguments:** `url_or_attachment`, `text`
- **Aliases:** _none_

### `media circuitboard` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media deepfry` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media fisheye` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media flag` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media flag2` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media fortune` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** `fortunecookie`

### `media gifmagik` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media grayscale` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media heart` ❌
- **Arguments:** `url_or_attachment`, `text`
- **Aliases:** `heartlocket`

### `media invert` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media magik` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media meme` ❌
- **Arguments:** `url_or_attachment`, `top`, `bottom`
- **Aliases:** _none_

### `media motivate` ❌
- **Arguments:** `url_or_attachment`, `top`, `bottom`
- **Aliases:** _none_

### `media neon` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media pixelate` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media rainbow` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media reverse` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media rubiks` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media scramble` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media speechbubble` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** `speech`, `bubble`

### `media speed` ❌
- **Arguments:** `url_or_attachment`, `multiplier`
- **Aliases:** _none_

### `media spin` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media spread` ❌
- **Arguments:** `url_or_attachment`, `strength`
- **Aliases:** _none_

### `media swirl` ❌
- **Arguments:** `url_or_attachment`, `strength`
- **Aliases:** _none_

### `media tattoo` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media toaster` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media valentine` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media wormhole` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media zoom` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

### `media zoomblur` ❌
- **Arguments:** `url_or_attachment`
- **Aliases:** _none_

---

## Counters (`counters`)

Show off interesting number statistics as channels in your server.

**Count:** 5 · ✅ 0 · ⚠️ 0 · ❌ 5

### `counter` ❌
Create counters for everybody to see
- **Arguments:** _none_
- **Aliases:** _none_

### `counter add` ❌
Create channel counter
- **Arguments:** `option`, `channel`
- **Aliases:** _none_

### `counter list` ❌
List every counter available in this server
- **Arguments:** _none_
- **Aliases:** _none_

### `counter remove` ❌
Remove a channel counter
- **Arguments:** `channel`, `action`
- **Aliases:** _none_

### `counter set` ❌
Set a channel counter to an existing channel
- **Arguments:** `channel`, `option`
- **Aliases:** _none_

---

## Timers (`timers`)

Schedule messages to be sent in channels at an interval.

**Count:** 6 · ✅ 0 · ⚠️ 0 · ❌ 6

### `timer` ❌
Post repeating messages in your server
- **Arguments:** _none_
- **Aliases:** `automessage`, `scheduletimers`, `automsg`

### `timer activity` ❌
Enable or disable channel activity requirement
- **Arguments:** `setting`
- **Aliases:** `ignoreactivity`

### `timer add` ❌
Add repeating message to a channel
- **Arguments:** `channel`, `interval`, `message`
- **Aliases:** `create`, `send`

### `timer list` ❌
View all auto messages in your server
- **Arguments:** _none_
- **Aliases:** _none_

### `timer remove` ❌
Remove repeating message from a channel
- **Arguments:** `channel`
- **Aliases:** `delete`, `del`

### `timer view` ❌
Preview a channel's auto message
- **Arguments:** `channel`
- **Aliases:** `check`

---

## Bump Reminder (`bumpreminder`)

Receive reminders to advertise your server on DISBOARD.

**Count:** 9 · ✅ 0 · ⚠️ 0 · ❌ 9

### `bumpreminder` ❌
Get reminders to /bump your server on Disboard!
- **Arguments:** `setting`
- **Aliases:** `bumpremind`, `bprm`

### `bumpreminder autoclean` ❌
Automatically delete messages that aren't /bump
- **Arguments:** `choice`
- **Aliases:** _none_

### `bumpreminder autolock` ❌
Lock channel until ready to use /bump
- **Arguments:** `choice`
- **Aliases:** _none_

### `bumpreminder channel` ❌
Set Bump Reminder channel for the server
- **Arguments:** `channel`
- **Aliases:** `set`

### `bumpreminder config` ❌
View server configuration for Bump Reminder
- **Arguments:** _none_
- **Aliases:** `settings`

### `bumpreminder message` ❌
Set the reminder message to run /bump
- **Arguments:** `message`
- **Aliases:** _none_

### `bumpreminder thankyou` ❌
Set the 'Thank You' message for successfully running /bump
- **Arguments:** `message`
- **Aliases:** `ty`

### `bumpreminder message view` ❌
View the current remind message
- **Arguments:** _none_
- **Aliases:** `check`

### `bumpreminder thankyou view` ❌
View the current Thank You message
- **Arguments:** _none_
- **Aliases:** `check`

---

## Fortnite (`fortnite`)

View the daily fortnite shop or set up reminders for new items.

**Count:** 8 · ✅ 0 · ⚠️ 0 · ❌ 8

### `fortnite` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `fortniteshop` ❌
Show daily shop rotation
- **Arguments:** _none_
- **Aliases:** `fnshop`

### `fortnite item` ❌
- **Arguments:** `name`
- **Aliases:** _none_

### `fortnite shop` ❌
- **Arguments:** `channel`
- **Aliases:** _none_

### `fortnite watch` ❌
- **Arguments:** `item`
- **Aliases:** _none_

### `fortnite shop ping` ❌
- **Arguments:** `role`
- **Aliases:** _none_

### `fortnite shop voting` ❌
- **Arguments:** `setting`
- **Aliases:** _none_

### `fortnite watch list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

---

## Crypto (`crypto`)

Various cryptocurrency commands and confirmation reminders for Bitcoin payments.

**Count:** 4 · ✅ 0 · ⚠️ 0 · ❌ 4

### `crypto` ❌
Checks the current price of the specified cryptocurrency
- **Arguments:** `crypto`, `cur`
- **Aliases:** `cryptocurrency`

### `gas` ❌
View the current gas prices
- **Arguments:** _none_
- **Aliases:** _none_

### `subscribe` ❌
Subscribe to a bitcoin transaction for one confirmation
- **Arguments:** `hash`
- **Aliases:** `sub`

### `transaction` ❌
Get information about a BTC, LTC or ETH transaction
- **Arguments:** `hash`
- **Aliases:** `txid`, `tx`

---

## Instagram (`instagram`)

Receive notifications in your server for new posts & stories.

**Count:** 9 · ✅ 0 · ⚠️ 0 · ❌ 9

### `instagram` ❌
Gets profile information on the given Instagram user
- **Arguments:** `username`
- **Aliases:** `ig`, `insta`

### `instagram add` ❌
Create a new feed for a user
- **Arguments:** `channel`, `username`
- **Aliases:** _none_

### `instagram highlights` ❌
- **Arguments:** `username`
- **Aliases:** `hl`

### `instagram list` ❌
List all Instagram user feeds
- **Arguments:** _none_
- **Aliases:** _none_

### `instagram message` ❌
Set a message for new posts
- **Arguments:** `username`, `message`
- **Aliases:** _none_

### `instagram remove` ❌
Removes an existing feed for a user
- **Arguments:** `channel`, `username`
- **Aliases:** _none_

### `instagram stories` ❌
Turn on or off new Instagram story posts
- **Arguments:** `username`, `setting`
- **Aliases:** _none_

### `instagram story` ❌
- **Arguments:** `username`
- **Aliases:** `s`

### `instagram message view` ❌
View Instagram message for new posts
- **Arguments:** `username`
- **Aliases:** `check`

---

## X (`twitter`)

Receive notifications in your server for new tweets.

**Count:** 7 · ✅ 0 · ⚠️ 0 · ❌ 7

### `twitter` ❌
Gets profile information on the given Twitter user
- **Arguments:** `handle`
- **Aliases:** `tw`, `twit`, `x`

### `twitter add` ❌
Create feed for new tweets from a user
- **Arguments:** `channel`, `handle`
- **Aliases:** _none_

### `twitter list` ❌
View list of every Twitter feed
- **Arguments:** _none_
- **Aliases:** _none_

### `twitter message` ❌
Set a message for new tweets
- **Arguments:** `handle`, `message`
- **Aliases:** _none_

### `twitter remove` ❌
Remove feed for new tweets
- **Arguments:** `channel`, `handle`
- **Aliases:** `del`, `delete`

### `twitter retweets` ❌
Enable or disable retweets for a user
- **Arguments:** `channel`, `handle`, `setting`
- **Aliases:** _none_

### `twitter message view` ❌
View Twitter message for new tweets
- **Arguments:** `handle`
- **Aliases:** `check`

---

## TikTok (`tiktok`)

Receive notifications in your server for new TikToks.

**Count:** 7 · ✅ 0 · ⚠️ 0 · ❌ 7

### `tiktok` ❌
Gets profile information on the given TikTok user
- **Arguments:** `username`
- **Aliases:** `tt`

### `tiktok add` ❌
Create a new feed for a user
- **Arguments:** `channel`, `username`
- **Aliases:** _none_

### `tiktok list` ❌
List all TikTok user feeds
- **Arguments:** _none_
- **Aliases:** _none_

### `tiktok live` ❌
Toggle live notifications for a user
- **Arguments:** `username`, `setting`
- **Aliases:** _none_

### `tiktok message` ❌
Set a message for new posts
- **Arguments:** `username`, `message`
- **Aliases:** _none_

### `tiktok remove` ❌
Removes an existing feed for a user
- **Arguments:** `channel`, `username`
- **Aliases:** _none_

### `tiktok message view` ❌
View TikTok message for new posts
- **Arguments:** `username`
- **Aliases:** `check`

---

## Youtube (`youtube`)

Receive notifications in your server for new uploads.

**Count:** 6 · ✅ 0 · ⚠️ 0 · ❌ 6

### `youtube` ❌
Search YouTube for video results
- **Arguments:** `search`
- **Aliases:** `yt`

### `youtube add` ❌
Enable post notifications for a channel
- **Arguments:** `channel`, `channel_url`
- **Aliases:** _none_

### `youtube list` ❌
View all YouTube post notifications
- **Arguments:** _none_
- **Aliases:** _none_

### `youtube message` ❌
Customize the message for YouTube notifications
- **Arguments:** `channel_url`, `message`
- **Aliases:** _none_

### `youtube remove` ❌
Disable post notifications for a channel
- **Arguments:** `channel`, `channel_url`
- **Aliases:** `del`, `delete`

### `youtube message view` ❌
View YouTube message for new posts
- **Arguments:** `channel_url`
- **Aliases:** `check`

---

## Soundcloud (`soundcloud`)

Receive notifications in your server for new tracks.

**Count:** 6 · ✅ 0 · ⚠️ 0 · ❌ 6

### `soundcloud` ❌
- **Arguments:** `query`
- **Aliases:** `sc`

### `soundcloud add` ❌
- **Arguments:** `channel`, `username`
- **Aliases:** _none_

### `soundcloud list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `soundcloud message` ❌
- **Arguments:** `username`, `message`
- **Aliases:** _none_

### `soundcloud remove` ❌
- **Arguments:** `channel`, `username`
- **Aliases:** `delete`, `del`

### `soundcloud message view` ❌
- **Arguments:** `username`
- **Aliases:** `check`

---

## Twitch (`twitch`)

Receive notifications in your server for new streams.

**Count:** 6 · ✅ 0 · ⚠️ 0 · ❌ 6

### `twitch` ❌
Check a Twitch profile or set up stream notifications
- **Arguments:** `username`
- **Aliases:** _none_

### `twitch add` ❌
Add stream notifications to channel
- **Arguments:** `channel`, `streamer`
- **Aliases:** _none_

### `twitch list` ❌
View all Twitch stream notifications
- **Arguments:** _none_
- **Aliases:** _none_

### `twitch message` ❌
Set a message when for Twitch notifications
- **Arguments:** `streamer`, `message`
- **Aliases:** _none_

### `twitch remove` ❌
Remove stream notifications from a channel
- **Arguments:** `channel`, `streamer`
- **Aliases:** `delete`, `del`

### `twitch message view` ❌
View Twitch message for new streams
- **Arguments:** `streamer`
- **Aliases:** `check`

---

## Reddit (`reddit`)

Receive notifications in your server for new posts.

**Count:** 7 · ✅ 0 · ⚠️ 0 · ❌ 7

### `subreddit` ❌
- **Arguments:** `name`
- **Aliases:** `reddit`

### `subreddit add` ❌
Stream a subreddit's posts into a channel
- **Arguments:** `channel`, `name`
- **Aliases:** `follow`

### `subreddit color` ❌
Set default embed color for posts
- **Arguments:** `color`
- **Aliases:** _none_

### `subreddit list` ❌
View a list of every existing subreddit stream
- **Arguments:** _none_
- **Aliases:** _none_

### `subreddit message` ❌
Set a message when subreddit posts are sent
- **Arguments:** `name`, `message`
- **Aliases:** _none_

### `subreddit remove` ❌
Remove a stream for a subreddit from a channel
- **Arguments:** `channel`, `name`
- **Aliases:** `delete`, `del`

### `subreddit message view` ❌
View current subreddit message
- **Arguments:** `name`
- **Aliases:** `check`

---

## Pinterest (`pinterest`)

Receive notifications in your server for new pins.

**Count:** 8 · ✅ 0 · ⚠️ 0 · ❌ 8

### `pinterest` ❌
- **Arguments:** `username`
- **Aliases:** _none_

### `pinterestsearch` ❌
- **Arguments:** `url`
- **Aliases:** `pinsearch`

### `pinterest add` ❌
- **Arguments:** `channel`, `username`
- **Aliases:** _none_

### `pinterest embeds` ❌
- **Arguments:** `channel`, `setting`
- **Aliases:** _none_

### `pinterest list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `pinterest message` ❌
- **Arguments:** `username`, `message`
- **Aliases:** _none_

### `pinterest remove` ❌
- **Arguments:** `channel`, `username`
- **Aliases:** `del`, `delete`

### `pinterest message view` ❌
- **Arguments:** `username`
- **Aliases:** `check`

---

## Kick (`kick`)

Receive notifications in your server for new streams.

**Count:** 6 · ✅ 0 · ⚠️ 0 · ❌ 6

### `kick` ❌
- **Arguments:** `member`, `reason`
- **Aliases:** _none_

### `kick add` ❌
- **Arguments:** `channel`, `username`
- **Aliases:** _none_

### `kick list` ❌
- **Arguments:** _none_
- **Aliases:** _none_

### `kick message` ❌
- **Arguments:** `username`, `message`
- **Aliases:** _none_

### `kick remove` ❌
- **Arguments:** `channel`, `username`
- **Aliases:** `delete`, `del`

### `kick message view` ❌
- **Arguments:** `username`
- **Aliases:** `check`

---

## Permission levels (Bleed labels)

Bleed docs use permission labels such as:

- **None** — no special permissions
- **Booster Only** — server booster
- **Staff Only** — staff role (`settings staff`)
- **Tier 2 Only** — Bleed premium tier
- **Manage Messages / Channels / Guild / Webhooks / Roles** — Discord permissions
- **Administrator** — Discord administrator
- **Server Owner** — guild owner

Map these to Poise `required_permissions`, custom staff/booster checks, or owner checks when implementing.

---

## Source and refresh

1. Catalog URL: `https://bucket.bleed.bot/commands.json`
2. Website UI: `https://bleed.bot/commands`
3. Related docs: `https://docs.bleed.bot`
4. This inventory was generated from the JSON snapshot dated **2026-04-04**.
5. Re-fetch the JSON and regenerate this section when Bleed adds or renames commands.

