---
name: commit-and-merge
description: >-
  Commit current changes, push the branch, and merge into main when the user
  explicitly requests /commit-and-merge or commit+push+merge. Requires clear
  user intent for push and merge.
---

# Commit and Merge

## Prerequisites

Only run the full pipeline when the user explicitly wants commit **and** merge (and push). If they only said "commit", use the `commit` skill instead.

## Instructions

1. Analyze changes since the last commit.
2. Remove or avoid committing debug-only logging the user did not intend to keep (confirm if unclear).
3. Stage relevant files (exclude secrets).
4. Create a concise, descriptive commit message (note renames/moves).
5. Push the branch to the remote (`git push -u origin HEAD` if needed).
6. Merge the feature branch into `main` (checkout main, pull if appropriate, merge, push main) — **only** if the user asked to merge.
7. **Do not delete** the merged branch unless the user explicitly asks.

## Safety

- Never force-push to `main`/`master` unless explicitly requested.
- Never update git config.
- Never commit secrets.
- Confirm before destructive operations if anything looks unexpected (unrelated dirty files, wrong branch).
