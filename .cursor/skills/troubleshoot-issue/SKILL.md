---
name: troubleshoot-issue
description: >-
  Fills a blank Discord bot troubleshooting report from interactive repro intake
  plus description, optional screenshots, and reference files. Use when the user
  reports a bug, asks to troubleshoot, wants an issue/root-cause writeup, or
  mentions ISSUE-REPORT / Troubleshoot.
---

# Troubleshoot Issue

## Interactive intake (required first)

Before deep code search, collect missing repro context. Skip questions already answered clearly.

Ask in one batch when possible:

1. **Symptom?** What exactly is wrong (one sentence)?
2. **Command / flow?** Exact command, subcommand, or event path?
3. **Expected vs actual?** What should happen vs what happens?
4. **Repro steps?** Minimal steps to reproduce?
5. **Environment?** Prefix vs slash, guild vs DM, recent deploy/branch if known?
6. **Started when?** After a specific change, always, or intermittent?
7. **Evidence?** Screenshots, logs, error text, HTML/test output?
8. **Reference files?** Paths you already suspect?
9. **Desired output?** Report only, or report + implement fix afterward?

If repro is still unclear, ask follow-ups before concluding root cause. Do not invent symptoms.

## Instructions

1. Run interactive intake (above).
2. Inspect likely code paths using answers + reference files.
3. Fill the blank template with concrete findings (files, symbols, expected vs actual).
4. Prefer repo-relative paths.
5. Propose investigation steps and fixes; implement only if the user asked for a fix (intake Q9) or explicitly requests it later.

## Output template

```markdown
# Troubleshooting Report: <short title>

## Issue Summary
[1–3 sentences]

## Repro Context
- Command / flow:
- Environment:
- Steps:
- Started when:
- Evidence:

## Root Cause Analysis

### Problem Location
- File: `path`
- Symbol / area: ...

### Issue Details
[what is wrong]

### Visual / UX Impact (if applicable)
[from screenshots or repro]

### Code Analysis
[relevant snippets or call flow]

### Root Cause Possibilities
1. ...
2. ...

## Expected vs Actual Behavior

**Expected:**
...

**Actual:**
...

## Impact Assessment
- ...

## Solution Required

### Investigation Steps
1. ...

### Immediate Fixes
1. ...

### Code Changes Required
[outline only unless asked to implement]

## Verification Steps
1. ...

## Files to Investigate/Modify
- `path` — why

## Open questions / TBD
- ...
```
