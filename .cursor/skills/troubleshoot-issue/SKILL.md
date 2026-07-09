---
name: troubleshoot-issue
description: >-
  Fills a blank Discord bot troubleshooting report from a brief description,
  optional screenshots, and reference files. Use when the user reports a bug,
  asks to troubleshoot, wants an issue/root-cause writeup, or mentions
  ISSUE-REPORT / Troubleshoot.
---

# Troubleshoot Issue

## Instructions

1. Read the brief description, screenshots, and any reference files.
2. Inspect likely code paths in the repo before concluding root cause.
3. Fill the blank template below with concrete findings (files, symbols, expected vs actual).
4. Prefer repo-relative paths.
5. Propose investigation steps and fixes; implement only if the user asks.

## Input (from user)

- **Description:** brief symptom statement
- **Screenshots:** optional
- **Reference files:** optional paths

## Output template

```markdown
# Troubleshooting Report: <short title>

## Issue Summary
[1–3 sentences]

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
```
