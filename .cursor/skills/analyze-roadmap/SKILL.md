---
name: analyze-roadmap
description: >-
  Analyzes a roadmap for alignment with DEATHxRUST Cursor development rules and
  writes enhancement notes under .cursor/enhancements/ when gaps exist. Use when
  the user asks to analyze a roadmap, check guideline alignment, or run
  ANALYZE-ROADMAP.
---

# Analyze Roadmap

## Instructions

1. Read the provided roadmap (typically under `.cursor/roadmaps/`).
2. Compare against Cursor project rules:
   - `.cursor/rules/project-overview.mdc`
   - `.cursor/rules/rust-discord-bot.mdc`
   - `.cursor/rules/command-development.mdc`
   - `.cursor/rules/data-management.mdc`
3. Check that roadmap code examples and steps match those rules (Poise structure, `ResponseHelper`/`EmbedColor`, error handling, async, permissions, registration).
4. If enhancements are needed, write `.cursor/enhancements/<roadmap-slug>-enhancements.md`.
5. If aligned, say so briefly and list any minor nits (no file required for nits-only unless the user wants one).

## Enhancement file structure

```markdown
# Enhancements: <roadmap name>

## Summary
[alignment verdict]

## Gaps / violations
- [rule] — [where in roadmap] — [fix]

## Recommended roadmap edits
1. ...

## Optional improvements
- ...
```

## Notes

- Prefer repo-relative paths.
- Do not implement the feature unless asked; this skill only analyzes and documents.
