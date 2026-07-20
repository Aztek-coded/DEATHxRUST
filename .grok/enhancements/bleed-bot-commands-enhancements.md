# Enhancements: Bleed Bot Commands Reference

## Summary

**Alignment verdict: partial / needs structural upgrades before use as an implementation plan.**

The document at `.grok/roadmaps/bleed-bot-commands-roadmap.md` (moved from `.claude/roadmaps/bleed_bot_commands.md`) is primarily a **Bleed command catalog + implementation status tracker**, not a Grok-style feature implementation roadmap. Implemented suites (core, prefix, boosterrole, settings high-priority) are documented usefully. Unimplemented command groups are inventory-only: arguments and Discord permissions, without Poise structure, registration, data layer, handlers, validation, or ResponseHelper/`EmbedColor` guidance required by project rules.

Use this file as a **product backlog / parity map**. For any feature you actually build, produce a focused `.grok/roadmaps/<slug>-roadmap.md` (via `new-feature-roadmap`) that incorporates the gaps below.

## Gaps / violations

### Document role and scope

- **[project-overview / roadmap skill]** — Whole document — Not an implementable roadmap: no branch strategy, file layouts, phased technical steps, or acceptance criteria for remaining features. Settings Phases 1–2 are already shipped but still read as “immediate implementation.”
- **[implementation summary accuracy]** — “Overall Progress: 40/41 (98%)” — Counts only core + prefix + boosterrole + high-priority settings. Large unfinished areas (boosts, alias, sticky, welcome/goodbye, filter, autoresponder, suggest, etc.) are excluded, so the percentage overstates Bleed parity.

### Command development rules

- **[command-development — Poise structure]** — All unimplemented sections — No parent/subcommand shell pattern, `#[poise::command(...)]` shape, `guild_only`, categories, or `subcommands(...)` wiring.
- **[command-development — ResponseHelper / EmbedColor]** — Entire catalog — No requirement to use `ResponseHelper::{send_success,send_error,send_info,send_warning}` or `EmbedColor::*.value()`; risk of ad-hoc embeds and raw hex if implementers treat this doc as the plan.
- **[command-development — aliases]** — Unimplemented command groups — No 1-letter shortcuts, abbreviations, or subcommand alias mirrors (`v`/`show`, `s`/`set`, `r`/`rm`).
- **[command-development — registration checklist]** — Unimplemented groups — Missing steps: implement under `src/commands/`, export in `src/commands/mod.rs`, register in `src/bot/framework.rs` `commands` vec.
- **[command-development — permissions]** — Permission Levels section + many commands — Labels like “Booster Only”, “Staff Only”, “Tier 2 Only” are not mapped to Poise checks (`required_permissions`, custom checks, staff-role DB lookup). Several inventory entries look inconsistent (e.g. `fakepermissions add` lists “Permissions: none” while the suite is owner-gated; `webhook edit` description duplicates “Send message”).

### Rust / Discord bot patterns

- **[rust-discord-bot — errors]** — Future work — No typed errors / `?` + context / user-facing vs `tracing` split for new modules.
- **[rust-discord-bot — security]** — filter regex, autoresponder, webhook send/edit, seticon/banner, invoke messages — No input validation (length, `@everyone`/`@here`, URL safety), rate limits, or bot permission requirements.
- **[rust-discord-bot — async]** — sticky, welcome, filter, spam, reposter — No guidance on non-blocking handlers, timeouts, or avoiding holds across Discord API awaits.
- **[rust-discord-bot — logging]** — Unimplemented — No structured `tracing` fields (`user_id`, `guild_id`, command name) on command paths.

### Data management

- **[data-management — schema & access]** — Unimplemented suites — No SQLite tables, models, UPSERT patterns, Discord IDs as `i64`, or repository helpers on `Data`.
- **[data-management — caching]** — alias, filter, autoresponder, ignore, command enable/disable — Hot-path guild configs should plan TTL/bounded cache and invalidation after writes; not mentioned.
- **[data-management — concurrency]** — Message filters / sticky re-post — Must not hold write locks across Discord API awaits; not specified.
- **[data-management — event coupling]** — boosts, sticky, welcome, goodbye, imgonly, filter, autonick-like flows — Not mapped to `src/handlers/` (e.g. existing `MemberHandler` / `BoostHandler` patterns) or message create/delete events.

