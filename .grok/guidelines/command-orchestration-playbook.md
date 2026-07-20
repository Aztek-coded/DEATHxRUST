# Command orchestration playbook

**Product:** DEATHxRUST (Bleed command parity)  
**Audience:** Humans and agents who plan, implement, test, review, or merge suite work  
**Registry:** `.grok/orchestration/suite-registry.yaml`  
**Skill:** `/orchestrate-commands` (`.grok/skills/orchestrate-commands/`)  
**Design:** `docs/superpowers/specs/2026-07-19-command-list-orchestration-design.md`  
**Inventory (human map only):** `.grok/roadmaps/full command list roadmap.md`

Use ASD-STE100 Simplified Technical English in suite roadmaps, PR titles, PR bodies, and acceptance notes.

---

## 1. Purpose

This playbook tells you how to **orchestrate Bleed parity work** as many small, shippable suite slices.

The full command inventory lists about **1144** command names in **36** categories. That inventory is a **backlog map**. It is **not** the implementation plan and it is **not** the orchestrator queue.

The **suite registry** is the source of truth for:

- which suites exist
- dependencies between suites
- status of each suite
- branch and PR fields
- which work is ready next

Orchestration goals:

1. Schedule work by **dependency** and **leverage**.
2. Run plan, implement, test, and review agents in **parallel** only when paths and deps allow it.
3. Use **one git worktree and one PR per suite**.
4. Keep **humans** in control of merges to `main`.
5. Deliver a **vertical slice** that works in Discord, not a full Bleed catalog dump in one PR.

Do not try to implement the full catalog in one roadmap or one session.

---

## 2. Roles

| Role | Who | Inputs | Outputs | Limits |
|------|-----|--------|---------|--------|
| **Coordinator** | Human or skill session running `/orchestrate-commands` | Registry, playbook, user request | Next suites, child agents, status updates, PR instructions | Orchestrates only; does not merge to `main` |
| **Plan** | Plan agent | Suite id, catalog slice, current code | `.grok/roadmaps/<suite-id>-roadmap.md` | Writes roadmap only unless the user expands scope |
| **Implement** | Implement agent | Approved roadmap or explicit MVP scope | Code and docs in a suite worktree | Edits worktree; local `cargo`; does not merge |
| **Test** | Test agent | Worktree / branch | Pass/fail report; fix notes | `cargo fmt`, `clippy`, `test`; up to fix limits in Recovery |
| **Review** | Review agent | Branch or PR diff | Review notes; optional critical fixes | Flags security, permissions, registration gaps |
| **Human merge** | Project owner | Open PR that passed gates | Merge to `main` | Only the human merges; then run `record-merge` |

### Role rules

1. Agents follow existing project patterns: Poise commands, `ResponseHelper`, `EmbedColor`, typed errors, `tracing`, data helpers on `Data`.
2. Status transitions go through the coordinator or registry tool. Implement agents do not invent new status strings.
3. Allowed status values only: `pending`, `planning`, `implementing`, `testing`, `reviewing`, `pr_open`, `done`, `blocked`.
4. When status is `blocked`, set a non-empty `blocked_reason`.
5. Project git rules still apply: do not commit, push, or merge unless the user asks. Modes that open PRs need explicit permission in that run (see Git).

---

## 3. Suite rules

A **suite** is one shippable product slice. It is usually one Bleed parent family or one cog subsystem. It is **not** a whole multi-hundred-command category.

**Examples:** `moderation-core`, `snipe`, `welcome-goodbye`, `boosts`, `starboard`.

**Registry size target:** about **40–80** suite ids for the full program. Leaf command detail stays in the inventory file.

**Score formula (for `next`):**

```text
score = priority + unlock_bonus + infra_reuse_bonus - blocked_penalty
```

**Base priority bands:**

