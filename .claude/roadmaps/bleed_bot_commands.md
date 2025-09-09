# Bleed Bot Commands Reference

**Legend:**
- ✅ = Implemented
- ⚠️ = Partially Implemented
- ❌ = Not Implemented

## Implementation Summary

### Core Bot Commands
- **Implemented:** 5/5 commands (100%)
  - ✅ `ping`
  - ✅ `help`
  - ✅ `info`
  - ✅ `cache_status` (dev only)
  - ✅ `test_responses` (dev only)

### Prefix Commands
- **Implemented:** 4/5 commands (80%)
  - ✅ `prefix` (view)
  - ✅ `prefix set`
  - ✅ `prefix remove`
  - ✅ `prefix reset` (additional)
  - ❌ `prefix self` (Tier 2 feature)

### Booster Role Commands
- **Implemented:** 24/24 commands (100%)
  - ✅ `boosterrole` (parent command)
  - ✅ `boosterrole color`
  - ✅ `boosterrole dominant`
  - ✅ `boosterrole link`
  - ✅ `boosterrole filter` (parent command)
  - ✅ `boosterrole filter add`
  - ✅ `boosterrole filter remove`
  - ✅ `boosterrole filter list`
  - ✅ `boosterrole list`
  - ✅ `boosterrole cleanup`
  - ✅ `boosterrole limit`
  - ✅ `boosterrole rename`
  - ✅ `boosterrole award` (with set/unset/view subcommands)
  - ✅ `boosterrole icon`
  - ✅ `boosterrole remove`
  - ✅ `boosterrole share` (with role/remove/max/list/limit subcommands)
  - ✅ `boosterrole base`
  - ✅ `boosterrole random`

### Settings Commands
- **Implemented:** 7/7 high-priority commands (100%)
  - ✅ `settings` (parent command)
  - ✅ `settings config`
  - ✅ `settings staff` (with add/remove/list subcommands)
  - ✅ `settings autonick` (with set/disable/view subcommands)
  - ✅ `settings joinlogs` (with set/disable/test subcommands)
  - ✅ `settings premiumrole` (with set/disable/view subcommands)
  - ✅ `settings baserole` (via boosterrole base)

### Total Implementation Status
- **Overall Progress:** 40/41 total commands (98%)

### Notes
- Some permissions differ from original Bleed bot (e.g., using `Manage Guild` instead of `Administrator`)
- Additional aliases and features added to some commands
- Database schema and caching implemented for persistence

## Prefix Commands

### `prefix` ✅
View guild prefix
- **Arguments:** none
- **Permissions:** none
- **Status:** Implemented (defaults to view subcommand)