### Architecture / product planning

- **[dependency systems]** — invoke, settings moderation (muted/jail/modlog), music/Last.fm/Google items — Correctly marked not implementable without larger systems, but there is no dependency graph or “do not start until X” gate for filter/invoke vs moderation.
- **[prefix self]** — Tier 2 personal prefix — No premium/subscription design, storage model, or multi-guild resolution vs guild prefix in `dynamic_prefix`.
- **[missing inventory detail]** — `boosterrole share` — Parent notes `role` subcommand; no dedicated `boosterrole share role` entry (implementation exists as `share_role` renamed to `role` in `src/commands/boosterrole/share.rs`).
- **[stale status markers]** — `boosterrole` parent marked ⚠️ while suite is otherwise 100%; settings Phases 1–2 still “planned”; unimplemented sections lack consistent ❌ / priority tags.

## Recommended roadmap edits

1. **Retitle and reframe** the Grok copy as a **Bleed parity inventory / backlog**, and point implementers to per-feature roadmaps under `.grok/roadmaps/`.
2. **Split the summary** into:
   - Implemented suites (with accurate command counts),
   - Not implementable (blocked on moderation / music / integrations),
   - Candidate backlog (boosts, alias, sticky, welcome, goodbye, imgonly, filter, autoresponder, …) with priority and dependency notes.
3. **Mark every unfinished entry** with status (❌), priority (🔴/🟡/🟢/⚫), and one-line dependency (e.g. “needs MessageCreate handler”).
4. **Delete or archive Settings Phases 1–2** as historical; keep Phase 3 only as blocked backlog.
5. **For each high-value backlog group** (suggest: boosts → welcome/goodbye → sticky → alias → filter), add a minimal “when implementing” checklist:
   - Module path under `src/commands/<name>/`
   - Parent + subcommands Poise skeleton
   - Tables / models / cache keys
   - Handler hooks if event-driven
   - `ResponseHelper` + `EmbedColor`
   - Registration in `framework.rs` + `commands/mod.rs`
   - Permissions: Discord flags + custom staff/booster checks
   - Input validation and bot permissions
6. **Fix inventory nits**: document `boosterrole share role`; correct `webhook edit` description; align `fakepermissions` permission lines; clarify `boosterrole` parent ⚠️ vs help-only parent pattern.
7. **Map permission levels** to concrete checks:
   - Manage Guild / Channels / Messages / Webhooks / Administrator → `required_permissions`
   - Booster Only → guild premium subscription / boost member check
   - Staff Only → `guild_staff_roles` lookup (existing settings staff data)
   - Server Owner → owner id check
   - Tier 2 Only → document as blocked until premium tier system exists

## Optional improvements

- Add a short **“Already in DEATHxRUST”** section linking real paths (`src/commands/boosterrole/`, `src/commands/settings/`, `src/handlers/`) so the inventory stays tied to the codebase.
- Cross-link related Claude-era plans still under `.claude/roadmaps/` (`settings-command-suite-roadmap.md`, boosterrole roadmaps) and prefer Grok copies when those are migrated.
- Produce focused roadmaps next (suggested order for parity with existing boost/member infrastructure):
  1. `boosts` (extends `BoostHandler`)
  2. `welcome` / `goodbye` (extends `MemberHandler`)
  3. `stickymessage` (message events)
  4. `alias` (prefix resolution integration)
  5. `filter` (message moderation — larger design)
- Normalize slug naming: keep Grok path as `bleed-bot-commands-roadmap.md` (kebab-case + `-roadmap` suffix).
- When any feature is picked up, run `analyze-roadmap` again on the **feature** roadmap, not only this catalog.

## Migration note

| Item | Path |
|------|------|
| Source (removed) | `.claude/roadmaps/bleed_bot_commands.md` |
| Destination | `.grok/roadmaps/bleed-bot-commands-roadmap.md` |
| Enhancements | `.grok/enhancements/bleed-bot-commands-enhancements.md` |
