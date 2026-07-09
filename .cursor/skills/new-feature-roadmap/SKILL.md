---
name: new-feature-roadmap
description: >-
  Creates a Discord bot feature implementation roadmap markdown file under
  .cursor/roadmaps/ after confirming required context (or running intake). Use
  when the user asks for a feature roadmap, implementation plan,
  New-Feature-Roadmap, think-hard / think-hardest planning, or after a
  new-feature-report is completed.
---

# New Feature Roadmap

## Interactive gate (required first)

Do **not** write the roadmap until required fields are present.

### If a feature report exists

Confirm or ask only for gaps:

1. Report path / paste is complete?
2. **Depth:** `standard` / `think-hard` / `think-hardest`?
3. Any hard constraints (out of scope, must reuse X, no new tables, etc.)?
4. Preferred branch name (or accept derived `feature/<slug>`)?

### If no report (or report is thin)

Either:

- Run the `new-feature-report` skill intake first, **or**
- Ask this mini-intake in one batch:

1. Feature name and one-sentence goal?
2. New vs update?
3. Slash / prefix / both?
4. Permissions?
5. Arguments / options?
6. Persistence (none / cache / DB)?
7. Events vs command-only?
8. UX (text / embeds / components)?
9. Must-have outcomes?
10. Required reference files (repo-relative)?
11. Depth: `standard` / `think-hard` / `think-hardest`?
12. Branch name preference?

Refuse to invent product decisions; use `TBD` only for non-blocking details and list them in the roadmap.

## Depth

| Depth | When |
|-------|------|
| `standard` | Clear feature, normal planning |
| `think-hard` | Ambiguous design, more alternatives |
| `think-hardest` | High risk / many modules / deep Discord event flows |

Increase analysis depth and alternative consideration accordingly. Output structure is the same.

## Instructions

1. Pass the interactive gate above.
2. **Feature details** — command name, behavior, slash/prefix, args, permissions, events, expected outcomes.
3. **Guidelines** — align with `.cursor/rules/` (project overview, rust-discord-bot, command-development, data-management).
4. **Required references** — map listed repo files to reuse opportunities.
5. **Optional references** — only if needed; justify briefly.
6. **Implementation analysis**
   - Hypothesize handlers, models, Discord API touchpoints
   - Trace: interaction/event → parse → logic → response
   - Cover async, rate limits, permissions, errors
7. **Roadmap strategy**
   - Modules/files to add or change (repo-relative)
   - Git branch name
   - `tracing` log points (invoke, API, errors)
   - Verification via logs + Discord behavior
8. **Key changes** — registration, permissions, guild vs global, priority order
9. **Write output** — create `.cursor/roadmaps/<feature-slug>-roadmap.md`
10. **Do not implement code**

## Output file must include

- Feature/command summary
- Discord interaction / event flow
- Explicit modules/commands/handlers with repo-relative paths
- Branch name
- Logging plan (`tracing`)
- Hypothesized approaches
- Step-by-step roadmap (Poise/Serenity aware)
- Alignment notes with project rules
- Open questions / TBD (if any)

## Feature details input

Use the completed report, or this skeleton after intake:

```markdown
### Command/Feature Name:

### Intended Functionality:
- Command type:
- Description:
- Arguments/options:
- Permissions:
- Persistence:

### Discord Interactions:
- Message types:
- Event handling:
- Expected user flow:

### Symptoms/Behaviors (if updating):

### Expected Outcomes:
- Must-have:
- Nice-to-have:

### Required Files to Analyze:
### Optional Files:
### Constraints / out of scope:
```