### `prefix set` ✅
Set command prefix for server
- **Arguments:** prefix
- **Permissions:** ~~Administrator~~ **Manage Guild** (implemented differently)
- **Status:** Implemented with validation (1-5 chars, no @ or #)

### `prefix self` ❌
Set personal prefix across all servers with bleed
- **Arguments:** prefix
- **Permissions:** Tier 2 Only
- **Status:** Not implemented

### `prefix remove` ✅
Remove command prefix for server
- **Arguments:** none
- **Permissions:** ~~Administrator~~ **Manage Guild** (implemented differently)
- **Status:** Implemented (reverts to default prefix)

### `prefix reset` ✅
Reset the guild prefix to default
- **Arguments:** none
- **Permissions:** Manage Guild
- **Status:** Implemented (same as remove)
- **Note:** Not in original Bleed bot documentation, added in our implementation

---

## Booster Role Commands

### `boosterrole` ⚠️
Create your own booster role
- **Arguments:** color, second color, name
- **Permissions:** Booster Only
- **Status:** Parent command implemented (shows help for subcommands)

### `boosterrole color` ✅
Get your own custom booster color role
- **Arguments:** color, ~~second color~~ (optional), name
- **Permissions:** Booster Only
- **Status:** Implemented with gradient support

### `boosterrole dominant` ✅
Set booster roles color to the most dominant color in avatar
- **Arguments:** none
- **Permissions:** Booster Only
- **Status:** Implemented with dual color extraction
- **Aliases:** `dom`, `avatar`, `auto`

### `boosterrole link` ✅
Link an existing role to be a booster role
- **Arguments:** member, role
- **Permissions:** Manage Guild
- **Status:** Implemented

### `boosterrole filter` ✅
Blacklist words for booster role names
- **Arguments:** none (shows help for subcommands)
- **Permissions:** Manage Guild
- **Status:** Implemented as parent command (subcommands: add, remove, list)

### `boosterrole filter add` ✅
Add a word to the blacklist for booster role names
- **Arguments:** word
- **Permissions:** Manage Guild
- **Status:** Implemented with pattern matching and verification

### `boosterrole filter remove` ✅
Remove a word from the blacklist for booster role names
- **Arguments:** word
- **Permissions:** Manage Guild
- **Status:** Implemented

### `boosterrole filter list` ✅
View blacklisted words for booster role names
- **Arguments:** none
- **Permissions:** Manage Guild
- **Status:** Implemented with pagination support

### `boosterrole list` ✅
View all booster roles
- **Arguments:** none
- **Permissions:** Manage Guild
- **Status:** Implemented

### `boosterrole cleanup` ✅
Clean up unused booster roles
- **Arguments:** optional dry_run flag
- **Permissions:** Manage Guild
- **Status:** Implemented with dry-run option for safe testing

### `boosterrole limit` ✅
Set limit for booster roles
- **Arguments:** optional max limit (view if not provided)
- **Permissions:** Manage Guild
- **Status:** Implemented with view/set functionality

### `boosterrole rename` ✅
Edit your booster roles name
- **Arguments:** new name
- **Permissions:** Booster Only
- **Status:** Implemented with 1-hour cooldown and blacklist checking

### `boosterrole award` ✅
Reward a member a specific role upon boost
- **Arguments:** subcommands (set/unset/view)
- **Permissions:** Manage Guild, Roles
- **Status:** Implemented as parent command with subcommands

### `boosterrole award set` ✅
Set the reward role for new boosters
- **Arguments:** role
- **Permissions:** Manage Guild, Roles
- **Status:** Implemented with role validation

### `boosterrole award unset` ✅
Remove the reward role
- **Arguments:** none
- **Permissions:** Manage Guild, Roles
- **Status:** Implemented

### `boosterrole award view` ✅
View the current award role
- **Arguments:** none
- **Permissions:** Manage Guild, Roles
- **Status:** Implemented

### `boosterrole icon` ✅
Set an icon for booster role
- **Arguments:** url
- **Permissions:** Booster Only
- **Status:** Implemented with URL validation and bot permission checks

### `boosterrole remove` ✅
Remove custom color booster role
- **Arguments:** none
- **Permissions:** Booster Only
- **Status:** Implemented with confirmation and cleanup

### `boosterrole share` ✅
Share your booster role with others (parent command)
- **Arguments:** subcommands (role/remove/max/list/limit)
- **Permissions:** varies by subcommand
- **Status:** Implemented as parent command with comprehensive subcommands

### `boosterrole share remove` ✅
Remove yourself from a shared booster role
- **Arguments:** role
- **Permissions:** none
- **Status:** Implemented as subcommand of `boosterrole share`

### `boosterrole share max` ✅
Limit how many members can be in a booster role
- **Arguments:** number
- **Permissions:** Manage Guild
- **Status:** Implemented as subcommand of `boosterrole share`

### `boosterrole share list` ✅
List all members in your booster role
- **Arguments:** none
- **Permissions:** Manage Guild
- **Status:** Implemented as subcommand of `boosterrole share`

### `boosterrole share limit` ✅
Limit how many booster roles a member can have
- **Arguments:** number
- **Permissions:** Manage Guild
- **Status:** Implemented as subcommand of `boosterrole share`

### `boosterrole base` ✅
Set the base role for where boost roles will go under
- **Arguments:** role
- **Permissions:** Manage Guild
- **Status:** Implemented with role hierarchy validation

### `boosterrole random` ✅
Set a booster role with a random hex code
- **Arguments:** none
- **Permissions:** Booster Only
- **Status:** Implemented with random color generation and role creation

---

## Additional Implemented Commands (Not in Original Bleed Bot)

### `ping` ✅
Test bot responsiveness
- **Arguments:** none
- **Permissions:** none
- **Status:** Implemented with latency measurement

### `help` ✅
Show bot help information
- **Arguments:** optional command name
- **Permissions:** none
- **Status:** Implemented with comprehensive command information

### `info` ✅
Display bot information and statistics
- **Arguments:** none
- **Permissions:** none
- **Status:** Implemented with system stats and version info

### `cache_status` ✅
View database cache statistics (Development only)
- **Arguments:** none
- **Permissions:** Administrator
- **Status:** Implemented (debug builds only)

### `test_responses` ✅
Development testing command (Development only)
- **Arguments:** subcommands for various response types
- **Permissions:** Administrator
- **Status:** Implemented (debug builds only)

---

## Settings Commands

### Implementation Status & Priority

**Priority Levels:**
- 🔴 **High Priority** - Core functionality, implementable with existing infrastructure
- 🟡 **Medium Priority** - Useful features, require moderate development
- 🟢 **Low Priority** - Nice-to-have features or require significant new systems
- ⚫ **Not Implementable** - Requires missing systems (moderation, music, etc.)

### Settings Commands Analysis

#### ✅ **IMPLEMENTED** - Settings Command Suite

### `settings` ✅ **IMPLEMENTED**
Server configuration (parent command)
- **Arguments:** none
- **Permissions:** Manage Guild
- **Status:** Fully implemented with subcommands overview
- **Database:** Uses existing guild settings tables
- **Location:** `src/commands/settings/mod.rs`

### `settings config` ✅ **IMPLEMENTED**
View settings configuration for guild
- **Arguments:** none
- **Permissions:** None (view only)
- **Status:** Displays all guild configurations in an embed with concurrent queries
- **Database:** Queries all settings tables (staff roles, auto nicknames, join logs, premium role)
- **Location:** `src/commands/settings/config.rs`

### `settings baserole` ✅ **ALREADY IMPLEMENTED**
Set the base role for where boost roles will go under
- **Arguments:** role
- **Permissions:** Manage Guild
- **Status:** Already exists as `boosterrole base` command
- **Database:** Uses `guild_booster_base_roles` table

### `settings staff` ✅ **IMPLEMENTED**
Manage staff roles
- **Arguments:** subcommands (add/remove/list)
- **Permissions:** Manage Guild
- **Status:** Fully implemented with add, remove, and list functionality
- **Database:** Uses `guild_staff_roles` table
- **Location:** `src/commands/settings/staff.rs`
- **Features:**
  - `settings staff add` - Add a staff role
  - `settings staff remove` - Remove a staff role
  - `settings staff list` - View all staff roles

### `settings autonick` ✅ **IMPLEMENTED**
Set a nickname to be assigned to members when they join
- **Arguments:** subcommands (set/disable/view)
- **Permissions:** Manage Guild
- **Status:** Fully implemented with member join event handler
- **Database:** Uses `guild_auto_nicknames` table
- **Location:** `src/commands/settings/autonick.rs`
- **Features:**
  - `settings autonick set` - Set auto-nickname template with {username} placeholder
  - `settings autonick disable` - Disable auto-nickname
  - `settings autonick view` - View current template with preview
  - Automatic application on member join via `MemberHandler`

### `settings joinlogs` ✅ **IMPLEMENTED**
Set a channel to log join/leaves in a server
- **Arguments:** subcommands (set/disable/test)
- **Permissions:** Manage Guild
- **Status:** Fully implemented with member join/leave event handlers
- **Database:** Uses `guild_join_log_channels` table
- **Location:** `src/commands/settings/joinlogs.rs`
- **Features:**
  - `settings joinlogs set` - Set logging channel with permission validation
  - `settings joinlogs disable` - Disable join/leave logging
  - `settings joinlogs test` - Send test message to configured channel
  - Automatic logging via `MemberHandler` with formatted embeds

### `settings premiumrole` ✅ **IMPLEMENTED**
Set the Premium Members role for Server Subscriptions
- **Arguments:** subcommands (set/disable/view)
- **Permissions:** Manage Guild
- **Status:** Fully implemented with role hierarchy validation
- **Database:** Uses `guild_premium_roles` table
- **Location:** `src/commands/settings/premiumrole.rs`
- **Features:**
  - `settings premiumrole set` - Set premium role with hierarchy checks
  - `settings premiumrole disable` - Remove premium role designation
  - `settings premiumrole view` - View current premium role

#### ⚫ **Not Currently Implementable - Missing Core Systems**

### `settings rmuted` ❌ **NOT IMPLEMENTABLE**
Set the reaction muted role
- **Arguments:** role
- **Permissions:** Manage Guild
- **Status:** Requires moderation system

### `settings jail` ❌ **NOT IMPLEMENTABLE**
Set the jail channel
- **Arguments:** channel
- **Permissions:** Manage Guild
- **Status:** Requires moderation/jail system

### `settings reset` ❌ **NOT IMPLEMENTABLE**
Reset moderation configuration
- **Arguments:** none
- **Permissions:** Administrator
- **Status:** Requires moderation system

### `settings jailmsg` ❌ **NOT IMPLEMENTABLE**
Set a custom jail message
- **Arguments:** message
- **Permissions:** Manage Guild
- **Status:** Requires moderation/jail system

### `settings resetcases` ❌ **NOT IMPLEMENTABLE**
Reset jail-log cases
- **Arguments:** none
- **Permissions:** Administrator
- **Status:** Requires moderation case system

### `settings autoplay` ❌ **NOT IMPLEMENTABLE**
Set auto play for music
- **Arguments:** setting
- **Permissions:** Manage Guild
- **Status:** Requires music player system

### `settings googlesafetylevel` ❌ **NOT IMPLEMENTABLE**
Enable or disable safety level for Google commands
- **Arguments:** yes or no
- **Permissions:** Manage Guild
- **Status:** Requires Google search integration

### `settings imuted` ❌ **NOT IMPLEMENTABLE**
Set the image muted role
- **Arguments:** role
- **Permissions:** Manage Guild, Roles
- **Status:** Requires moderation system

### `settings modlog` ❌ **NOT IMPLEMENTABLE**
Set mod logs for punishments in guild
- **Arguments:** channel
- **Permissions:** Manage Guild
- **Status:** Requires moderation system

### `settings dj` ❌ **NOT IMPLEMENTABLE**
Set DJ role for music player
- **Arguments:** role
- **Permissions:** Manage Guild
- **Status:** Requires music player system

### `settings muted` ❌ **NOT IMPLEMENTABLE**
Set the text muted role
- **Arguments:** role
- **Permissions:** Manage Guild, Roles
- **Status:** Requires moderation system

### `settings disablecustomfms` ❌ **NOT IMPLEMENTABLE**
Disable custom Now Playing commands
- **Arguments:** yes or no
- **Permissions:** Manage Channels
- **Status:** Requires Last.fm integration

### `settings jailroles` ❌ **NOT IMPLEMENTABLE**
Enable or disable removal of roles for jail
- **Arguments:** yes or no
- **Permissions:** Manage Guild
- **Status:** Requires moderation/jail system

### Implementation Roadmap

#### Phase 1 - Immediate Implementation
1. `settings` - Parent command
2. `settings config` - View all settings
3. `settings staff` - Set staff roles
4. `settings staff list` - List staff roles

#### Phase 2 - Event Handler Implementation
1. `settings autonick` - Auto nickname on join
2. `settings joinlogs` - Join/leave logging
3. `settings premiumrole` - Premium member tracking

#### Phase 3 - Future Systems (Not Currently Planned)
- Moderation system commands (muted, jail, modlog, etc.)
- Music system commands (autoplay, dj)
- Integration commands (googlesafetylevel, disablecustomfms)

---

## Boost Messages

### `boosts`
Set up boost messages in one or multiple channels
- **Arguments:** none
- **Permissions:** Manage Guild

### `boosts add`
Add a boost message to a channel
- **Arguments:** channel, message
- **Permissions:** Manage Guild

### `boosts variables`
View all available variables for boost messages
- **Arguments:** none
- **Permissions:** Manage Guild

### `boosts view`
View a boost message for a channel
- **Arguments:** channel
- **Permissions:** Manage Guild

### `boosts list`
View all boost messages
- **Arguments:** none
- **Permissions:** Manage Guild

### `boosts remove`
Remove a boost message from a channel
- **Arguments:** channel
- **Permissions:** Manage Guild

---

## Alias Commands

### `alias`
Create your own shortcuts for commands
- **Arguments:** none
- **Permissions:** Manage Guild

### `alias add`
Create an alias for command
- **Arguments:** shortcut, command
- **Permissions:** Manage Guild

### `alias remove`
Remove an alias for command
- **Arguments:** shortcut
- **Permissions:** Manage Guild

### `alias list`
List every alias for all commands
- **Arguments:** none
- **Permissions:** Manage Guild

### `alias reset`
Reset every alias for all commands
- **Arguments:** none
- **Permissions:** Manage Guild

### `alias view`
View command execution for alias
- **Arguments:** shortcut
- **Permissions:** Manage Guild

### `alias removeall`
Remove an alias for command
- **Arguments:** command
- **Permissions:** Manage Guild

---

## Sticky Messages

### `stickymessage`
Set up a sticky message in one or multiple channels
- **Arguments:** none
- **Permissions:** Manage Guild

### `stickymessage list`
View all sticky messages
- **Arguments:** none
- **Permissions:** Manage Guild

### `stickymessage remove`
Remove a sticky message from a channel
- **Arguments:** channel
- **Permissions:** Manage Guild

### `stickymessage view`
View the sticky message for a channel
- **Arguments:** channel
- **Permissions:** Manage Guild

### `stickymessage add`
Add a sticky message to a channel
- **Arguments:** channel, message
- **Permissions:** Manage Guild

---

## Welcome Messages

### `welcome`
Set up a welcome message in one or multiple channels
- **Arguments:** none
- **Permissions:** Manage Guild

### `welcome list`
View all welcome messages
- **Arguments:** none
- **Permissions:** Manage Guild

### `welcome add`
Add a welcome message for a channel
- **Arguments:** channel, message
- **Permissions:** Manage Guild

### `welcome remove`
Remove a welcome message from a channel
- **Arguments:** channel
- **Permissions:** Manage Guild

### `welcome variables`
View all available variables for welcome messages
- **Arguments:** none
- **Permissions:** Manage Guild

### `welcome view`
View welcome message for a channel
- **Arguments:** channel
- **Permissions:** Manage Guild

---

## Goodbye Messages

### `goodbye`
Set up a goodbye message in one or multiple channels
- **Arguments:** none
- **Permissions:** Manage Guild

### `goodbye view`
View goodbye message for a channel
- **Arguments:** channel
- **Permissions:** Manage Guild

### `goodbye list`
View all goodbye messages
- **Arguments:** none
- **Permissions:** Manage Guild

### `goodbye add`
Add a goodbye message for a channel
- **Arguments:** channel, message
- **Permissions:** Manage Guild

### `goodbye remove`
Remove a goodbye message from a channel
- **Arguments:** channel
- **Permissions:** Manage Guild

### `goodbye variables`
View all available variables for goodbye messages
- **Arguments:** none
- **Permissions:** Manage Guild

---

## Image Only Channels

### `imgonly`
Set up image + caption only channels
- **Arguments:** none
- **Permissions:** Manage Guild

### `imgonly remove`
Remove a gallery channel
- **Arguments:** channel
- **Permissions:** Manage Guild

### `imgonly list`
View all gallery channels
- **Arguments:** none
- **Permissions:** Manage Guild

### `imgonly add`
Set a gallery channel
- **Arguments:** channel
- **Permissions:** Manage Guild

---

## Invoke Messages (Punishment DMs/Responses)

### `invoke`
Change punishment messages for DM or command response
- **Arguments:** none
- **Permissions:** Manage Guild

### Untimeout Messages
- `invoke untimeout` - Change untimeout message for DM or command response
- `invoke untimeout message` - Change untimeout message for command response (message)
- `invoke untimeout message view` - View the untimeout message for command response
- `invoke untimeout dm` - Change untimeout message for Direct Messages (message)
- `invoke untimeout dm view` - View the untimeout message for Direct Messages

### Jail Messages
- `invoke jail` - Change jail message for DM or command response
- `invoke jail message` - Change jail message for command response (message)
- `invoke jail message view` - View the jail message for command response
- `invoke jail dm` - Change jail message for Direct Messages (message)
- `invoke jail dm view` - View the jail message for Direct Messages

### Mute Messages
- `invoke mute` - Change mute message for DM or command response
- `invoke mute dm` - Change mute message for Direct Messages (message)
- `invoke mute dm view` - View the mute message for Direct Messages
- `invoke mute message` - Change mute message for command response (message)
- `invoke mute message view` - View the mute message for command response

### Unmute Messages
- `invoke unmute` - Change unmute message for DM or command response
- `invoke unmute message` - Change unmute message for command response (message)
- `invoke unmute message view` - View the unmute message for command response
- `invoke unmute dm` - Change unmute message for Direct Messages (message)
- `invoke unmute dm view` - View the unmute message for Direct Messages

### Other Punishment Messages
- `invoke rmute` / `invoke rmute message` / `invoke rmute message view` - Reaction mute messages
- `invoke runmute` / `invoke runmute message` / `invoke runmute message view` - Reaction unmute messages
- `invoke softban` / `invoke softban dm` / `invoke softban dm view` / `invoke softban message` / `invoke softban message view` - Softban messages
- `invoke unjail` / `invoke unjail message` / `invoke unjail message view` / `invoke unjail dm` / `invoke unjail dm view` - Unjail messages
- `invoke unban` / `invoke unban message` / `invoke unban message view` / `invoke unban dm` / `invoke unban dm view` - Unban messages
- `invoke iunmute` / `invoke iunmute message` / `invoke iunmute message view` - Image unmute messages
- `invoke kick` / `invoke kick dm` / `invoke kick dm view` / `invoke kick message` / `invoke kick message view` - Kick messages
- `invoke timeout` / `invoke timeout message` / `invoke timeout message view` / `invoke timeout dm` / `invoke timeout dm view` - Timeout messages
- `invoke imute` / `invoke imute message` / `invoke imute message view` - Image mute messages
- `invoke warn` / `invoke warn dm` / `invoke warn dm view` / `invoke warn message` / `invoke warn message view` - Warn messages
- `invoke ban` / `invoke ban message` / `invoke ban message view` / `invoke ban dm` / `invoke ban dm view` - Ban messages
- `invoke hardban` / `invoke hardban message` / `invoke hardban message view` - Hardban messages
- `invoke tempban` / `invoke tempban dm` / `invoke tempban dm view` / `invoke tempban message` / `invoke tempban message view` - Tempban messages

---

## Filter Commands

### `filter`
View a variety of options to help clean chat
- **Arguments:** none
- **Permissions:** Manage Channels

### `filter snipe`
Filter snipe command from allowing certain content
- **Arguments:** type
- **Permissions:** Manage Channels

### Links Filter
- `filter links` - Delete any message that contains a link (channel, setting, parameters)
- `filter links whitelist` - Whitelist links from the links filter (channel, url)
- `filter links exempt` - Exempt roles from the links filter (role)
- `filter links exempt list` - View list of roles exempted from links filter

### Word Filter
- `filter list` - View a list of filtered words in guild
- `filter add` - Add a filtered word (word)
- `filter remove` - Remove a filtered word (word)
- `filter exempt` - Exempt roles from the word filter (role)
- `filter exempt list` - View list of roles exempted from nicknames filter
- `filter reset` - Reset all filtered words
- `filter whitelist` - Add or remove a whitelisted word (word)
- `filter wordmigrate` - Migrate your filtered words to Discords Automod

### Invites Filter
- `filter invites` - Delete any message that contains a server link (channel, setting, parameters)
- `filter invites exempt` - Exempt roles from the invite filter (role)
- `filter invites exempt list` - View list of roles exempted from invites filter

### Caps Filter
- `filter caps` - Delete messages that contain too many uppercase characters (channel, setting, parameters)
- `filter caps exempt` - Exempt roles from the caps filter (role)
- `filter caps exempt list` - View list of roles exempted from caps filter

### Mass Mention Filter
- `filter massmention` - Delete any message exceeding the threshold for user mentions (channel, setting, parameters)
- `filter massmention exempt` - Exempt roles from the mass mention filter (role)
- `filter massmention exempt list` - View list of roles exempted from massmention filter

### Emoji Filter
- `filter emoji` - Delete any message exceeding the threshold for emojis (channel, setting, parameters)
- `filter emoji exempt` - Exempt roles from the emoji filter (role)
- `filter emoji exempt list` - View list of roles exempted from emoji filter

### Spam Filter
- `filter spam` - Delete messages from users that send messages too fast (channel, setting, parameters)
- `filter spam exempt` - Exempt roles from the antispam filter (role)
- `filter spam exempt list` - View list of roles exempted from spam filter

### Spoilers Filter
- `filter spoilers` - Delete any message exceeding the threshold for spoilers (channel, setting, parameters)
- `filter spoilers exempt` - Exempt roles from the spoilers filter (role)
- `filter spoilers exempt list` - View list of roles exempted from spoilers filter

### Music Files Filter
- `filter musicfiles` - Delete any message that contains a music file (channel, setting, parameters)
- `filter musicfiles exempt` - Exempt roles from the music files filter (role)
- `filter musicfiles exempt list` - View list of roles exempted from musicfiles filter

### Regex Filter
- `filter regex` - Add or remove a regex pattern (pattern)

---

## Autoresponder Commands

### `autoresponder`
Set up automatic replies to messages matching a trigger
- **Arguments:** none
- **Permissions:** Manage Channels

### `autoresponder add`
Create a reply for a trigger word
- **Arguments:** args
- **Permissions:** Manage Channels

### `autoresponder update`
Update a reply for a trigger word
- **Arguments:** args
- **Permissions:** Manage Channels

### `autoresponder remove`
Remove a reply for a trigger word
- **Arguments:** trigger
- **Permissions:** Manage Channels

### `autoresponder reset`
Remove every auto response
- **Arguments:** none
- **Permissions:** Manage Channels

### `autoresponder variables`
View a list of available variables
- **Arguments:** none
- **Permissions:** Manage Channels

### `autoresponder exclusive`
Toggle exclusive access for an autoresponder to a role or channel
- **Arguments:** role or channel, trigger
- **Permissions:** Manage Channels

### `autoresponder exclusive list`
View a list of roles and channels that have exclusive access to an autoresponder
- **Arguments:** trigger
- **Permissions:** Manage Channels

### `autoresponder list`
View a list of auto-reply triggers in guild
- **Arguments:** none
- **Permissions:** Manage Channels

### Role Management
- `autoresponder role` - Assign or remove roles on messages matching a trigger
- `autoresponder role remove` - Add a role to be removed when an autoresponder is triggered (role, trigger)
- `autoresponder role remove list` - View roles removed upon messages matching a trigger (trigger)
- `autoresponder role add` - Add a role to be given when an autoresponder is triggered (role, trigger)
- `autoresponder role add list` - View roles assigned upon messages matching a trigger (trigger)

---

## Pagination Commands

### `pagination`
Set up multiple embeds on one message
- **Arguments:** none
- **Permissions:** Manage Messages

### `pagination delete`
Delete a pagination embed entirely
- **Arguments:** messagelink
- **Permissions:** Manage Messages

### `pagination update`
Update an existing page on pagination embed
- **Arguments:** messagelink, id, embed code
- **Permissions:** Manage Messages

### `pagination list`
View all existing pagination embeds
- **Arguments:** none
- **Permissions:** Manage Messages

### `pagination reset`
Remove every existing pagination in guild
- **Arguments:** none
- **Permissions:** Administrator

### `pagination remove`
Remove a page from a pagination embed
- **Arguments:** messagelink, id
- **Permissions:** Manage Messages

### `pagination add`
Add a page to a pagination embed
- **Arguments:** messagelink, embed code
- **Permissions:** Manage Messages

### `pagination restorereactions`
Restore reactions to an existing pagination
- **Arguments:** messagelink
- **Permissions:** Manage Messages

### `pagination set`
Set up an existing embed to be paginated
- **Arguments:** messagelink
- **Permissions:** Manage Messages

---

## Command Management

### Enable Commands
- `enablecommand` - Enable a previously disabled command in a channel (channel or member, command)
- `enablecommand all` - Enable a command in every channel (command)

### Disable Commands
- `disablecommand` - Disable a command in a channel (channel or member, command)
- `disablecommand all` - Disable a command in every channel (command)
- `disablecommand list` - View a list of disabled commands in guild

### Copy Settings
- `copydisabled` - Copy disabled modules, events, filters and commands to another channel (old channel, new channel)

### Events Management
- `enableevent` - Enable a bot event in a channel (channel, event)
- `enableevent all` - Enables a bot event in every channel (event)
- `disableevent` - Disable a bot event in a channel (channel, event)
- `disableevent list` - View a list of disabled bot events in guild
- `disableevent all` - Disable a bot event in every channel (event)

### Module Management
- `enablemodule` - Enable a module in a channel (channel, module)
- `enablemodule all` - Enables a module in every channel (module)
- `disablemodule` - Disable a module in a channel (channel, module)
- `disablemodule list` - View a list of disabled modules in guild
- `disablemodule all` - Disable a module in every channel (module)

---

## Ignore Commands

### `ignore`
No description given
- **Arguments:** member or channel
- **Permissions:** Administrator

### `ignore add`
Ignore a member or channel
- **Arguments:** member or channel
- **Permissions:** Administrator

### `ignore list`
View a list of ignored members or channels
- **Arguments:** none
- **Permissions:** Administrator

### `ignore remove`
Remove ignoring for a member or channel
- **Arguments:** member or channel
- **Permissions:** Administrator

---

## Server Customization

### `seticon`
Set a new guild icon
- **Arguments:** url
- **Permissions:** Manage Guild

### `setsplashbackground`
Set a new guild splash background
- **Arguments:** url
- **Permissions:** Manage Guild

### `setbanner`
Set a new guild banner
- **Arguments:** url
- **Permissions:** Manage Guild

---

## Pin Management

### `unpin`
Unpin a message
- **Arguments:** message
- **Permissions:** Manage Messages

### `pin`
Pin the most recent message or by URL
- **Arguments:** message
- **Permissions:** Manage Messages

### `firstmessage`
Get a link for the first message in a channel
- **Arguments:** channel
- **Permissions:** none

### Pin Archival System
- `pins` - Pin archival system commands
- `pins reset` - Reset the pin archival config
- `pins config` - View the pin archival config
- `pins channel` - Set the pin archival channel (channel)
- `pins set` - Enable or disable the pin archival system (option)
- `pins archive` - Archive the pins in the current channel
- `pins unpin` - Enable or disable the unpinning of messages during archival (option)

---

## Webhook Commands

### `webhook`
Set up webhooks in your server
- **Arguments:** none
- **Permissions:** Manage Webhooks

### `webhook lock`
Lock your webhook from being accessed by others
- **Arguments:** identifier
- **Permissions:** Manage Webhooks

### `webhook delete`
Delete webhook for a channel
- **Arguments:** identifier
- **Permissions:** Manage Webhooks

### `webhook send`
Send message to existing channel webhook
- **Arguments:** identifier, message or embed code
- **Permissions:** Manage Webhooks

### `webhook edit`
Send message to existing channel webhook
- **Arguments:** messagelink, message or embed code
- **Permissions:** Manage Webhooks

### `webhook unlock`
Unlock your webhook from being accessed by others
- **Arguments:** identifier
- **Permissions:** Manage Webhooks

### `webhook create`
Create webhook to forward messages to
- **Arguments:** name
- **Permissions:** Manage Webhooks

### `webhook list`
List all available webhooks in the server
- **Arguments:** none
- **Permissions:** none

---

## Fake Permissions

### `fakepermissions`
Set up fake permissions for role through the bot!
- **Arguments:** none
- **Permissions:** Server Owner

### `fakepermissions remove`
Remove a fake permission from a role
- **Arguments:** role, permission
- **Permissions:** Server Owner

### `fakepermissions add`
Grant a fake permission to a role
- **Arguments:** role, permission
- **Permissions:** none

### `fakepermissions reset`
Resets all fake permissions
- **Arguments:** none
- **Permissions:** Server Owner

### `fakepermissions list`
List all fake permissions
- **Arguments:** role
- **Permissions:** Server Owner

---

## Extraction Commands

### `extractstickers`
Sends all of your servers stickers in a zip file
- **Arguments:** none
- **Permissions:** Administrator

### `extractemotes`
Sends all of your servers emojis in a zip file
- **Arguments:** none
- **Permissions:** Administrator

---

## Reposter Commands

### `reposter`
Manage, enable or disable social media reposting
- **Arguments:** option
- **Permissions:** Manage Guild

### `reposter prefix`
Enable or disable bleed prefix for reposting
- **Arguments:** option
- **Permissions:** Manage Guild

### `reposter delete`
Enable or disable deletion of social media links
- **Arguments:** option
- **Permissions:** Manage Guild

### `reposter embed`
Enable or disable embed attached to media
- **Arguments:** option
- **Permissions:** Manage Guild

### `reposter strict`
Enable or disable matching links throughout messages
- **Arguments:** option
- **Permissions:** Manage Guild

### `reposter suppress`
Enable or disable suppression of context links
- **Arguments:** option
- **Permissions:** Manage Guild

---

## Suggestion System

### `suggest`
Suggest a new idea or feature to server staff
- **Arguments:** suggestion
- **Permissions:** none

### `suggest set`
Set the channel for new suggestions
- **Arguments:** channel
- **Permissions:** Manage Channels

### `suggest lock`
Disable suggestions system
- **Arguments:** none
- **Permissions:** Manage Channels

### `suggest reply`
Reply to a suggestion
- **Arguments:** id, comment
- **Permissions:** Staff Only

### `suggest config`
View suggestion system configuration
- **Arguments:** none
- **Permissions:** Manage Channels

### `suggest reactions`
Set custom reactions for new suggestions
- **Arguments:** upvote, downvote
- **Permissions:** Manage Channels

### `suggest deny`
Change a suggestion status to Denied
- **Arguments:** id
- **Permissions:** Staff Only

### `suggest threads`
Create a thread along with the suggestion message
- **Arguments:** setting
- **Permissions:** Manage Channels

### `suggest unlock`
Enable suggestions system
- **Arguments:** none
- **Permissions:** Manage Channels

### `suggest progress`
Change a suggestion status to In Progress
- **Arguments:** id
- **Permissions:** Staff Only

### `suggest reset`
Change a suggestion status to Pending
- **Arguments:** id
- **Permissions:** Staff Only

### `suggest consider`
Change a suggestion status to In Consideration
- **Arguments:** id
- **Permissions:** Staff Only

### `suggest approve`
Change a suggestion status to Approved
- **Arguments:** id
- **Permissions:** Staff Only

### `suggest review`
Enable or disable review of suggestions before displayed publicly
- **Arguments:** setting
- **Permissions:** Manage Channels

### `suggest review channel`
Set the review channel for suggestions that require approval
- **Arguments:** channel
- **Permissions:** Manage Channels

### `suggest ignore`
Prevent members or roles from creating suggestions
- **Arguments:** member or role
- **Permissions:** Manage Channels

### `suggest ignore list`
List all ignored members or roles
- **Arguments:** none
- **Permissions:** Manage Channels

---

## Badge System

### `badge`
Reward members for setting the guild tag
- **Arguments:** setting
- **Permissions:** Manage Guild

### `badge message`
Set an award message with embed code or regular text
- **Arguments:** message
- **Permissions:** Manage Guild

### `badge message view`
View current award message
- **Arguments:** none
- **Permissions:** Manage Guild

### `badge sync`
Sync guild tag member roles
- **Arguments:** none
- **Permissions:** Manage Guild

### `badge channel`
Set an award channel for new guild tag members
- **Arguments:** channel
- **Permissions:** Manage Guild

### `badge role`
Award members for applying the guild tag
- **Arguments:** none
- **Permissions:** Manage Guild

### `badge role list`
List all roles that can be awarded for applying the guild tag
- **Arguments:** none
- **Permissions:** Manage Guild

### `badge role remove`
Remove a role from the list of roles that can be awarded for applying the guild tag
- **Arguments:** role
- **Permissions:** Manage Guild

### `badge role add`
Add a role to the list of roles that can be awarded for applying the guild tag
- **Arguments:** role
- **Permissions:** Manage Guild

---

## Permission Levels

- **None** - No special permissions required
- **Booster Only** - Must be a server booster
- **Staff Only** - Must have a staff role
- **Tier 2 Only** - Requires Tier 2 subscription
- **Manage Messages** - Discord permission
- **Manage Channels** - Discord permission
- **Manage Guild** - Discord permission
- **Manage Webhooks** - Discord permission
- **Administrator** - Discord admin permission
- **Server Owner** - Must be the server owner
- **Roles** - Additional role permission requirement