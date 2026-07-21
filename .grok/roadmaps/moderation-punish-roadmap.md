# Roadmap: moderation-punish

**Suite id:** `moderation-punish`  
**Category:** moderation  
**Branch:** `feature/moderation-punish`  
**Registry paths:**  
- `commands: src/commands/moderation/`  
- `data: src/data/models/moderation.rs` (reuse F1; no new tables in MVP)  
- `handlers: null`  
**Depends on:** `moderation-foundation` (done)  
**Unlocks:** `moderation-history` (partial; also needs history work)

Use this file as the implement plan. Write user-facing notes in ASD-STE100.

---

## 1. Goal

Ship the **core punish command slice** for Bleed parity Wave 1.

Staff must be able to:

1. **Timeout** and **untimeout** a member (Discord communication disable).
2. **Ban**, **unban**, and **softban** a user.
3. **Warn** a member and **list warnings** for a member.
4. **Write a moderation case** for each punish action (except pure list views).

This suite builds on F1 (`ModerationCase`, reason helpers, staff check, case embeds). It does **not** ship jail, hardban, mute roles, purge, or full case history UI.

---

## 2. MVP command list

| Command | Inventory args | MVP behavior | Aliases |
|---------|----------------|--------------|---------|
| `timeout` | `member`, `duration`, `reason` | Timeout member; create case `timeout` with `duration_seconds` | none in inventory |
| `untimeout` | `member`, `reason` | Clear timeout; create case `untimeout` | none |
| `ban` | `member`, `delete_history`, `reason` | Ban user; optional delete message days (0–7); create case `ban` | none |
| `unban` | `user`, `reason` | Unban by user id/user; create case `unban` | none |
| `softban` | `member`, `delete_history`, `reason` | Ban then unban to wipe messages; default delete **1 day** if omitted; create case `softban` | none |
| `warn` | `member`, `reason` | Create case `warn`; try DM the target with the warning | none |
| `warnings` | `member` | List warn cases for target (newest first); show count | `warns` |

### MVP defaults (implement these when inventory is silent)

| Topic | Decision |
|-------|----------|
| Permissions | `guild_only`; Discord bot needs matching powers (`MODERATE_MEMBERS` for timeout, `BAN_MEMBERS` for ban/softban/unban). Caller must pass **staff check** (`require_guild_staff` / F1 helper) **or** hold the matching Discord permission for that action. Prefer: staff OR Discord perm (document chosen rule in code). |
| Hierarchy | Refuse action if target is guild owner, or target top role ≥ moderator top role (same for bot). |
| Self-target | Refuse timeout/ban/warn on self. |
| Reason | Optional for all punish cmds; use `prepare_reason`. Empty → store `NULL` and show “No reason”. |
| Duration parse | Accept human forms for timeout: `30m`, `1h`, `1d`, or integer seconds. Cap at Discord max **28 days**. Reject `0` and over-max. |
| Ban `delete_history` | Integer **0–7** days (Discord API). Default **0** for `ban`; default **1** for `softban`. |
| Case write order | Prefer: Discord action **first**, then case create. If Discord fails, do not create a case. If Discord succeeds and case write fails, log error and still report success with a note that the case was not stored. |
| Responses | `case_created_embed` via `ResponseHelper::send_embed` for punish success. Errors via `moderation_error_embed` or `ResponseHelper::send_error`. |
| `warnings` | Only `action = warn` rows via `count_for_target_action` + `list_for_target` filtered to warn (or add a thin list helper if cleaner). Cap list length (e.g. 15 lines) and show total count. |
| DM on warn | Best-effort DM; failure does not fail the command. Note in embed if DM failed. |
| Category | `Moderation` on all commands. |

### Explicitly out of MVP (deferred)

See section 8.

---

## 3. Current codebase facts

| Fact | Path / note |
|------|-------------|
| F1 case store | `src/data/models/moderation.rs` — `ModerationCase::create`, `list_for_target`, `count_for_target_action`, etc. |
| F1 action enum | `Warn`, `Ban`, `Unban`, `Softban`, `Timeout`, `Untimeout` — already matches this suite |
| Reason + staff + embeds | `src/utils/moderation.rs` — `prepare_reason`, `member_is_staff`, `require_guild_staff`, `load_staff_role_ids`, `case_created_embed`, `moderation_error_embed` |
| Data helpers | `Data::create_moderation_case`, `Data::get_moderation_case` in `src/bot/data.rs` |
| Staff roles table | `GuildStaffRole` + settings staff commands (done) |
| No mod commands yet | No `src/commands/moderation/` |
| Registration | `src/commands/mod.rs`, `src/bot/framework.rs` — register **last** |
| Response patterns | `ResponseHelper`, `EmbedColor` only (no raw hex) |
| Discord IDs | `i64` in SQLite (already) |

---

## 4. Design

### 4.1 Module layout