| Band | Score | Examples |
|------|-------|----------|
| Foundations | 1000+ | case store, staff checks, message event router |
| Core moderation | 900 | ban, timeout, warn, purge, history |
| Server glue (existing infra) | 800 | boosts, welcome/goodbye, autorole |
| Message systems | 700 | snipe, stickymessage, filter (after router) |
| Server config utilities | 600 | alias, disablecommand, fakepermissions |
| Community systems | 500 | giveaways, levels, starboard, tickets |
| Safety systems | 450 | antinuke, antiraid (after mod core) |
| Fun / roleplay | 300 | fun, roleplay, manipulation |
| Integrations | 200 | lastfm, music, social embeds |

**MVP bias:** Prefer a working subset of a large Bleed family over incomplete stubs for the whole family.

## Vertical slice checklist

A suite PR is ready when:

1. Commands use Poise and project patterns.
2. Permissions match intent.
3. Data models exist only if needed (Discord IDs as i64).
4. Handlers exist only if needed.
5. Responses use ResponseHelper and EmbedColor.
6. Commands are exported and registered.
7. cargo fmt, clippy, and test pass for the change.
8. Acceptance notes list what works and what is deferred.
9. Deferred Bleed subcommands are listed, not silent gaps.

### Expanded DoD notes

Use the checklist above as the gate. Apply these details when you plan and review:

- **Commands:** Parent and implemented subcommands use `#[poise::command(...)]`, correct category, and catalog aliases where listed (1-letter shortcuts when frequent and non-conflicting).
- **Permissions:** Discord flags and/or staff/booster checks match intent. Do not leave admin tools as “permission none”.
- **Data:** Tables, models, and helpers only if the slice needs them. Prefer UPSERT. Store Discord IDs as `i64`.
- **Handlers:** Event hooks only if required. Do not hold write locks across Discord API awaits.
- **Responses:** Always `ResponseHelper` and `EmbedColor`. Do not use raw hex for embeds.
- **Registration:** Export in `src/commands/mod.rs` and register in `src/bot/framework.rs`. If hot-file conflict occurs, use a deferred registration commit (see Parallelism).
- **Quality:** `cargo fmt`, `cargo clippy` (no new errors; reduce new warnings in touched code where practical), `cargo test` green for applicable workspace tests.
- **Docs:** Short acceptance notes in the suite roadmap or PR body.
- **Scope honesty:** List deferred Bleed subcommands as out of slice.

### Quality gates before `pr_open`

1. `cargo test` passes.
2. `cargo clippy` has no errors.
3. Review agent reports **no critical** issues (security, data loss, silent missing registration).
4. Registry has branch and PR fields set.

Full Discord E2E automation is **not** required for every suite unless the project already covers that path.

---

## 4. Foundations

Foundation suites run **serially** before product suites that depend on them. The `wave` mode never schedules a suite whose dependencies are incomplete.

| ID | Registry id | Purpose | Unlocks |
|----|-------------|---------|---------|
| **F0** | `agent-orchestration` | Coordinator skill, suite registry, this playbook, catalog bootstrap tooling | The whole orchestration program |
| **F1** | `moderation-foundation` | Case log model, reason helpers, staff permission check, shared mod embeds | ban, timeout, warn, jail, history, modstats, and related mod suites |
| **F2** | `message-foundation` | Shared message create/update/delete routing hooks | snipe, sticky, filter, autoresponder, levels XP, and related message suites |

### Foundation rules

1. Complete F0 before you treat product orchestration as ready.
2. Write a focused roadmap for F1 and F2 before implement.
3. Product suites that list a foundation in `depends_on` stay out of the ready queue until that foundation status is `done`.
4. F0 itself must not change Discord bot application code under `src/**/*.rs` except if a later revision of F0 explicitly requires it. Default F0 is docs, skill, registry, and scripts only.

---

## 5. Waves

Waves are **recommended defaults**. Live scheduling always uses the registry score and dependency graph.

