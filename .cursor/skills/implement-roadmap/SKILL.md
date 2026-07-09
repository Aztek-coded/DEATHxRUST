---
name: implement-roadmap
description: >-
  Implements a referenced roadmap or resolution file thoroughly on a new git
  branch. Use when the user asks to implement a roadmap, build from a plan in
  .cursor/roadmaps/, or run IMPLEMENT against a planning document.
---

# Implement Roadmap

## Instructions

1. **Analyze** the referenced roadmap/resolution file end-to-end.
2. **Create a git branch** before code changes (use the roadmap's suggested name, or derive `feature/<slug>`).
3. **Implement completely** per the roadmap — modules, commands, handlers, data, registration, logging.
4. Follow `.cursor/rules/` for Poise patterns, embeds/colors, errors, and data access.
5. Register new commands in `src/bot/framework.rs` and export via `src/commands/mod.rs`.
6. Run relevant checks (`cargo check` / `cargo test` / `cargo clippy`) and fix issues you introduce.
7. **Do not commit or push** unless the user explicitly asks.

## Completion checklist

- [ ] Branch created
- [ ] Roadmap steps addressed (note any intentional deferrals)
- [ ] Commands registered/exported if applicable
- [ ] `tracing` added where the roadmap specified
- [ ] Build/tests pass for touched areas
- [ ] Brief summary of what was implemented and what remains
