---
name: merge-suite-pr
description: >
  Finish a DEATHxRUST feature suite after the user confirms it is ready to
  merge: push remaining commits if needed, merge the open PR into main, run
  suite registry record-merge, commit/push registry updates, delete the
  remote and local feature branch, and clean worktrees when safe. Use when
  the user runs /merge-suite-pr, says a suite or PR is ready to merge, asks
  to ship/finish/land a suite branch, or wants merge + record-merge + branch
  delete automated in one step.
---

# Merge suite PR

Automate the **post-approval** land path for DEATHxRUST suite (or feature) work.

**Default path:** push → merge PR to `main` → `record-merge` (if suite) → push registry → delete branches → optional worktree cleanup.

## When this skill is permission

Invoking this skill **with a clear target** (suite id, PR number, or current feature branch) **is** explicit permission for this run to:

1. Push the feature branch (if behind/ahead needs publishing)
2. Merge the PR into `main` via `gh`
3. Checkout `main`, pull, and update the suite registry
4. Commit and push registry-only follow-up on `main` when `record-merge --write` changes files
5. Delete remote and local feature branches
6. Remove a project-owned suite worktree under `.worktrees/` when present

Do **not** use this skill to invent a merge without user intent. If the user only asked “what’s next” or “status”, do not merge.

If the message is ambiguous (multiple open PRs, wrong branch), resolve the target first, then confirm once before merging.

## Inputs (resolve in order)

Parse the user message for any of:

| Input | Example |
|-------|---------|
| Suite id | `moderation-foundation`, `merge-suite-pr boosts` |
| PR number | `#1`, `pr 1` |
| Branch | `feature/moderation-foundation` |
| Implicit | Current branch is `feature/<suite-id>` and an open PR exists |

If none resolve cleanly:

1. List open PRs: `gh pr list --state open --limit 10`
2. Ask which PR/suite to land (one short question)

### Suite id discovery

1. If user gave a suite id, use it.
2. Else if branch is `feature/<id>`, use `<id>`.
3. Else if registry `pr:` URL matches the open PR, use that suite’s `id`.
4. Else treat as **non-suite** PR: still merge and delete branch; **skip** `record-merge`.

Registry path:

`.grok/orchestration/suite-registry.yaml`

## Preconditions (fail closed)

Run these before merge. Stop and report if any hard check fails.

```bash
# Identity
git status -sb
git branch --show-current
gh pr view <PR> --json number,url,state,mergeable,mergeStateStatus,baseRefName,headRefName,title,statusCheckRollup
```

| Check | Action |
|-------|--------|
| PR `state` is `OPEN` | Else stop |
| `baseRefName` is `main` (unless user named another base) | Else stop and ask |
| `mergeable` is `MERGEABLE` (or equivalent) | If conflicts, stop; tell user to rebase |
| `mergeStateStatus` not blocked by required checks (when GitHub reports failure) | Stop unless user said “merge anyway” / “admin merge” in this run |
| Working tree on feature branch: no unrelated secret files staged | Never commit `.env` or tokens |
| Unpushed commits on feature branch | Push first (`git push -u origin HEAD`) before merge |
| Dirty worktree with **uncommitted** work | Stop and ask: commit, stash, or discard — do not merge over lost work |

Soft warnings (continue after short note):

- Optional CI still running but mergeable and user confirmed ready
- Pre-existing failing tests unrelated to the suite (note in summary)

## Procedure

### 1. Align remote feature branch

```bash
git checkout feature/<suite-id>   # or the PR head branch
git status -sb
# If ahead of origin:
git push -u origin HEAD
```

If push is rejected, stop and report (do not force-push unless the user explicitly allows force-push **in this run**).

### 2. Merge the PR

Prefer GitHub merge so history and PR state stay correct:

```bash
# Default: create a merge commit (change only if user asked squash/rebase)
gh pr merge <PR> --merge --delete-branch
```

Notes:

- `--delete-branch` deletes the **remote** head branch when the host allows it.
- If the user asked for **squash**: `gh pr merge <PR> --squash --delete-branch`
- If the user asked for **rebase merge**: `gh pr merge <PR> --rebase --delete-branch`
- If `--delete-branch` fails, continue and delete remote manually in step 5.

If `gh pr merge` fails, stop. Do not run `record-merge`. Do not delete local branches.

### 3. Update local `main`

```bash
git fetch origin
git checkout main
git pull --ff-only origin main
```

If fast-forward fails, stop and report (do not hard reset unless the user asks).

### 4. Suite registry `record-merge` (suite only)

When a suite id is known and exists in the registry:

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb record-merge <suite-id> --write
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb validate
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb status
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb next 5
```

If `record-merge` changes the registry (or related files):

```bash
git add .grok/orchestration/suite-registry.yaml
# include only registry-related paths that the tool touched
git commit -m "$(cat <<'EOF'
chore: record-merge <suite-id> after PR land

Mark suite done in the orchestration registry and unlock dependents.
EOF
)"
git push origin main
```

If there is nothing to commit, skip the commit.

If the suite id is missing from the registry, report that and skip this step (merge still succeeded).

### 5. Delete local feature branch and prune

Remote may already be gone from `--delete-branch`.

```bash
git branch -d feature/<suite-id> 2>/dev/null || git branch -D feature/<suite-id>
git remote prune origin
# If remote branch still exists:
git push origin --delete feature/<suite-id> 2>/dev/null || true
```

Prefer `-d` (safe delete). Use `-D` only if the branch is fully merged per `git` but `-d` still refuses, and the PR merge succeeded.

### 6. Worktree cleanup (safe only)

If a worktree exists for this suite under `.worktrees/<suite-id>` or a path clearly owned by this project’s orchestration:

```bash
# From main repo root only
git worktree list
git worktree remove <path>   # only if path is suite-specific and unused
git worktree prune
```

Do **not** remove unknown worktrees or the primary workspace checkout.

### 7. Final report (always)

Tell the user in short form:

| Item | Value |
|------|--------|
| PR | number + URL + merged |
| Suite | id or “non-suite” |
| `record-merge` | done / skipped / failed |
| Dependents now ready | from `next` (if suite) |
| Branches deleted | remote + local |
| Current branch | should be `main` |

## Failure recovery

| Failure | Do |
|---------|----|
| Push failed | Stop before merge |
| Merge failed | Stop; leave branches; no `record-merge` |
| `record-merge` failed after merge | Stay on `main`; report; leave feature branch deletion optional; user can re-run record-merge |
| Registry commit push failed | Report; main has merge but registry may be local-only |
| Branch delete failed | Report remaining branch names; merge is still success |

## Relation to other skills

| Skill | Role |
|-------|------|
| `/orchestrate-commands` | Plan → implement → test → review → open PR. Does **not** merge. |
| `/orchestrate-commands record-merge <id>` | Registry-only after a merge already happened |
| `/merge-suite-pr` | **This skill** — full land path after user confirms ready |

Do not open a new PR here. Do not implement suite code here.

## STE100

User-facing summaries (PR notes, status lines in chat when they are formal docs) use short clear sentences. Commit messages stay conventional (`chore:`, `feat:`).

## Examples

User: `/merge-suite-pr moderation-foundation`  
→ Resolve PR for `feature/moderation-foundation`, push if needed, merge, record-merge, cleanup.

User: `PR 1 is good, merge it`  
→ Target PR #1, derive suite from branch/registry, run full procedure.

User: `/merge-suite-pr` while on `feature/boosts`  
→ Use current branch + open PR; if suite `boosts` exists, record-merge after land.
