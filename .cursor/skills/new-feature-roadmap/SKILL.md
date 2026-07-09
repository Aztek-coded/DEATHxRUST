---
name: new-feature-roadmap
description: >-
  Creates a Discord bot feature implementation roadmap markdown file under
  .cursor/roadmaps/. Use when the user asks for a feature roadmap, implementation
  plan, New-Feature-Roadmap, think-hard / think-hardest planning, or after a
  new-feature-report is completed.
---

# New Feature Roadmap

## Depth

Ask if not specified:

| Depth | When |
|-------|------|
| `standard` | Clear feature, normal planning |
| `think-hard` | Ambiguous design, more alternatives |
| `think-hardest` | High risk / many modules / deep Discord event flows |

Increase analysis depth and alternative consideration accordingly. Output structure is the same.

## Instructions

1. **Feature details** — command name, behavior, slash/prefix, args, permissions, events, expected outcomes.
2. **Guidelines** — align with `.cursor/rules/` (project overview, rust-discord-bot, command-development, data-management).
3. **Required references** — map listed repo files to reuse opportunities.
4. **Optional references** — only if needed; justify briefly.
5. **Implementation analysis**
   - Hypothesize handlers, models, Discord API touchpoints
   - Trace: interaction/event → parse → logic → response
   - Cover async, rate limits, permissions, errors
6. **Roadmap strategy**
   - Modules/files to add or change (repo-relative)
   - Suggested git branch name (e.g. `feature/boost-messages`)
   - `tracing` log points (invoke, API, errors)
   - Verification via logs + Discord behavior
7. **Key changes** — registration, permissions, guild vs global, priority order
8. **Write output** — create `.cursor/roadmaps/<feature-slug>-roadmap.md`
9. **Do not implement code**

## Output file must include

- Feature/command summary
- Discord interaction / event flow
- Explicit modules/commands/handlers with repo-relative paths
- Branch name
- Logging plan (`tracing`)
- Hypothesized approaches
- Step-by-step roadmap (Poise/Serenity aware)
- Alignment notes with project rules

## Feature details input

Use the user's report, or this skeleton:

```markdown
### Command/Feature Name:

### Intended Functionality:
- Command type:
- Description:
- Arguments/options:
- Permissions:

### Discord Interactions:
- Message types:
- Event handling:
- Expected user flow:

### Symptoms/Behaviors (if updating):

### Expected Outcomes:

### Required Files to Analyze:
### Optional Files:
```
