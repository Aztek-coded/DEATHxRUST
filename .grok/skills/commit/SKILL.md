---
name: commit
description: >-
  Create a git commit for current changes with a concise summary. Use when the
  user asks to commit, /commit, or save work to git. Does not push unless the
  user explicitly asks to push.
---

# Commit

## Instructions

1. Analyze changes since the last commit (`git status`, `git diff`, recent log style).
2. Stage the relevant changed files (prefer what the user intends; default to all intentional project changes, exclude secrets like `.env`).
3. Draft a concise but descriptive commit message summarizing **why** / what changed.
   - Mention filename renames or moves when relevant.
4. Create the commit (prefer a HEREDOC for the message body).
5. **Do not push** unless the user explicitly asks to push in the same request.
6. Do not amend published commits unless the user explicitly requests amend and conditions are safe.

## Safety

- Never update git config.
- Never commit secrets (`.env`, tokens, credentials).
- Follow repo AGENTS.md: no commit unless the user asked (this skill is that ask).
