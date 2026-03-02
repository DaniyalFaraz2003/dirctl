---
name: architecture-authority
description: >
  The single source of architectural truth for dirctl. Merges engineering
  principles, Clean Architecture, Domain-Driven Design, determinism rules,
  and safety invariants. Use for all design, refactor, planning, and
  structural review tasks.
---

# dirctl — Architecture Authority Skill

This skill is the architectural constitution of dirctl.

It governs:

- Crate boundaries
- Clean Architecture enforcement
- Domain-Driven Design
- Deterministic planning guarantees
- Safety and journal invariants
- AI-driven refactor discipline

If a request violates any invariant defined here, the skill must:
- Refuse blind modification
- Explain the violation
- Suggest a compliant alternative

This file overrides all other architecture-related skills.

---

# 1. Canonical Crate Structure (Non-Negotiable)

dirctl must follow strict Clean Architecture boundaries:

dirctl-core
    Pure domain logic
    No IO
    No CLI
    No concrete infrastructure

dirctl-fs-*
    Filesystem adapters
    Implements ports defined in core

dirctl-cli
    Argument parsing
    User interaction
    Calls use cases from core

---

## Hard Boundary Rules

dirctl-core must NEVER depend on:

- std::fs
- tokio::fs
- clap
- logging implementations
- concrete filesystem adapters
- environment variables
- system time directly

Core may depend ONLY on:

- Traits (ports)
- Domain entities
- Value objects
- Pure deterministic logic
- Explicitly injected abstractions

Infrastructure implements ports defined in core.
Direction of dependency must always point inward.

If this boundary is violated, the skill must block the change.

---

# 2. Domain-Driven Design (dirctl-Specific)

## Ubiquitous Language (Mandatory)

Use only domain-specific terminology:

Workspace
Rule
Condition
Evaluator
Scanner
Planner
Plan
Operation
Executor
Journal
Transaction
Quarantine
Conflict Policy

Never introduce generic modules:

- utils
- helpers
- common
- shared
- manager
- misc

Use domain-specific names:

RuleEvaluator
ConflictResolver
PlanSerializer
JournalWriter
OperationHasher
WorkspaceScanner

Each module must represent a clear domain responsibility.

---

# 3. Determinism Contract (Critical)

dirctl is a deterministic planner.

Identical inputs must always produce identical Plan outputs.

## Required Ordering Key

Plan ordering must follow:

(rule_priority, file_path_depth, lexicographic_path)

All iteration affecting semantics must be explicitly sorted.

Forbidden:

- HashMap iteration influencing behavior
- OS filesystem enumeration order reliance
- Randomness
- System time in core
- Implicit ordering assumptions

If time is needed, it must be injected via trait abstraction.

Any proposed change must explicitly state determinism impact.

---

# 4. Safety Model (Non-Optional)

dirctl is safety-first.

All architecture must preserve:

1. Journal write BEFORE execution
2. Journal entries include:
   - tx_id (uuid)
   - timestamp (injected)
   - operation list
   - content hashes
3. Undo verifies current file hash matches journal hash
4. Delete operations move to:
   .dirctl/quarantine/<tx_id>/
5. Hard delete only when explicitly enabled
6. Destination templates must reject ".."

If a design weakens safety, the skill must reject it.

All proposed changes must explicitly mention safety impact.

---

# 5. Use Case Isolation

All business flows must follow:

CLI → UseCase → Core Domain → Port → Infrastructure

Never:

CLI → Infrastructure → Domain

Use cases must be explicit types:

PlanWorkspace
ExecutePlan
UndoTransaction
ValidateRules
PreviewChanges

Controllers must not contain business logic.

---

# 6. Rust Code Standards (Core Crates)

## Strict Rules

- No unwrap() in core
- No expect() in core
- No panic! in library code
- No global mutable state
- No hidden side effects
- Avoid Arc<Mutex<>> unless unavoidable
- No unsafe unless explicitly justified

## Structure Constraints

- Functions ideally ≤ 50 lines
- Files ideally ≤ 200 lines
- Large modules must be decomposed
- Early returns preferred over deep nesting
- Max nesting depth: 3 levels

---

# 7. Library-First Philosophy (Rust Context)

Before writing custom code:

- Check crates.io
- Prefer established crates for:
  - error handling (thiserror)
  - serialization (serde)
  - UUID generation (uuid)
  - deterministic maps (indexmap)

Custom code is justified only when:

- It is core business logic
- It enforces safety invariants
- It preserves determinism
- Existing crate introduces nondeterminism or excessive risk

Avoid NIH syndrome.

---

# 8. Refactor Discipline (AI Governance)

When this skill is active and changes are requested:

Claude must:

1. Provide a 3–5 step architectural plan first.
2. List exact file paths impacted.
3. Confirm crate boundary compliance.
4. Explicitly mention determinism impact.
5. Explicitly mention safety impact.
6. Provide unified diff only.
7. Suggest required tests.
8. Provide rollback strategy.

No automatic edits without explicit confirmation.

---

# 9. Architecture Audit Mode

If asked to audit:

Return structured report:

{
  "boundary_violations": [],
  "determinism_risks": [],
  "safety_violations": [],
  "naming_violations": [],
  "recommended_refactors": []
}

Do not produce diffs in audit mode.

---

# 10. Absolute Rejection Conditions

The skill must block changes that:

- Introduce nondeterministic behavior
- Bypass journal writes
- Perform filesystem IO in core
- Add unwrap() in core
- Collapse crate boundaries
- Introduce generic dumping-ground modules
- Allow path traversal via templates

---

# 11. Invocation Examples

"Using architecture-authority, refactor planner conflict resolution."

"Architecture-authority audit the current crate graph."

"Using architecture-authority, design a new rule type for checksum validation."

---

# Final Rule

Architecture Authority is the final decision-maker for structural changes.

If conflict exists between:
- convenience
- speed
- architectural integrity

Architectural integrity wins.

This is the constitution of dirctl.