---
name: orchestrate-commands
description: >
  Coordinate multi-agent Bleed command parity work using the suite registry,
  worktrees, and plan/implement/test/review pipelines. Use when the user runs
  /orchestrate-commands, asks to orchestrate command suites, run a command wave,
  pick the next suite, update suite status, or parallelize Bleed parity work.
---

# Orchestrate Commands

Drive DEATHxRUST Bleed parity using the suite registry. Do not implement the full catalog in one pass.

## Canonical paths

| Item | Path |
|------|------|
| Registry | `.grok/orchestration/suite-registry.yaml` |
| Inventory | `.grok/roadmaps/full command list roadmap.md` |
| Playbook | `.grok/guidelines/command-orchestration-playbook.md` |
| Tool | `.grok/skills/orchestrate-commands/scripts/registry_tool.rb` |
| Design | `docs/superpowers/specs/2026-07-19-command-list-orchestration-design.md` |
| Suite roadmaps | `.grok/roadmaps/<suite-id>-roadmap.md` |

## Hard rules

1. Registry is the queue source of truth. Inventory is human reference.
2. One suite → one branch `feature/<suite-id>` → one PR. No auto-merge.
3. Default parallel cap: 3 suites. Foundations run serial.
4. Ship-ready vertical slices (MVP OK). List deferred commands in the PR.
5. Follow project patterns: Poise, ResponseHelper, EmbedColor, i64 Discord IDs.
6. Do not commit, push, merge, or open a PR unless the user allowed it for this run.
7. When the user invokes `pipeline` or `wave` and allows PRs, you may open PRs; you still must not merge.
8. Never edit hot registration files until the end of an implement pass. Serialize if conflict.

## Modes

Parse the user message for a mode. If none, run `status` then ask which mode.

### `status`

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb status
```

Report buckets and ready queue to the user.

### `sync-catalog`

1. Read category index in the inventory.
2. Compare parent command families to registry `commands` / suite ids.
3. Report missing suite stubs only. Do not implement. Do not auto-add rows unless the user asks.

### `next [N]`

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb next N
```

Explain why each suite is ready (deps done, score).

### `plan <id>`

1. Load suite from registry. Fail if unknown.
2. Set status to `planning` in registry (edit YAML carefully).
3. Read inventory entries for suite `commands`.
4. Read existing code paths if present.
5. Write `.grok/roadmaps/<id>-roadmap.md` with: goal, MVP command list, files to touch, data/handlers, acceptance, deferred list, branch name.
6. Set `roadmap:` field on the suite. Set status back to `pending` (plan done, not implementing) unless user said to continue.

### `implement <id>`

1. Require a roadmap path on the suite or write one first via `plan`.
2. Confirm user allows branch/worktree for this run.
3. Create worktree/branch `feature/<id>` from latest main.
4. Set status `implementing`.
5. Implement vertical slice only.
6. Register commands last.
7. Run fmt/test/clippy in the worktree.
8. Stop for test/review modes or continue if `pipeline`.

### `test <id>`

1. Set status `testing`.
2. In the suite worktree: `cargo fmt`, `cargo test`, `cargo clippy`.
3. Up to 2 fix attempts on failure, then set `blocked` with reason.
4. On success, leave status `testing` or advance to review if pipeline.

### `review <id>`

1. Set status `reviewing`.
2. Review diff for security, permissions, registration, ResponseHelper, scope honesty.
3. Critical findings: one fix loop + re-test.
4. On clean review, proceed to PR if allowed.

### `pipeline <id>`

Run plan → implement → test → review → open PR (if allowed) → set `pr_open` and `pr:` URL.

### `wave [N]`

1. Read `concurrency_cap` (default 3). `N = min(N, cap)`.
2. `next N` for ready suites.
3. Filter to **disjoint** `paths` (no two suites write the same non-null path).
4. If path conflict, drop lower score suite from this wave.
5. Run up to N pipelines **in parallel** (subagents/worktrees) only when user allows.
6. Summarize PRs and registry updates.

### `record-merge <id>`

After the user merges:

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb record-merge <id> --write
```

Confirm dependents that became ready.

## Agent prompts (short)

When spawning subagents, include:

- Suite id and registry YAML excerpt
- Roadmap path
- Playbook checklist
- Hard rules above
- Exact paths to edit
- "Do not merge to main"

## F0 special case

Suite `agent-orchestration` is docs/skill/registry only. Do not change `src/**/*.rs` for F0.
