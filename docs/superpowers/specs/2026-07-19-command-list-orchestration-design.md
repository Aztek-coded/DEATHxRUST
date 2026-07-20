# Design: Full Command List Multi-Agent Orchestration

**Date:** 2026-07-19  
**Status:** Draft for user review  
**Product:** DEATHxRUST (Bleed command parity program)  
**Inventory source:** `.grok/roadmaps/full command list roadmap.md`  
**Related:** `.grok/enhancements/bleed-bot-commands-enhancements.md`

---

## 1. Problem

DEATHxRUST has a Bleed parity inventory of about **1144** commands in **36** categories. About **2.6%** of catalog names are implemented. The inventory is a backlog map, not an implementation plan.

Building the full list as one roadmap or one session is not workable. The need is an **orchestration system** that:

- Schedules work by **dependency and leverage**
- Runs **plan / implement / test / review** agents in **parallel** when safe
- Uses **git worktrees** and **one PR per suite**
- Keeps humans in control of merges to `main`

---

## 2. Goals and non-goals

### Goals

1. Ship a **reusable Grok coordinator skill** that drives parity work with many agents.
2. Keep a **machine-readable suite registry** (priority, dependencies, status, paths, PR).
3. Use **worktree + PR per suite**; agents do not land on `main` without a PR and human merge.
4. Schedule by **dependency + leverage** (foundations first, then high-unlock suites).
5. Deliver **ship-ready vertical slices** per suite PR (usable MVP, not always full Bleed subcommand parity).
6. Provide a **playbook** (waves, agent roles, git rules, recovery) in clear technical English.

### Non-goals

1. Implement all ~1111 remaining commands in one design or one session.
2. Exact Bleed behavior when it conflicts with DEATHxRUST patterns (Poise, `ResponseHelper`, staff DB).
3. Auto-merge to `main`.
4. Music / Last.fm / Tier-2 premium systems until their foundation suites are scheduled on purpose.
5. Treat the catalog markdown as the executable build plan.

### Invariants

- Inventory stays the human parity map. The **registry** is the orchestrator source of truth for queues.
- Project git rules still apply: no commit, push, or merge unless the user asks. Agents prepare PRs; the user merges.
- Follow existing layout: `src/commands/<suite>/`, registration in `framework.rs` and `commands/mod.rs`, SQLite Discord IDs as `i64`, handlers under `src/handlers/`.

---

## 3. Architecture

### 3.1 Components

| Piece | Role |
|--------|------|
| Catalog inventory | `.grok/roadmaps/full command list roadmap.md` — human map; not executed directly |
| Suite registry | Machine-readable suite list: deps, priority, status, paths, branch, PR |
| Coordinator skill | Grok skill: pick work, spawn agents, update registry, report status |
| Playbook | Waves, foundations, roles, git rules, recovery |
| Suite agents | Plan → implement → test → review (parallel when independent) |
| Git worktrees | One worktree and branch per suite implementation |

### 3.2 Data flow

```text
Inventory (1144 commands)
        │
        ▼ bootstrap / sync
Suite registry (suites, DAG, status)
        │
        ▼
Coordinator skill
   ├── next: select ready suites (deps met, not blocked)
   ├── plan agent     → .grok/roadmaps/<id>-roadmap.md
   ├── implement agent → worktree, vertical slice
   ├── test agent     → cargo test / clippy, report
   ├── review agent   → findings; fix loop if critical
   └── open PR + registry update → status = pr_open
        │
        ▼
User merges PR
        │
        ▼
Coordinator: status = done; unlock dependents
```

### 3.3 Suite (unit of work)

A **suite** is one shippable product slice. It is usually one Bleed parent family or one cog subsystem. It is **not** a whole 300-command category.

**Examples:** `moderation-core`, `snipe`, `welcome-goodbye`, `boosts`, `starboard`.

**Registry entry shape (conceptual):**

```yaml
id: moderation-punish
category: moderation
title: Core punish commands
commands:
  - timeout
  - untimeout
  - ban
  - unban
  - softban
  - warn
  - warnings
depends_on:
  - moderation-foundation
priority: 900
status: pending   # pending | planning | implementing | testing | reviewing | pr_open | done | blocked
blocked_reason: null
paths:
  commands: src/commands/moderation/
  data: src/data/models/moderation.rs
  handlers: null
branch: feature/moderation-punish
pr: null
roadmap: .grok/roadmaps/moderation-punish-roadmap.md
notes: "MVP: case writes; no hardban/jail yet"
```

