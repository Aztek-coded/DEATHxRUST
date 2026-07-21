# Roadmap: moderation-foundation (F1)

**Suite id:** `moderation-foundation`  
**Category:** foundation  
**Branch:** `feature/moderation-foundation`  
**Registry paths:** `data: src/data/models/moderation.rs`  
**Depends on:** `agent-orchestration` (done)  
**Unlocks:** `moderation-punish`, `moderation-purge-basic`, `moderation-history` (and later jail/mute/modlog suites)

Use this file as the implement plan. Write user-facing notes in ASD-STE100.

---

## 1. Goal

Ship the **moderation data and helper layer** that later suites need.

This suite must provide:

1. A guild-scoped **case log** store (SQLite).
2. **Reason** normalize and validate helpers.
3. A shared **staff check** that reuses `guild_staff_roles`.
4. Shared **moderation embeds** (`ResponseHelper` / `EmbedColor`).

This suite must **not** ship punish or history Discord commands. Those are separate suites.

---

## 2. MVP scope

| Deliverable | In scope | Out of scope |
|-------------|----------|--------------|
| Case table + model CRUD | Yes | UI commands (`caselog`, `history`, `reason`) |
| Case action types for Wave 1 | Yes (`warn`, `ban`, `unban`, `softban`, `timeout`, `untimeout`) | Jail, mute role, hardban, notes, proof |
| Staff role membership check | Yes | New `/settings staff` features (already shipped) |
| Reason length and mention safety | Yes | Full invoke/DM template system |
| Shared case embed builders | Yes | Modlog channel config (`settings` Phase 3) |
| Unit tests for store/helpers | Yes | Full Discord E2E of ban/timeout |
| Poise command registration | **No** (`commands: []`) | Any slash/prefix mod commands |

### MVP command list

**None.** Registry lists `commands: []` for this foundation.

Downstream suites will call the helpers:

| Later suite | Commands (inventory) | Uses from F1 |
|-------------|----------------------|--------------|
| `moderation-punish` | `timeout`, `untimeout`, `ban`, `unban`, `softban`, `warn`, `warnings` | Create case, reason helper, staff check, embeds |
| `moderation-purge-basic` | `purge`, `purge bots`, `purge humans` | Staff check; optional note-only cases later |
| `moderation-history` | `history`, `caselog`/`case`, `reason` | Read/update cases, embeds |

Inventory reference: `.grok/roadmaps/full command list roadmap.md` (Moderation section).

---

## 3. Current codebase facts

| Fact | Path / note |
|------|-------------|
| Staff roles table exists | `guild_staff_roles` in `src/data/database.rs` |
| Staff CRUD exists | `GuildStaffRole` in `src/data/models/guild_settings.rs` |
| Staff commands exist | `src/commands/settings/staff.rs` |
| No case store | No `moderation` model or table today |
| No shared staff check | Settings uses `MANAGE_GUILD` only in `validate_permissions` |
| Discord IDs | Store as `i64` (project rule) |
| Embeds | Use `EmbedColor::*.value()` and `ResponseHelper` / `EmbedBuilder` |
| Shared `Data` | `src/bot/data.rs` — prefer small helpers if useful |

---

## 4. Design

### 4.1 Case identity

- Each guild has its own case sequence: `case_number` starts at 1 per `guild_id`.
- Primary key: surrogate `id` (autoincrement).
- Unique: `(guild_id, case_number)`.
- Allocate next number with a transaction: read `MAX(case_number)` for guild, insert next, or use a small `guild_moderation_counters` table if race risk is a concern. Prefer one clear approach and document it in code comments.

### 4.2 Table: `moderation_cases`

```sql
CREATE TABLE IF NOT EXISTS moderation_cases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id BIGINT NOT NULL,
    case_number INTEGER NOT NULL,
    action TEXT NOT NULL,
    target_id BIGINT NOT NULL,
    moderator_id BIGINT NOT NULL,
    reason TEXT,
    duration_seconds INTEGER,
    active INTEGER NOT NULL DEFAULT 1,
    related_case_id INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(guild_id, case_number)
);

CREATE INDEX IF NOT EXISTS idx_moderation_cases_guild
    ON moderation_cases(guild_id);

CREATE INDEX IF NOT EXISTS idx_moderation_cases_target
    ON moderation_cases(guild_id, target_id);

CREATE INDEX IF NOT EXISTS idx_moderation_cases_moderator
    ON moderation_cases(guild_id, moderator_id);
```

**Field notes:**

