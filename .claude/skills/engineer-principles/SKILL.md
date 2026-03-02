---
name: engineer-principles
description: >-
  Authoritative engineering constraints for dirctl: safety, determinism, trait
  boundaries, and AI-driven edit governance. Trigger on architecture planning,
  policy changes, and any multi-file automated refactor.
---

# dirctl — Engineering Principles (Skill)

## Purpose
Provide the canonical constraints and non-negotiable invariants for any
architecture, plan, or code edit proposed by Claude. Always anchor reasoning to
this file when asked to design, change, or audit core behaviors.

## When to trigger
- Prompts that request architecture changes, module moves, trait/API changes,
  safety model modification, or automated refactors across crates.
- When a user begins a session with "Plan", "Refactor", "Design", "Audit".

## Instructions (what Claude should do)
1. Read and enforce the following invariants:
   - Core library (`dirctl-core`) must not perform filesystem I/O.
   - All filesystem operations must be abstracted behind `FileSystemPort`.
   - Planner output must be deterministic for identical inputs.
   - No hard deletes by default; use quarantine and undo journal.
2. For any proposed change:
   - Produce a 3–5 step plan before any edits.
   - List exact file paths to change.
   - Provide a rollback strategy and tests required.
3. If the user asks to apply changes, require explicit human confirmation.

## Example invocations
- "Architect Mode: propose a migration to move planner conflict logic to a new crate. Use engineering-principles."
- "Before refactor, follow engineering-principles and list tests required."

## Verification commands (run locally)
- `cargo test --workspace`
- `rg "std::fs::|tokio::fs::" crates/dirctl-core -n || true`  # should show no results