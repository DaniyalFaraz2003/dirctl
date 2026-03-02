---
name: commit-assistant
description: >-
  Produce Conventional Commit messages, PR bodies, changelog notes and migration
  steps from a diff. Attach AI-transcript metadata when AI-assisted. Trigger on
  requests to summarize staged changes or draft PR descriptions.
---

# dirctl — Commit Assistant (Skill)

## Purpose
Given a staged diff or list of changed files, produce:
1) A Conventional Commit subject line with crate scope.
2) A 2–4 paragraph PR description explaining what changed and why.
3) A short changelog entry and migration notes (if applicable).
4) If changes were produced by AI, include `AI-ASSISTED: true` and transcript hash.

## Output format
Return Markdown with sections:
- `Commit-Subject:`
- `PR-Description:`
- `Changelog:`
- `Migration:`
- `AI-Metadata:` (optional)

## Example invocation
- "Commit Assistant: summarize staged diff for crates/dirctl-core"

## Verification commands
- `git add -N . && git diff --staged --no-color | sed -n '1,200p'`