| Field | Purpose |
|-------|---------|
| `action` | Stable string enum: `warn`, `ban`, `unban`, `softban`, `timeout`, `untimeout` (extend later for jail/mute) |
| `target_id` | User who received the action |
| `moderator_id` | Staff or bot operator |
| `reason` | Optional after normalize; store empty as `NULL` or `""` consistently (prefer `NULL` if empty) |
| `duration_seconds` | For timeout (and later temp actions); `NULL` if N/A |
| `active` | Soft flag for future unmute/unjail lists; default `1` |
| `related_case_id` | Optional link (e.g. unban → prior ban); nullable |

### 4.3 Model API (`ModerationCase`)

Implement on `src/data/models/moderation.rs` with `sqlx` + `FromRow`:

| Method | Behavior |
|--------|----------|
| `create(...)` | Insert case; assign next `case_number`; return full row |
| `get(pool, guild_id, case_number)` | One case or `None` |
| `list_for_target(pool, guild_id, target_id, limit, offset)` | Newest first |
| `list_for_moderator(pool, guild_id, moderator_id, limit, offset)` | Newest first (for later `moderationhistory`) |
| `count_for_target(...)` | For warnings counts |
| `update_reason(pool, guild_id, case_number, reason)` | For later `reason` command |
| `set_active(...)` | Optional in F1 if cheap; else defer |

Map sqlx errors with context; do not use `.unwrap()` on production paths.

### 4.4 Action type

```rust
// Prefer a small enum with as_str / from_str for DB TEXT
pub enum ModerationAction {
    Warn,
    Ban,
    Unban,
    Softban,
    Timeout,
    Untimeout,
}
```

Reject unknown strings at parse time. Downstream suites may add variants in a later migration of the enum (same TEXT column).

### 4.5 Reason helpers

New module (suggested): `src/utils/moderation.rs` (or `src/utils/mod_reason.rs`).

| Helper | Rules |
|--------|-------|
| `normalize_reason(input: Option<&str>) -> Option<String>` | Trim; treat empty as `None` |
| `validate_reason(reason: &str) -> Result<(), ModerationError>` | Max length (suggest **512** chars); reject bare `@everyone` / `@here` in stored reason (strip or error — pick one and document; prefer **error** with user-facing message for staff tools) |

Keep messages short and clear for later command UX.

### 4.6 Staff check

New helper (same utils module or `src/utils/staff_check.rs`):

```text
is_guild_staff(ctx or http+pool, guild_id, user_id) -> Result<bool, Error>
require_guild_staff(...) -> Result<(), Error>
```

**True when any of:**

1. Member is guild owner, or
2. Member has Discord `ADMINISTRATOR` or `MANAGE_GUILD`, or
3. Member has at least one role listed in `GuildStaffRole::list` for that guild

**Notes:**

- Reuse existing staff table; do not duplicate staff storage.
- Do not hold a write lock across Discord API awaits.
- Cache of staff role ids is optional in F1; if added, use a small TTL and invalidate on settings staff add/remove in a follow-up (document if deferred).
- Settings admin commands may keep `MANAGE_GUILD`-only checks; this helper is for **moderation** staff paths.

### 4.7 Shared mod embeds

Builders that return `CreateEmbed` (or thin wrappers) using `EmbedColor`:

| Builder | Color | Use |
|---------|-------|-----|
| `case_created_embed(case)` | `Success` or `Primary` | Confirm punish (later suites) |
| `case_view_embed(case)` | `Primary` | `caselog` / history rows |
| `moderation_error_embed(title, body)` | `Error` | Permission / validation failures |
| `moderation_warning_embed(...)` | `Warning` | Soft notices |

Include case number, action, target mention/id, moderator, reason (or “No reason”), and timestamp when available.

Do **not** use raw hex colors.

### 4.8 Optional `Data` helpers

If it reduces duplication, add thin async methods on `Data`:

- `create_moderation_case(...)`
- `get_moderation_case(...)`

Keep model methods as the source of truth. Do not leak lock details.

### 4.9 Errors

Add a small typed error (e.g. `ModerationError` in `src/utils/moderation.rs` or next to settings errors):

- `NotStaff`
- `InvalidReason`
- `CaseNotFound`
- `Database` (or map via `?` + context)

Implement `Into<crate::bot::Error>` or `Display` + conversion consistent with `SettingsError`.

---

## 5. Files to touch

### Create

| Path | Role |
|------|------|
| `src/data/models/moderation.rs` | Case model + CRUD |
| `src/utils/moderation.rs` | Reason helpers, staff check, embeds, errors |
| `tests/moderation_foundation_tests.rs` (or unit tests in model file) | Case CRUD, reason, staff list membership pure checks |

