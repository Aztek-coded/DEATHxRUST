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
2. One suite → one branch `feature/<suite-id>` → one PR. No silent auto-merge (use `/merge-suite-pr` after user confirms).
3. Default parallel cap: 3 suites. Foundations run serial.
4. Ship-ready vertical slices (MVP OK). List deferred commands in the PR.
5. Follow project patterns: Poise, ResponseHelper, EmbedColor, i64 Discord IDs.
6. Discrete modes (`plan`, `implement`, `test`, `review`) do **not** commit, push, merge, or open a PR unless the user allowed that for the run.
7. **`pipeline` and `wave` grant automation up to PR** (see Continuous pipeline). They still must **not** merge to `main`. Land with `/merge-suite-pr` after the user confirms.
8. Never edit hot registration files until the end of an implement pass. Serialize if conflict.

## Continuous pipeline (default for `pipeline` / `wave`)

Goal: run without mid-stage human prompts until a **human gate**.

### Permission contract

When the user runs:

```text
/orchestrate-commands pipeline <id>
/orchestrate-commands wave [N]
```

treat that as **explicit permission** for that run to:

| Allowed | Not allowed |
|---------|-------------|
| Create branch `feature/<id>` | Merge to `main` |
| Create/use worktree (e.g. `.worktrees/<id>`) | Force-push shared branches |
| Implement, test, review (with fix limits) | Skip quality gates |
| Commit suite work on the feature branch | Auto-merge PR |
| Push the feature branch | |
| Open one PR per suite | |
| Set registry `pr_open` + `pr:` URL | |

Optional flags:

| Flag / phrasing | Effect |
|-----------------|--------|
| `--no-pr` or “stop before PR” | Stop after clean review; leave status `reviewing`; no commit/push/PR unless user also asked to commit |
| “no push” | Commit locally only; do not push or open PR |

Do **not** re-ask for branch/worktree/commit/PR permission during `pipeline` / `wave` unless a flag removes PR rights.

### Continuous stage chain

```text
plan → implement → test → review → commit → push → open PR → STOP (human)
```

| Stage | Status | Auto-continue? |
|-------|--------|----------------|
| Plan | `planning` then leave roadmap set | Yes → implement |
| Implement | `implementing` | Yes → test (no extra confirm for worktree) |
| Test | `testing` | Yes → review on success |
| Review | `reviewing` | Yes → commit/push/PR on clean review |
| Open PR | `pr_open` | **STOP** — human PR review + optional Discord test |
| Land | `done` after record | Only after user confirms → `/merge-suite-pr` |

### When to stop early (agent must stop)

Stop and report; set `blocked` with reason when applicable:

1. Suite id unknown or deps not `done`.
2. Test still failing after **2** fix attempts.
3. Critical review findings remain after **one** fix loop + re-test.
4. Path conflict that cannot be serialized safely in this run.
5. Unexpected dirty/foreign worktree risk (do not delete unknown worktrees).

### Hand-off summary (required at stop)

When status becomes `pr_open` or `blocked` / `reviewing` (if `--no-pr`), print:

1. Suite id, branch, worktree path  
2. PR URL (if any)  
3. What passed (fmt / test / clippy / review)  
4. Deferred commands (scope honesty)  
5. **Human next steps:** PR review, optional Discord smoke tests, then `/merge-suite-pr` when ready  

### Resume

If a suite is already mid-pipeline (e.g. `reviewing` with clean review):

```text
/orchestrate-commands pipeline <id>
```

Resume from the next incomplete stage (do not redo plan if roadmap exists; do not re-implement if code is done). User phrasing like “resume \<id\> to PR” means the same: continue to `pr_open`.

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
6. Set `roadmap:` field on the suite. Set status back to `pending` (plan done, not implementing) unless user said to continue or this is inside `pipeline`.

### `implement <id>`

1. Require a roadmap path on the suite or write one first via `plan`.
2. **Discrete mode only:** confirm user allows branch/worktree if not already granted this session. **`pipeline` / `wave`:** create worktree/branch without re-asking.
3. Create worktree/branch `feature/<id>` from latest main.
4. Set status `implementing`.
5. Implement vertical slice only.
6. Register commands last.
7. Run fmt/test/clippy in the worktree.
8. Stop for discrete mode; **continue to test** if `pipeline` / `wave`.

### `test <id>`

1. Set status `testing`.
2. In the suite worktree: `cargo fmt`, `cargo test`, `cargo clippy` (suite-applicable tests; document pre-existing unrelated failures).
3. Up to 2 fix attempts on failure, then set `blocked` with reason.
4. On success: leave status `testing` in discrete mode; **advance to review** if `pipeline` / `wave`.

### `review <id>`

1. Set status `reviewing`.
2. Review diff for security, permissions, registration, ResponseHelper, scope honesty.
3. Critical findings: one fix loop + re-test.
4. On clean review: stop in discrete mode; **commit → push → open PR** if `pipeline` / `wave` (unless `--no-pr`).

### `pipeline <id>`

**Continuous by default.** Run (or resume) until human gate:

```text
plan → implement → test → review → commit → push → open PR → pr_open
```

Skip completed stages when resuming (roadmap present, code present, tests green, review clean).

1. Apply Continuous pipeline permission contract (branch, worktree, commit, push, PR).
2. Execute stages without mid-stage “may I continue?” prompts.
3. Open PR with STE100 title/body: slice commands, deferred list, roadmap link, test notes.
4. Set registry `status: pr_open`, `branch:`, `pr:` URL, `roadmap:`.
5. Print hand-off summary. **Do not merge.**

### `wave [N]`

1. Read `concurrency_cap` (default 3). `N = min(N, cap)`.
2. `next N` for ready suites.
3. Filter to **disjoint** `paths` (no two suites write the same non-null path).
4. If path conflict, drop lower score suite from this wave.
5. Same permission contract as `pipeline` (branch/worktree/commit/push/PR).
6. Run up to N pipelines **in parallel** (subagents/worktrees).
7. Summarize PRs and registry updates; hand-off per suite. **Do not merge.**

### `record-merge <id>`

After the PR is already on `main` (user merged, or `/merge-suite-pr` already merged):

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb record-merge <id> --write
```

Confirm dependents that became ready.

To merge the PR, run registry update, and delete the feature branch in one step, use `/merge-suite-pr` instead.

## Agent prompts (short)

When spawning subagents, include:

- Suite id and registry YAML excerpt
- Roadmap path
- Playbook checklist
- Hard rules above
- Continuous pipeline rules when parent mode is `pipeline` / `wave`
- Exact paths to edit
- "Do not merge to main"

## F0 special case

Suite `agent-orchestration` is docs/skill/registry only. Do not change `src/**/*.rs` for F0.