```text
src/commands/moderation/
  mod.rs          # re-exports; optional shared helpers (staff gate, hierarchy, duration parse)
  timeout.rs      # timeout + untimeout (or split files if clearer)
  ban.rs          # ban, unban, softban
  warn.rs         # warn + warnings
```

Prefer **flat commands** (not a single parent `mod` shell) so prefix and slash match Bleed-style names (`!ban`, `/ban`).

Alternative if code is small: one file per command. Keep shared gate helpers in `mod.rs` to avoid copy-paste.

### 4.2 Shared pre-checks (`mod.rs` helpers)

Suggested async helpers (names flexible):

| Helper | Behavior |
|--------|----------|
| `ensure_guild(ctx)` | Require `guild_id`; error if DM |
| `ensure_moderator(ctx, needed_discord_perm)` | Load member; load staff roles; require staff **or** Discord permission; use F1 `member_is_staff` |
| `ensure_can_moderate(ctx, target: &Member)` | Hierarchy + not self + not owner |
| `parse_duration_to_seconds(input: &str) -> Result<i64, ...>` | Timeout only |
| `clamp_delete_days(n: Option<u8>) -> u8` | 0–7 |

Do not hold DB write locks across Discord HTTP awaits (project rule).

### 4.3 Per-command Discord API map

| Command | Serenity / Discord operation | Case action | duration_seconds |
|---------|------------------------------|-------------|------------------|
| `timeout` | `EditMember` / disable communication until `now + duration` | `Timeout` | yes |
| `untimeout` | Clear communication disable (`None` / null until) | `Untimeout` | null |
| `ban` | `ban_user` with `dmd` delete message days | `Ban` | null |
| `unban` | `unban_user` | `Unban` | null |
| `softban` | ban with delete days, then unban | `Softban` | null |
| `warn` | optional DM only (no guild mute) | `Warn` | null |
| `warnings` | none (read DB) | — | — |

Use audit-log reason strings that include moderator id when Discord supports a reason field (truncate to Discord limits).

### 4.4 Staff vs Discord permissions (chosen rule)

**Allow** the command when **any** of these is true:

1. Guild owner  
2. Member has configured staff role (F1 list)  
3. Member has Discord permission for the action:
   - timeout/untimeout → `MODERATE_MEMBERS`
   - ban/unban/softban → `BAN_MEMBERS`
   - warn/warnings → `MODERATE_MEMBERS` or `KICK_MEMBERS` or staff (pick one stable rule: recommend staff OR `MODERATE_MEMBERS` OR `KICK_MEMBERS` for warn)

Still require the **bot** to have the Discord permission, or return a clear error.

Also set Poise `required_bot_permissions` / `required_permissions` where they help Discord’s UI, but **do not** rely only on Discord perms if staff roles should grant access without those flags.

### 4.5 Softban

1. Resolve target member.  
2. Hierarchy checks.  
3. Ban with `delete_history` (default 1 day).  
4. Immediately unban.  
5. Create one case with `ModerationAction::Softban`.  
6. If unban fails after ban, report partial failure and still create case if ban succeeded (log `error`).

### 4.6 Warnings list embed

- Title: warnings for target  
- Color: `EmbedColor::Primary`  
- Body: case number, moderator mention/id, reason, created_at (if present)  
- Footer: total warn count  
- Empty state: info embed “No warnings”

### 4.7 Errors (user-facing)

Short, clear messages (STE100):

- Not staff / missing permission  
- Invalid reason  
- Invalid duration  
- Cannot moderate this member (hierarchy)  
- Bot missing permission  
- User not found / not in guild (as applicable)  
- Already banned / not banned (map Discord errors when practical)

Map through `ModerationError` where it fits; use `ResponseHelper::send_error` so the user sees embeds, not raw panics.

### 4.8 Logging

`tracing` on punish paths:

```text
guild_id, action, target_id, moderator_id, case_number (if any)
```

- `info` on success  
- `warn` on DM failure / case write failure after Discord success  
- `error` on unexpected API/DB failure  

No secrets in logs.

---

## 5. Files to touch

### Create

| Path | Role |
|------|------|
| `src/commands/moderation/mod.rs` | Module root + shared gates |
| `src/commands/moderation/timeout.rs` | `timeout`, `untimeout` |
| `src/commands/moderation/ban.rs` | `ban`, `unban`, `softban` |
| `src/commands/moderation/warn.rs` | `warn`, `warnings` |
| Optional unit tests in `moderation/mod.rs` or `utils` | Duration parse, delete_days clamp |

### Modify

| Path | Change |
|------|--------|
| `src/commands/mod.rs` | `pub mod moderation;` |
| `src/bot/framework.rs` | Register the 7 commands **last** in implement |
| Optional small F1 tweaks only if needed | e.g. export helper; avoid schema churn |

### Do not change in this suite