### Modify

| Path | Change |
|------|--------|
| `src/data/database.rs` | Create `moderation_cases` (+ indexes) in schema init |
| `src/data/models/mod.rs` | `mod moderation;` and re-exports |
| `src/utils/mod.rs` | `mod moderation;` and public exports needed by later suites |

### Do not change in this suite

| Path | Why |
|------|-----|
| `src/bot/framework.rs` | No new commands to register |
| `src/commands/mod.rs` | No command module |
| `src/handlers/*` | No event routing in F1 |
| Settings staff commands | Already complete |

---

## 6. Implementation phases

### Phase A — Schema and model

1. Add table + indexes in `database.rs`.
2. Implement `ModerationAction` and `ModerationCase` CRUD.
3. Unit tests: create two cases in one guild → numbers 1 and 2; get; list_for_target; update_reason.

### Phase B — Helpers

1. Reason normalize/validate + tests.
2. Staff check using `GuildStaffRole::list` + owner/admin/manage guild.
3. Shared embed builders (smoke-test fields in unit tests if practical without full Context).

### Phase C — Integration polish

1. Export types from `models` and `utils`.
2. Optional `Data` helpers.
3. `cargo fmt`, `cargo clippy`, `cargo test`.
4. Short acceptance notes in this file or PR body.

---

## 7. Acceptance criteria

The suite is done when all of these are true:

1. Fresh DB init creates `moderation_cases` without error.
2. `ModerationCase::create` assigns per-guild monotonic `case_number`.
3. `get` / `list_for_target` / `update_reason` work in tests.
4. Reason helper rejects overlong or unsafe `@everyone`/`@here` reasons per chosen rule.
5. Staff helper returns true for configured staff roles and for owner / admin / manage guild (test with pure helpers where Discord HTTP is not required; document any HTTP-backed path).
6. Embed builders use `EmbedColor` only.
7. No new Poise commands are registered.
8. `cargo fmt`, `cargo test`, and `cargo clippy` pass for the change set.
9. Deferred work is listed (below) and not silently “half implemented” as commands.

---

## 8. Deferred (explicit)

| Item | Owner suite / later work |
|------|---------------------------|
| `ban`, `unban`, `softban`, `timeout`, `untimeout`, `warn`, `warnings` | `moderation-punish` |
| `purge` family | `moderation-purge-basic` |
| `history`, `caselog`/`case`, `reason` command | `moderation-history` |
| `hardban`, `jail`, `mute`, `imute`, notes, proof | Later mod suites after extra design |
| Modlog channel setting | Settings Phase 3 / modlog suite |
| Invoke ban/DM templates | invoke suite |
| Staff role cache invalidation on settings change | Optional follow-up |
| `moderationhistory`, `modstats` | Later analytics/history expansion |

---

## 9. Logging

Use `tracing` on write paths:

```text
guild_id, case_number, action, target_id, moderator_id
```

- `info` on successful case create / reason update  
- `debug` on list/get  
- `warn` / `error` on DB failures (no secrets)

---

## 10. Security and permissions

| Topic | Rule |
|-------|------|
| Staff check | Required by later mod commands; ship the helper here |
| User input | Reason length + mention safety |
| Discord IDs | `i64` in SQLite |
| Secrets | None |
| Escalation | F1 does not grant Discord punish powers; it only stores data and checks |

---

## 11. Git and registry

```bash
git checkout main
git pull
git checkout -b feature/moderation-foundation
```

Or worktree via `/orchestrate-commands implement moderation-foundation`.

**Registry fields after plan:**

- `roadmap: .grok/roadmaps/moderation-foundation-roadmap.md`
- `status: pending` (plan complete; not implementing yet)
- `branch: feature/moderation-foundation`

After merge of the implement PR:

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb record-merge moderation-foundation --write
```

---

## 12. Suggested implement order (agent checklist)

- [ ] Branch `feature/moderation-foundation` from latest `main`
- [ ] Set registry `status: implementing`
- [ ] Schema in `database.rs`
- [ ] `moderation.rs` model + tests
- [ ] Utils: reason, staff, embeds, errors
- [ ] Exports in `models/mod.rs` and `utils/mod.rs`
- [ ] Optional `Data` helpers
- [ ] `cargo fmt` / `test` / `clippy`
- [ ] Status → testing → reviewing
- [ ] PR only if user allows; no auto-merge

---

## 13. Success signal

`registry_tool next` shows Wave 1 product suites as ready (`moderation-punish` and/or `moderation-purge-basic`) once this suite is `done`. Implement agents can import case store and staff/reason helpers without inventing a second schema.
