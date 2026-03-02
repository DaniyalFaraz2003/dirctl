---
name: test-generator
description: >-
  Generate deterministic golden fixtures and unit/integration tests for planner,
  scanner, and executor behavior. Trigger when adding or changing core logic.
---

# dirctl — Test Generator (Skill)

## Purpose
Create `tests/fixtures/<name>/` snapshots and corresponding Rust tests that
load the snapshot, run the pipeline, and assert that the produced Plan JSON
matches the golden output.

## Output requirements
- Produce a fixture directory listing (relative paths + file contents).
- Produce an integration test under `crates/dirctl-core/tests/` that:
  - Loads the fixture snapshot
  - Invokes Scanner → Evaluator → Planner
  - Serializes Plan to JSON and compares to `expected_plan.json`

## Constraints
- Use deterministic timestamps injected via test helpers (do not use system time).
- Tests must compile and pass locally.

## Example invocation
- "Create a golden fixture for overlapping rename rules named `rename-conflict-1`."

## Verification commands
- `cargo test --package dirctl-core --test fixtures`