| Wave | Focus | Parallelism |
|------|--------|-------------|
| **0** | F0 orchestration (skill, registry, playbook) | Serial (1) |
| **1** | F1 + moderation punish MVP + history | Serial F1, then 1–2 mod suites |
| **2** | F2 + boosts, welcome/goodbye | Up to 3 after F2 |
| **3** | snipe, stickymessage; light utils (firstmessage, pin) | Up to 3 |
| **4+** | Expand mod, starboard, giveaways, and other scored suites | Up to concurrency cap |

### Suggested Wave 1 product suites (after F1)

1. **`moderation-punish`** — `timeout`, `untimeout`, `ban`, `unban`, `softban`, `warn`, `warnings` plus case writes.
2. **`moderation-purge-basic`** — `purge` by count plus bots/humans (not the full filter set).
3. **`moderation-history`** — `history`, `caselog`, `reason`.

### Bootstrap rule for the catalog

1. Seed the registry with high-value suites first (foundations + Waves 1–3).
2. Add remaining catalog parents as suite stubs over time with `sync-catalog` (report missing suites; do not auto-implement).
3. Keep leaf arguments and aliases in the inventory.
4. Keep the registry at about 40–80 suite ids for the full map.

---

## 6. Parallelism

Default **concurrency cap: 3** suite pipelines at once. The registry field is `concurrency_cap`. The coordinator must not exceed this cap.

### Safe parallel conditions

Run suite pipelines in parallel only when **all** of these hold:

1. Every suite in the set has all `depends_on` entries at status `done`.
2. Suite `paths` (commands, data, handlers) are **disjoint** across the set.
3. No suite is `blocked`.
4. Foundations that the set needs are already `done`.

### Serial rules

1. Foundation suites (F0, F1, F2) run **serial**.
2. If two pipelines share a path, the coordinator serializes them or picks the higher score suite first.
3. **Hot files** (registration and module roots):
   - `src/bot/framework.rs`
   - `src/commands/mod.rs`
   - `src/data/mod.rs`
   - `src/handlers/mod.rs`
4. Registration is the **last** implement step for a suite.
5. If two pipelines conflict on hot files, the coordinator **serializes** registration or applies a small registration-only follow-up on the losing branch.
6. Prefer additive registration (append a command to the list) to reduce merge pain.
7. Optional sequential test queue is allowed if parallel `cargo` thrash hurts the machine.

### How the coordinator picks work

1. Run `status` to see done, in progress, ready, blocked, and waiting on deps.
2. Run `next N` with N ≤ concurrency cap (default 3).
3. Prefer higher score when several suites are ready and paths are disjoint.
4. Do not schedule work that is waiting on deps.

---

## 7. Git

| Rule | Detail |
|------|--------|
| Base branch | `main` (or a user-specified integration branch later) |
| Feature branch | `feature/<suite-id>` |
| Worktree | Prefer `.worktrees/<suite-id>` or the git worktree path documented in the skill |
| PR shape | **One suite → one PR** |
| PR language | STE100; list commands in the slice; list deferred items; link the suite roadmap |
| Merge | **No auto-merge.** The user merges. |
| After merge | Run `record-merge <suite-id>` so status becomes `done` and dependents unlock |
| Force-push | Do not force-push shared branches. Force-push only on unshared feature branches if the user allows it in that session |
| Commit default | Do not commit, push, or merge unless the user asks (see `AGENTS.md`) |

### Orchestration permission note

When the user runs `/orchestrate-commands pipeline` or `wave`, that can mean explicit permission to create branches, worktrees, and open PRs **if** the skill documents that those modes create PRs. That is **not** permission to merge to `main`. Prefer to ask once at the start of a `wave` whether PR creation is allowed for that run.

### Dirty worktree policy

1. Do not delete an unknown worktree without a human check.
2. Document the path and the suite id.
3. Prefer rebase onto latest `main` when hot-file conflicts appear.
4. After a successful human merge, run `record-merge` even if the worktree still exists; then clean the worktree only when the user agrees.

