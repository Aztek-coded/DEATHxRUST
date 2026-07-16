---
name: analyze-roadmap
description: >-
  Analyzes a roadmap for alignment with DEATHxRUST Grok development rules and
  writes enhancement notes under .grok/enhancements/ when gaps exist. Use when
  the user asks to analyze a roadmap, check guideline alignment, or run
  ANALYZE-ROADMAP.
---

# Analyze Roadmap

## Instructions

1. Read the provided roadmap (typically under `.grok/roadmaps/` or `.claude/roadmaps/`).
2. Compare against Grok project rules:
   - `.grok/rules/project-overview.md`
   - `.grok/rules/rust-discord-bot.md`
   - `.grok/rules/command-development.md`
   - `.grok/rules/data-management.md`
3. Optionally cross-check longer guidelines in `.grok/guidelines/` if the roadmap is large or high-risk.
4. Check that roadmap code examples and steps match those rules (Poise structure, `ResponseHelper`/`EmbedColor`, error handling, async, permissions, registration).
5. If enhancements are needed, write `.grok/enhancements/<roadmap-slug>-enhancements.md`.
6. If aligned, say so briefly and list any minor nits (no file required for nits-only unless the user wants one).

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
