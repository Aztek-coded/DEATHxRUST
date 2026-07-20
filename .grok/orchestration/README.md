# Suite orchestration registry

## Purpose

This folder holds the machine-readable queue for Bleed command parity work.

The human catalog is:

`.grok/roadmaps/full command list roadmap.md`

Do not treat the catalog as the build plan.

## Files

| File | Role |
|------|------|
| `suite-registry.yaml` | Suite list, deps, status, paths |

## Edit rules

1. Keep `version: 1`.
2. Use only allowed status values: `pending`, `planning`, `implementing`, `testing`, `reviewing`, `pr_open`, `done`, `blocked`.
3. When `status` is `blocked`, set a non-empty `blocked_reason`.
4. Prefer suite groups (40–80 suites). Do not add one row per leaf command.
5. After you merge a suite PR, run `record-merge` (see skill).

## Tool commands

From the repo root:

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb validate
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb status
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb next 3
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb record-merge <suite-id> --write
```

## Related docs

- Design: `docs/superpowers/specs/2026-07-19-command-list-orchestration-design.md`
- Playbook: `.grok/guidelines/command-orchestration-playbook.md`
- Skill: `.grok/skills/orchestrate-commands/SKILL.md`
