---
name: rust-standards
description: >-
  Enforce dirctl Rust coding standards: no unwrap() in core, deterministic
  iteration, error types, docs, formatting and clippy expectations. Trigger on
  code generation, refactors, and code review tasks.
---

# dirctl — Rust Standards (Skill)

## Purpose
Provide explicit Rust best practices and automated guidance when generating or
refactoring Rust code for dirctl. Ensure generated code is idiomatic and safe.

## Key rules (to enforce)
- No `unwrap()` or `expect()` in core crates; allowed only in tests.
- Use `thiserror` for crate-local error types; map external errors at boundaries.
- No `panic!` in library code.
- Explicit ordering for any iteration affecting semantics (sort keys).
- `rustfmt` and `clippy` compliance required.

## What to produce on request
- If asked to change code, return diffs that follow the rules above.
- If tests fail, explain failures and propose fixes.
- Provide minimal, focused changes (do not refactor unrelated code).

## Example invocation
- "Generate a deterministic iteration snippet for Planner following rust-standards."

## Verification commands
- `rustfmt --check $(git ls-files '*.rs' | tr '\n' ' ')`
- `cargo clippy --all-targets --all-features -- -D warnings`