| Path / area | Why |
|-------------|-----|
| Case table schema | F1 already complete |
| `history` / `caselog` / `reason` commands | `moderation-history` |
| Purge | `moderation-purge-basic` |
| Jail / mute / hardban / tempban | Later suites |
| Modlog channel settings | Later settings/modlog work |
| Invoke / DM templates | invoke suite |
| Handlers | No event handlers for MVP |

**Path note vs purge suite:** Registry also lists `commands: src/commands/moderation/` for `moderation-purge-basic`. Paths **overlap**. Do **not** run both pipelines in the same wave without serializing. Prefer finishing `moderation-punish` registration first, then purge adds files under the same folder.

---

## 6. Implementation phases

### Phase A — Shared gates and duration parse

1. Create `src/commands/moderation/mod.rs` with staff gate, hierarchy check, duration parse, delete-days clamp.  
2. Unit tests for duration and clamp (no Discord HTTP).  

### Phase B — Timeout family

1. Implement `timeout` and `untimeout`.  
2. Case create + `case_created_embed`.  
3. Manual mental checklist: hierarchy, bot perms, reason.  

### Phase C — Ban family

1. Implement `ban`, `unban`, `softban`.  
2. Softban ban+unban order and partial failure messaging.  

### Phase D — Warn family

1. Implement `warn` (case + best-effort DM).  
2. Implement `warnings` / alias `warns`.  

### Phase E — Registration and quality

1. Export module; register all seven commands in `framework.rs` **last**.  
2. `cargo fmt`, `cargo test`, `cargo clippy`.  
3. Fill acceptance notes; list deferred items in PR body.  

---

## 7. Acceptance criteria

The suite is done when all of these are true:

1. All seven MVP commands exist as Poise slash + prefix commands and are registered.  
2. Each punish action (not `warnings`) creates a case with the correct `ModerationAction` when Discord succeeds.  
3. Reasons use `prepare_reason` (length and `@everyone` / `@here` rules).  
4. Staff roles (F1) or matching Discord permissions can run the commands; non-staff without perms cannot.  
5. Hierarchy and self-target checks block unsafe actions.  
6. Timeout duration parse rejects invalid values and caps at 28 days.  
7. Softban bans then unbans and stores one `softban` case.  
8. `warnings` / `warns` lists warn cases for a member.  
9. Responses use `ResponseHelper` / F1 embeds and `EmbedColor` only.  
10. `cargo fmt`, `cargo test`, and `cargo clippy` pass for the change set.  
11. Deferred Bleed commands are listed (below), not silent gaps.  
12. No purge/history/jail features are half-implemented in this PR.

---

## 8. Deferred (explicit)

| Item | Owner / later work |
|------|---------------------|
| `kick` | Later mod suite (not in registry MVP list) |
| `hardban`, `hardban list` | Later |
| `tempban`, `unbanall` / `massunban` | Later |
| `jail` / `unjail` / `jaillist`, mute role family (`mute`, `imute`, `rmute`, …) | Later + settings |
| `ban purge`, `ban recent` / `chunkban` | Later |
| `timeout list` | Later |
| Full `history`, `caselog`/`case`, `reason` | `moderation-history` |
| `purge` family | `moderation-purge-basic` |
| Modlog channel posts | Settings / modlog suite |
| Invoke ban/warn/timeout message templates | invoke suite |
| Notes, proof attachments | Later |
| Auto-punish / filter integration | After message foundation + filter |

---

## 9. Security and permissions

| Topic | Rule |
|-------|------|
| Guild only | All commands `guild_only` |
| Staff / Discord perms | See §4.4 |
| Hierarchy | No escalate past moderator or bot |
| User input | Reason via F1 helpers; duration/delete days validated |
| Mentions | Do not allow `@everyone` / `@here` in stored reasons |
| Secrets | None |
| Rate limits | Single-target commands only; no mass unban in MVP |

---

## 10. Git and registry

```bash
git checkout main
git pull
git checkout -b feature/moderation-punish
```

Or worktree via `/orchestrate-commands implement moderation-punish`.

**Registry fields after plan:**

- `roadmap: .grok/roadmaps/moderation-punish-roadmap.md`
- `status: pending` (plan complete; not implementing yet)
- `branch: feature/moderation-punish`

After implement PR is ready: set `pr:` URL and status `pr_open`.  
After merge: `/merge-suite-pr` or `record-merge moderation-punish`.

---

## 11. Implement agent checklist

1. Re-read this roadmap and F1 helpers (`src/utils/moderation.rs`, `src/data/models/moderation.rs`).  
2. Do not expand into deferred commands.  
3. Register commands last.  
4. Run fmt / test / clippy in the suite worktree.  
5. Do not merge to `main`.  
6. Do not start `moderation-purge-basic` in the same worktree without coordination (shared `src/commands/moderation/`).  