**Rule:** Registry tracks **suites** (about 40–80). Leaf command detail stays in the inventory file.

### 3.4 Coordinator skill modes

| Mode | Behavior |
|------|----------|
| `status` | Show done / in progress / ready / blocked |
| `sync-catalog` | Diff inventory vs registry; flag missing suites (no auto-implement) |
| `next [N]` | Pick up to N ready suites by score and deps |
| `plan <id>` | Spawn plan agent → focused roadmap |
| `implement <id>` | Worktree + implement agent (needs roadmap or explicit MVP scope) |
| `test <id>` | Test agent on that worktree |
| `review <id>` | Review agent on branch / PR |
| `pipeline <id>` | plan → implement → test → review → open PR (one suite, serial stages) |
| `wave [N]` | Run `pipeline` on up to N independent ready suites in parallel |
| `record-merge <id>` | Mark done after the user merges |

**Skill location (target):** `.grok/skills/orchestrate-commands/SKILL.md`  
**Invocation name:** `/orchestrate-commands` (or project skill discovery name)

### 3.5 Parallelism rules

1. Parallel only if suite `paths` are **disjoint** and all `depends_on` are **done**.
2. Foundation suites run **serial** first.
3. Default concurrency cap: **3** suite pipelines (configurable in skill/playbook).
4. Hot files (`src/bot/framework.rs`, `src/commands/mod.rs`, `src/data/mod.rs`, `src/handlers/mod.rs`):
   - Registration is the **last** implement step.
   - If two pipelines conflict, the coordinator **serializes** registration or applies a small registration-only follow-up on the losing branch.
   - Prefer additive registration (append command to vec) to reduce merge pain.

### 3.6 Agent roles

| Role | Inputs | Outputs | Capability |
|------|--------|---------|------------|
| Plan | Suite id, catalog slice, code | `.grok/roadmaps/<id>-roadmap.md` | Read + write roadmap only |
| Implement | Approved roadmap | Code in worktree | Edit + local cargo |
| Test | Worktree | Pass/fail report, fix notes | cargo test / clippy / fmt |
| Review | Branch / diff | Review notes; optional critical fixes | Read + optional fix |
| Coordinator | Registry | Child agents, status, PR instructions | Orchestrate only |

Agents use existing project patterns: Poise commands, `ResponseHelper`, `EmbedColor`, typed errors, `tracing`, data helpers on `Data`.

---

## 4. Foundations, waves, and scheduling

### 4.1 Priority scoring

```text
score = base_priority
      + unlock_bonus          # how many suites depend on this
      + infra_reuse_bonus     # extends BoostHandler / MemberHandler / existing data
      - blocked_penalty       # external API or large unknown systems
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

### 4.2 Foundation suites (serial)

| ID | Purpose | Unlocks |
|----|---------|---------|
| **F0 `agent-orchestration`** | Skill + registry seed + playbook + catalog bootstrap | This whole program |
| **F1 `moderation-foundation`** | Case log model, reason helpers, staff permission check, shared mod embeds | ban/timeout/warn/jail/history/modstats |
| **F2 `message-foundation`** | Shared message create/update/delete routing hooks | snipe, sticky, filter, autoresponder, levels XP |

**Rule:** `wave` never schedules a suite with incomplete dependencies.

### 4.3 Default waves (playbook)

| Wave | Focus | Parallelism |
|------|--------|-------------|
| **0** | F0 orchestration (skill, registry, playbook) | Serial (1) |
| **1** | F1 + moderation punish MVP + history | Serial F1, then 1–2 mod suites |
| **2** | F2 + boosts, welcome/goodbye | Up to 3 after F2 |
| **3** | snipe, stickymessage; light utils (firstmessage, pin) | Up to 3 |
| **4+** | Expand mod, starboard, giveaways, etc. by score | Up to concurrency cap |

Waves are **recommended defaults**. Live scheduling uses registry + score.

### 4.4 Suggested Wave 1 product suites (after F1)

1. **`moderation-punish`** — `timeout`, `untimeout`, `ban`, `unban`, `softban`, `warn`, `warnings` + case writes  
2. **`moderation-purge-basic`** — `purge` by count + bots/humans (not full filter set)  
3. **`moderation-history`** — `history`, `caselog`, `reason`

### 4.5 Explicitly blocked until foundations exist

| Area | Blocked on |
|------|------------|
| Jail / mute role systems | F1 + channel overwrite helpers |
| Settings Phase 3 (modlog, jail, mute, music) | Respective systems |
| `prefix self` / Tier 2 | Premium tier design |
| Music, Last.fm, deep Spotify | External APIs + media stack |
| Full filter / antinuke | F2 + dedicated suite design |

### 4.6 Catalog → registry bootstrap

On F0 implement:

1. Seed registry with high-value suites first (foundations + Waves 1–3), not all 1144 leaves.
2. Group remaining catalog parents into suite stubs with `status: pending` and low or medium priority over time via `sync-catalog`.
3. Keep leaf arguments/aliases in the inventory; registry holds suite membership lists.
4. Target **about 40–80** suite ids for the full program map.

---

## 5. Vertical slice definition of done

A suite is ready for PR when **all** of the following hold:

1. **Commands:** Parent and implemented subcommands use `#[poise::command(...)]`, correct category, aliases where catalog lists them (1-letter shortcuts when frequent and non-conflicting).
2. **Permissions:** Discord flags and/or staff/booster checks match intent; no “permission none” for admin tools.
3. **Data:** Tables/models/helpers only if needed; UPSERT patterns; IDs as `i64`.
4. **Handlers:** Event hooks only if required for the slice; no hold of write locks across Discord awaits.
5. **Responses:** `ResponseHelper` + `EmbedColor`; no raw hex embeds.
6. **Registration:** Exported in `commands/mod.rs` and registered in `framework.rs` (or deferred registration commit if hot-file conflict).
7. **Quality:** `cargo fmt`, `cargo clippy` (no new warnings in touched code where practical), `cargo test` green for workspace tests that apply.
8. **Docs for the suite:** Short acceptance notes in the suite roadmap or PR body (what works, what is deferred).
9. **Scope honesty:** Deferred Bleed subcommands listed as out of slice, not silently missing.

