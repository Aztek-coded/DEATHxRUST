# Grok agent surface (DEATHxRUST)

Project-local configuration for [Grok](https://x.ai/cli) agentic coding. Mirrors the workflows in `.claude/` (Claude Code) and `.cursor/` (Cursor), adapted for Grok discovery paths.

## Layout

| Path | Purpose |
|------|---------|
| `rules/` | Always-loaded project rules (`*.md`) |
| `skills/` | Project-local invocable skills (`*/SKILL.md`) |
| `commands/` | Optional legacy slash aliases (kept empty; use plugins + skills) |
| `guidelines/` | Longer reference docs (not auto-loaded; skills/rules link here) |
| `roadmaps/` | Feature implementation roadmaps |
| `enhancements/` | Guideline-alignment notes |
| `orchestration/` | Suite registry for multi-agent command parity |

## Skills

Keep only project-specific skills here. Generic workflows (commit, feature roadmaps, troubleshoot) come from installed Grok plugins so they do not clash with project copies.

| Skill | Slash | Use for |
|-------|-------|---------|
| `orchestrate-commands` | `/orchestrate-commands` | Multi-agent Bleed parity: registry, waves, plan/implement/test/review |
| `merge-suite-pr` | `/merge-suite-pr` | After you confirm ready: merge PR → record-merge → delete feature branch |

## Rules (auto-loaded)

| File | Scope guidance |
|------|----------------|
| `project-overview.md` | Architecture, commands, config, registration |
| `rust-discord-bot.md` | Errors, async, security, logging |
| `command-development.md` | Poise commands, embeds, aliases |
| `data-management.md` | Shared state, cache, DB |

## Relation to other harnesses

| Surface | Role |
|---------|------|
| `.grok/` | **Grok primary** — rules, skills, new roadmaps/enhancements |
| `.cursor/` | Cursor rules/skills (Grok may still discover via compat) |
| `.claude/` | Historical roadmaps, enhancements, guidelines only (slash commands removed; use `.grok/skills/`) |

**Do not treat `.claude/` as the Grok source of truth.** Prefer `.grok/` for new Grok work. Historical roadmaps live under `.claude/roadmaps/` and may be referenced when implementing older plans.

## Git

Follow user commit/PR rules. Do not commit, push, or merge unless the user explicitly asks.
