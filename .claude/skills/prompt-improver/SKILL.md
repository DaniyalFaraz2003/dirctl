---
name: prompt-improver
description: >-
  Converts short, vague, or underspecified user prompts into precise,
  constraint-aware, execution-ready instructions for Claude Code.
  Trigger when a prompt is brief, ambiguous, high-level, or missing constraints.
---

# Prompt Improver (Skill)

## Purpose

Transform minimal user prompts into:
- Fully scoped engineering instructions
- Explicit constraints
- Deterministic expectations
- Clear file targets
- Required validation steps
- Expected output format

This skill reduces the need for long manual prompts while preserving precision.

---

## When To Trigger

Activate automatically when:
- Prompt length < 3 sentences
- User says "improve this prompt"
- Prompt contains vague verbs like:
  - fix
  - improve
  - refactor
  - optimize
  - clean up
  - make better
  - design this
- Prompt lacks:
  - file paths
  - constraints
  - success criteria
  - safety requirements
  - output format

---

## What This Skill Does

Given a short user prompt:

1. Infer likely intent.
2. Infer missing constraints.
3. Add engineering safety constraints.
4. Add determinism constraints.
5. Add file scope.
6. Add output formatting requirements.
7. Add verification steps.
8. Convert into an execution-ready Claude instruction.

---

## Output Format (MANDATORY)

Return output in this exact structure:

### Improved Instruction

<fully rewritten precise instruction>

### Assumptions Made

- assumption 1
- assumption 2

### Clarifications Needed (only if critical)

- question 1
- question 2

### Validation Commands

```bash
<commands user should run>