**MVP bias:** Prefer a working subset of a large Bleed family over incomplete stubs for the whole family.

---

## 6. Testing strategy

| Layer | Who | What |
|-------|-----|------|
| Unit / pure logic | Implement + test agents | Parsing, permission helpers, case ID format, filter matchers |
| Integration (local) | Test agent | `cargo test`, clippy, fmt |
| Manual Discord | User or beta notes | High-risk mod actions, event-driven features |
| Review agent | Review role | Security (mentions, length), wrong permissions, missing registration |

**Gates before `pr_open`:**

1. `cargo test` passes  
2. `cargo clippy` has no errors  
3. Review agent reports **no critical** issues (security, data loss, silent no-op registration)  
4. Registry updated with branch and PR fields  

**Not required for every suite:** Full Discord E2E automation (out of scope unless the project already has harness coverage for that path).

---

## 7. Error recovery

| Failure | Recovery |
|---------|----------|
| Plan incomplete / vague | Re-run plan agent with critique notes; do not implement |
| Implement build fail | Test agent (or implement fix loop) up to **2** automatic fix attempts; then status `blocked` with log path |
| Hot-file merge conflict | Serial registration follow-up branch; or rebase worktree on latest main |
| Review critical findings | One fix loop; re-test; re-review |
| Dependency merged breaking API | Mark dependents `blocked` until foundation suite revision |
| Agent worktree dirty / stuck | Document path; human or `record-merge` / reset policy in playbook; never delete unknown work without check |
| Catalog and registry drift | `sync-catalog` report only; human or coordinator adds suite stubs |

**Status values must stay consistent.** Coordinator writes status transitions; implement agents do not invent new statuses outside the enum.

---

## 8. File layout (targets)

| Artifact | Path |
|----------|------|
| This design | `docs/superpowers/specs/2026-07-19-command-list-orchestration-design.md` |
| Playbook | `.grok/guidelines/command-orchestration-playbook.md` |
| Coordinator skill | `.grok/skills/orchestrate-commands/SKILL.md` |
| Skill helpers (optional) | `.grok/skills/orchestrate-commands/scripts/` |
| Suite registry | `.grok/orchestration/suite-registry.yaml` |
| Suite roadmaps | `.grok/roadmaps/<suite-id>-roadmap.md` |
| Inventory (unchanged role) | `.grok/roadmaps/full command list roadmap.md` |

**Registry format:** YAML preferred for human edits. If tooling needs JSON later, generate from YAML; do not maintain two sources of truth.

---

## 9. Git and PR policy

