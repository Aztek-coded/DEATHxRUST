---
name: new-feature-report
description: >-
  Turns a brief Discord bot feature description (and optional screenshots) into
  a structured feature report, then asks which roadmap depth to use. Use when
  the user requests a new feature report, describes a new bot command/feature to
  plan, or mentions NEW-FEATURE-REPORT / feature intake.
---

# New Feature Report

## Instructions

1. Analyze the brief feature description and any screenshots.
2. Fill the report template below (do not invent requirements the user did not imply; mark unknowns clearly).
3. Ask which roadmap depth to use next:
   - **standard** — solid plan
   - **think-hard** — deeper analysis
   - **think-hardest** — maximum rigor
4. After the user picks a depth, follow the `new-feature-roadmap` skill and pass this report as Feature Details.

## Output template

```markdown
### Name:

### Intended Function/Feature:

### Command type (slash / prefix / both):

### Arguments / options:

### Permissions:

### Discord interactions (messages / embeds / components / events):

### Symptoms/Behaviors (if update):

### Expected Outcomes:

### Suggested reference files (repo-relative):
```

## Notes

- Prefer repo-relative paths (`src/...`, `.cursor/...`).
- Do not implement code in this skill.
- If the description is too thin, ask clarifying questions before filling the template.
