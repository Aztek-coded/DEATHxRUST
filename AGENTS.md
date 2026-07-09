# AGENTS.md

Cursor is the primary agent surface for this repo. `.claude/` is for Claude Code only.

## Always-on

- `.cursor/rules/project-overview.mdc` — architecture, commands, config, command registration

## File-scoped rules

| Rule | When |
|------|------|
| `.cursor/rules/rust-discord-bot.mdc` | `**/*.rs` |
| `.cursor/rules/command-development.mdc` | `src/commands/**/*.rs` |
| `.cursor/rules/data-management.mdc` | `src/{bot,data,config}/**/*.rs` |

## Skills (auto-discoverable)

| Skill | Use for |
|-------|---------|
| `new-feature-report` | Intake a feature brief → structured report → pick roadmap depth |
| `new-feature-roadmap` | Write `.cursor/roadmaps/<slug>-roadmap.md` (standard / think-hard / think-hardest) |
| `implement-roadmap` | Branch + implement a roadmap (no commit unless asked) |
| `analyze-roadmap` | Guideline alignment → `.cursor/enhancements/` if needed |
| `troubleshoot-issue` | Blank troubleshooting report from description/screenshots |

## Artifacts

- Roadmaps: `.cursor/roadmaps/`
- Enhancements: `.cursor/enhancements/`

## Git

Follow the user's commit/PR rules. Do not commit, push, or merge unless explicitly asked.
