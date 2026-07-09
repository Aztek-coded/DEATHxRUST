---
name: new-feature-report
description: >-
  Turns a brief Discord bot feature description (and optional screenshots) into
  a structured feature report via interactive intake, then asks which roadmap
  depth to use. Use when the user requests a new feature report, describes a new
  bot command/feature to plan, or mentions NEW-FEATURE-REPORT / feature intake.
---

# New Feature Report

## Interactive intake (required first)

Before writing the report, collect missing context. Skip only questions already answered clearly in the user's message or attachments.

Ask in one batch when possible:

1. **New vs update?** New feature, or change to an existing command/system?
2. **Command surface?** Slash, prefix, or both?
3. **Permissions?** Who can run it (e.g. Manage Guild, booster-only, everyone)?
4. **Arguments / options?** Required and optional params (or "none / TBD")?
5. **Persistence?** None, cache-only, or database (new/existing tables)?
6. **Events?** Command-only, or also Discord events (joins, boosts, reactions, etc.)?
7. **UX surface?** Text, embeds, buttons/modals/selects?
8. **Outcomes?** Must-have vs nice-to-have expected results?
9. **References?** Screenshots, competitor bots, or repo files to study?
10. **Roadmap depth?** `standard` / `think-hard` / `think-hardest` (ask here or right after the report)

If answers are incomplete, ask follow-ups before filling the template. Do not invent requirements; mark remaining unknowns as `TBD`.

## Instructions

1. Run interactive intake (above).
2. Fill the report template from answers + description/screenshots.
3. Confirm roadmap depth if not already chosen.
4. After depth is chosen, follow the `new-feature-roadmap` skill and pass this report as Feature Details.

## Output template

```markdown
### Name:

### Intended Function/Feature:

### Command type (slash / prefix / both):

### Arguments / options:

### Permissions:

### Persistence (none / cache / database):

### Discord interactions (messages / embeds / components / events):

### Symptoms/Behaviors (if update):

### Expected Outcomes:
- Must-have:
- Nice-to-have:

### Suggested reference files (repo-relative):

### Open questions / TBD:
```

## Notes

- Prefer repo-relative paths (`src/...`, `.cursor/...`).
- Do not implement code in this skill.
- Prefer one concise question batch over many single-question turns.