1. Base branch: `main` (or user-specified integration branch later).  
2. Branch name: `feature/<suite-id>`.  
3. Worktree path: project convention under `.worktrees/<suite-id>` or git default worktree path documented in the skill.  
4. One suite → one PR.  
5. PR title/body: STE100; list commands in slice; list deferred items; link suite roadmap.  
6. **No auto-merge.** User merges. Then `record-merge <id>`.  
7. Do not force-push shared branches. Force-push only on unshared feature branches if the user allows in that session.  
8. Commit only when the user asks (or when a skill step explicitly includes commit after user confirmation). Default for agents: leave commits ready if the user’s agent rules require explicit commit requests—follow DEATHxRUST Agents.md: **do not commit/push/merge unless asked**.

**Clarification for orchestration skill:** When the user runs `/orchestrate-commands pipeline` or `wave`, that is explicit permission to create branches/worktrees and open PRs **if** the skill documents that those modes create PRs. It is **not** permission to merge. Prefer asking once at the start of a `wave` if PR creation is allowed for that run.

---

## 10. Success criteria

### For F0 (this design’s first implementation)

1. Coordinator skill exists and documents all modes in Section 3.4.  
2. Registry file exists with foundations + Wave 1–3 suite seeds.  
3. Playbook exists and matches this design.  
4. `status`, `next`, and dry-run `wave` (plan-only or print plan) work without breaking the bot.  
5. At least one end-to-end dry run documented: pick suite → plan artifact path → (optional) implement worktree path.

### For the program (ongoing)

1. Ready queue is always explainable from registry deps + score.  
2. Parallel waves do not corrupt `main`.  
3. Each merged suite updates inventory status markers (manual or scripted) over time.  
4. Blocked suites have a non-empty `blocked_reason`.

---

## 11. Implementation order for F0 (after this spec is approved)

This design’s first build is **only F0**. Command suites come after F0 via the skill.

1. Create `.grok/orchestration/suite-registry.yaml` with F0–F2 and Wave 1–3 seeds.  
2. Write playbook `.grok/guidelines/command-orchestration-playbook.md`.  
3. Write skill `.grok/skills/orchestrate-commands/SKILL.md` (and optional helper scripts).  
4. Wire skill discovery if the project indexes skills explicitly.  
5. Manual verification: `status`, `next`, `plan` on one dummy or real foundation suite.  
6. Do **not** implement moderation or message foundations until F0 skill is usable—unless the user starts a separate `implement-roadmap` for F1.

---

## 12. Risks and mitigations

| Risk | Mitigation |
|------|------------|
| Scope explosion to 1144 leaves | Suites only; MVP slices; inventory stays reference |
| Merge hell on registration files | Disjoint paths; serial registration; small last commits |
| Agents invent non-Bleed APIs | Plan agent must cite inventory + project patterns |
| Parallel cargo thrash | Concurrency cap 3; optional sequential test queue |
| Stale registry | `sync-catalog` + `record-merge` discipline |
| Foundation under-specified | F1/F2 get their own roadmaps before implement |

---

## 13. Decisions log

| Decision | Choice |
|----------|--------|
| Success definition | Orchestration system, not full catalog ship |
| Scheduling | Dependency + leverage |
| Git autonomy | Worktree + PR per suite; human merge |
| Suite DoD | Ship-ready vertical slice |
| Deliverable | Grok skill + playbook |
| Architecture | Catalog state + coordinator skill (Approach A) |
| Concurrency default | 3 parallel suite pipelines |
| Auto-merge | No |

---

## 14. Out of scope for the written implementation plan of F0

- Implementing F1/F2 application code  
- Opening real suite PRs for moderation  
- Auto-updating all ❌ markers in the 5800-line inventory  
- Discord bot runtime changes beyond what F0 needs (F0 should be docs/skill/registry only)

---

## 15. Section 4 content (definition of done, testing, recovery)

This section was validated as part of full design approval and is fully specified in **Sections 5–10** above:

- Vertical slice DoD → Section 5  
- Testing → Section 6  
- Recovery → Section 7  
- Layout → Section 8  
- Git/PR → Section 9  
- Success criteria → Section 10  

---

## 16. Next step after user approves this file

1. User reviews this spec and requests changes if needed.  
2. On approval, run the **writing-plans** skill to produce an implementation plan for **F0 only** (skill + registry + playbook).  
3. Execute that plan (separate session or same, per user).  
4. Then use `/orchestrate-commands` (or equivalent) to run Wave 1.