---

## 8. Pipeline stages

Stages for one suite run **in order**. Parallelism applies across **independent suites**, not across stages of the same suite.

```text
plan → implement → test → review → open PR → (human merge) → record-merge
```

| Stage | Status to set | Action |
|-------|---------------|--------|
| Plan | `planning` → then leave roadmap path set | Produce `.grok/roadmaps/<suite-id>-roadmap.md` with MVP scope, paths, deferred list |
| Implement | `implementing` | Create worktree + branch `feature/<suite-id>`; implement vertical slice |
| Test | `testing` | Run `cargo fmt`, `cargo clippy`, `cargo test`; fix within recovery limits |
| Review | `reviewing` | Security, permissions, registration, DoD checklist |
| Open PR | `pr_open` | Set `branch` and `pr` fields; STE100 PR body |
| Human merge | (unchanged until record) | User merges on the host (GitHub/GitLab/etc.) |
| Record merge | `done` | `record-merge <suite-id>`; unlock dependents |

### Skill modes (coordinator)

| Mode | Behavior |
|------|----------|
| `status` | Show done / in progress / ready / blocked / waiting on deps |
| `sync-catalog` | Diff inventory vs registry; flag missing suites; do not auto-implement |
| `next [N]` | Pick up to N ready suites by score and deps |
| `plan <id>` | Spawn plan agent → focused roadmap |
| `implement <id>` | Worktree + implement agent (needs roadmap or explicit MVP scope) |
| `test <id>` | Test agent on that worktree |
| `review <id>` | Review agent on branch / PR |
| `pipeline <id>` | plan → implement → test → review → open PR (one suite, serial stages) |
| `wave [N]` | Run `pipeline` on up to N independent ready suites in parallel (N ≤ concurrency cap) |
| `record-merge <id>` | Mark done after the user merges |

### Testing layers

| Layer | Who | What |
|-------|-----|------|
| Unit / pure logic | Implement + test agents | Parsing, permission helpers, case ID format, filter matchers |
| Integration (local) | Test agent | `cargo test`, clippy, fmt |
| Manual Discord | User or beta notes | High-risk mod actions, event-driven features |
| Review | Review role | Mentions, length limits, wrong permissions, missing registration |

---

## 9. Recovery

| Failure | Recovery |
|---------|----------|
| Plan incomplete or vague | Re-run plan agent with critique notes. Do **not** implement. |
| Implement build fail | Test agent or implement fix loop up to **2** automatic fix attempts; then set status `blocked` with a log path and reason |
| Hot-file merge conflict | Serial registration follow-up branch; or rebase the worktree on latest `main` |
| Review critical findings | One fix loop; re-test; re-review |
| Dependency merge breaks API | Mark dependents `blocked` until the foundation suite is revised |
| Agent worktree dirty or stuck | Document path and suite id; human decides reset or continue; never delete unknown work without check |
| Catalog and registry drift | Run `sync-catalog` for a report only; human or coordinator adds suite stubs |
| Registry write error | Re-validate with `registry_tool.rb validate`; fix YAML; do not leave partial status writes |

### Status discipline

1. Status values must stay in the allowed enum.
2. The coordinator (or `record-merge` / skill write paths) owns status transitions.
3. Blocked suites must keep a non-empty `blocked_reason`.
4. Do not set `done` until the user has merged the suite PR (or the user confirms the work landed on the base branch without a PR in an exceptional case—still run `record-merge`).

---

## 10. Commands

Run tool commands from the **repository root**.

### Registry tool

Script path:

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb
```

| Command | Purpose |
|---------|---------|
| `validate` | Check schema, statuses, deps, and required fields |
| `status` | Print counts and suite lists by state |
| `next [N]` | Print up to N ready suites (highest score first; default N from tool or 1) |
| `record-merge <suite-id> --write` | Set suite status to `done` after human merge |

Examples:

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb validate
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb status
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb next 3
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb record-merge moderation-punish --write
```

