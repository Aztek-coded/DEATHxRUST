# Grok agent surface (DEATHxRUST)

Project-local configuration for [Grok](https://x.ai/cli) agentic coding. Mirrors the workflows in `.claude/` (Claude Code) and `.cursor/` (Cursor), adapted for Grok discovery paths.

## Layout

| Path | Purpose |
|------|---------|
| `rules/` | Always-loaded project rules (`*.md`) |
| `skills/` | Invocable skills (`*/SKILL.md`) — also slash commands |
| `commands/` | Legacy flat slash-command markdown (optional aliases) |
| `guidelines/` | Longer reference docs (not auto-loaded; skills/rules link here) |
| `roadmaps/` | Feature implementation roadmaps written by skills |
| `enhancements/` | Guideline-alignment notes from `analyze-roadmap` |
| `orchestration/` | Suite registry for multi-agent command parity |

## Skills

| Skill | Slash | Use for |
|-------|-------|---------|
| `new-feature-report` | `/new-feature-report` | Feature intake → structured report → pick roadmap depth |
| `new-feature-roadmap` | `/new-feature-roadmap` | Write `.grok/roadmaps/<slug>-roadmap.md` |
| `implement-roadmap` | `/implement-roadmap` | Branch + implement a roadmap (no commit unless asked) |
| `analyze-roadmap` | `/analyze-roadmap` | Alignment check → `.grok/enhancements/` if needed |
| `troubleshoot-issue` | `/troubleshoot-issue` | Bug/repro intake → troubleshooting report |
| `commit` | `/commit` | Stage + commit (push only if user asks) |
| `commit-and-merge` | `/commit-and-merge` | Commit, push, merge to main (explicit only) |
| `orchestrate-commands` | `/orchestrate-commands` | Multi-agent Bleed parity: registry, waves, plan/implement/test/review |

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
