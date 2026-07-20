# AGENTS.md

Multi-harness agent instructions for DEATHxRUST.

| Surface | Role |
|---------|------|
| **`.grok/`** | Grok primary — rules, skills, new roadmaps/enhancements |
| **`.cursor/`** | Cursor rules/skills |
| **`.claude/`** | Claude Code only — historical roadmaps, enhancements, guidelines (no slash commands; use `.grok/skills/`) |

Prefer the active harness's directory for new artifacts. Do not treat `.claude/` as the Grok source of truth.

## Always-on (Grok)

- `.grok/rules/project-overview.md` — architecture, commands, config, registration
- `.grok/rules/rust-discord-bot.md` — errors, async, security, logging
- `.grok/rules/command-development.md` — Poise commands, embeds, aliases
- `.grok/rules/data-management.md` — shared state, cache, DB

## Always-on (Cursor)

- `.cursor/rules/project-overview.mdc` — architecture, commands, config, command registration

## File-scoped rules (Cursor)

| Rule | When |
|------|------|
| `.cursor/rules/rust-discord-bot.mdc` | `**/*.rs` |
| `.cursor/rules/command-development.mdc` | `src/commands/**/*.rs` |
| `.cursor/rules/data-management.mdc` | `src/{bot,data,config}/**/*.rs` |

## Skills (Grok — auto-discoverable under `.grok/skills/`)

| Skill | Use for |
|-------|---------|
| `new-feature-report` | Intake a feature brief → structured report → pick roadmap depth |
| `new-feature-roadmap` | Write `.grok/roadmaps/<slug>-roadmap.md` (standard / think-hard / think-hardest) |
| `implement-roadmap` | Branch + implement a roadmap (no commit unless asked) |
| `analyze-roadmap` | Guideline alignment → `.grok/enhancements/` if needed |
| `troubleshoot-issue` | Blank troubleshooting report from description/screenshots |
| `commit` | Stage + commit (push only if asked) |
| `commit-and-merge` | Commit, push, merge to main (explicit only) |
| `orchestrate-commands` | Multi-agent Bleed command parity orchestration (registry + waves) |

## Skills (Cursor)

| Skill | Use for |
|-------|---------|
| `new-feature-report` | Intake a feature brief → structured report → pick roadmap depth |
| `new-feature-roadmap` | Write `.cursor/roadmaps/<slug>-roadmap.md` |
| `implement-roadmap` | Branch + implement a roadmap (no commit unless asked) |
| `analyze-roadmap` | Guideline alignment → `.cursor/enhancements/` if needed |
| `troubleshoot-issue` | Blank troubleshooting report from description/screenshots |

## Artifacts

| Kind | Grok | Cursor | Claude (historical) |
|------|------|--------|---------------------|
| Roadmaps | `.grok/roadmaps/` | `.cursor/roadmaps/` | `.claude/roadmaps/` |
| Enhancements | `.grok/enhancements/` | `.cursor/enhancements/` | `.claude/Enhancements/` |
| Guidelines | `.grok/guidelines/` | (embedded in rules) | `.claude/guidelines/` |

## Longer references

- `.grok/README.md` — Grok surface index
- `CLAUDE.md` — Claude Code dev commands and architecture
- `.grok/guidelines/` — full Rust/Poise, command, and data guidelines

## Git

Follow the user's commit/PR rules. Do not commit, push, or merge unless explicitly asked.