Optional registry path argument: pass the YAML path if you do not use the default `.grok/orchestration/suite-registry.yaml` (see skill / script help).

Tests for the tool:

```bash
ruby .grok/skills/orchestrate-commands/scripts/test_registry_tool.rb
```

### `/orchestrate-commands` modes

Invoke the skill as `/orchestrate-commands` (or via project skill discovery) with a mode:

| Invocation | Effect |
|------------|--------|
| `/orchestrate-commands status` | Summarize registry via tool + human-readable notes |
| `/orchestrate-commands sync-catalog` | Report inventory vs registry gaps |
| `/orchestrate-commands next [N]` | Propose next ready suites (respect concurrency and disjoint paths) |
| `/orchestrate-commands plan <suite-id>` | Plan agent → roadmap |
| `/orchestrate-commands implement <suite-id>` | Worktree + implement |
| `/orchestrate-commands test <suite-id>` | Quality gates on the suite worktree |
| `/orchestrate-commands review <suite-id>` | Diff review against DoD |
| `/orchestrate-commands pipeline <suite-id>` | Full serial pipeline for one suite through PR |
| `/orchestrate-commands wave [N]` | Up to N parallel pipelines for independent ready suites |
| `/orchestrate-commands record-merge <suite-id>` | After human merge; update registry |

### Layout reference

| Artifact | Path |
|----------|------|
| This playbook | `.grok/guidelines/command-orchestration-playbook.md` |
| Suite registry | `.grok/orchestration/suite-registry.yaml` |
| Registry folder README | `.grok/orchestration/README.md` |
| Coordinator skill | `.grok/skills/orchestrate-commands/SKILL.md` |
| Registry tool | `.grok/skills/orchestrate-commands/scripts/registry_tool.rb` |
| Suite roadmaps | `.grok/roadmaps/<suite-id>-roadmap.md` |
| Inventory | `.grok/roadmaps/full command list roadmap.md` |
| Design | `docs/superpowers/specs/2026-07-19-command-list-orchestration-design.md` |

---

## 11. Out of scope

Do **not** schedule or implement these until their foundation or product design is unblocked on purpose:

| Area | Why it is out of scope now |
|------|----------------------------|
| Music systems | Needs external APIs and media stack (`music-core` and related) |
| Last.fm | Integration foundation and API credentials (`lastfm-core`) |
| Tier-2 / premium (`prefix self` and similar) | Needs premium tier design |
| Full filter / antinuke | Needs F2 and dedicated suite design |
| Jail / mute role systems | Needs F1 plus channel overwrite helpers |
| Settings Phase 3 (modlog, jail, mute, music) | Depends on the respective systems |
| Deep Spotify and other media embeds | External APIs + media stack |
| Exact Bleed behavior when it conflicts with DEATHxRUST patterns | Prefer Poise, `ResponseHelper`, staff DB, and project rules |
| Auto-merge to `main` | Humans always merge |
| Treating the catalog markdown as the executable build plan | Registry is the queue |

When a suite must stay blocked, set `status: blocked` and a clear `blocked_reason` in the registry. Do not leave silent gaps in the inventory without a registry note.

---

## Quick start (human)

1. Open the repo root.
2. Run `ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb validate`.
3. Run `status` and `next 3`.
4. If no suite is ready, finish foundations (F0 → F1 / F2 as needed) or clear blocked reasons.
5. For one suite: run `pipeline <id>` (or plan, implement, test, review by hand).
6. Open the PR if the mode did not open it.
7. Merge only when you accept the PR.
8. Run `record-merge <id> --write`.
9. Re-run `next` for the unlocked dependents.

For parallel product work after foundations: run `wave` with N ≤ 3 and only disjoint ready suites.
