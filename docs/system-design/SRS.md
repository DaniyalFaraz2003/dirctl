
## Project Overview

**Name:** `dirctl` (working title)
**Type:** Cross-platform CLI tool + reusable Rust library engine
**Purpose:** A policy-driven, safety-first directory control tool — not a file manager replacement, but a general-purpose engine that lets users define rules, preview their effects, and apply/undo them deterministically. [perforce](https://www.perforce.com/blog/alm/how-write-software-requirements-specification-srs-document)

***

## Functional Requirements (FR)

These define *what the system must do*. Each requirement is written as a testable statement. [jamasoftware](https://www.jamasoftware.com/requirements-management-guide/writing-requirements/functional-vs-non-functional-requirements/)

### FR-1: Workspace Initialization
- `FR-1.1` — The system **must** create a `.dirctl/` hidden directory in the target workspace upon `init`
- `FR-1.2` — The system **must** generate a default `dirctl.yaml` config with commented examples on `init`
- `FR-1.3` — The system **must** refuse to re-initialize an already initialized workspace without a `--force` flag

### FR-2: Directory Scanning & Indexing
- `FR-2.1` — The system **must** recursively scan any given directory path (not limited to standard OS folders)
- `FR-2.2` — The system **must** extract metadata for every file: name, extension, MIME type, size (bytes), created date, modified date, and content hash (SHA-256)
- `FR-2.3` — The system **must** support `.dirctlignore` files (gitignore syntax) to exclude paths from scanning
- `FR-2.4` — The system **must** complete a scan of 100,000 files in under 5 seconds on standard hardware

### FR-3: Rule Engine
- `FR-3.1` — The system **must** load rules from `dirctl.yaml` in the workspace
- `FR-3.2` — Each rule **must** support at least these conditions: `extension`, `min_size`, `max_size`, `min_age_days`, `max_age_days`, `name_regex`, `mime_type`, `not_path`
- `FR-3.3` — Each rule **must** support at least these actions: `move`, `rename`, `copy`, `delete`, `tag`
- `FR-3.4` — Rules **must** be evaluated top-to-bottom; first match wins (unless `match_all: true`)
- `FR-3.5` — The system **must** support a `dirctl rules test <file>` command that shows every rule's match/no-match result for a given file with a reason

### FR-4: Planning
- `FR-4.1` — The system **must** compute a full plan (list of intended operations) without touching the disk
- `FR-4.2` — The plan output **must** show: source path → destination path, which rule triggered it, and conflict status
- `FR-4.3` — The system **must** detect and surface all conflicts before apply: name collisions, moves to non-existent directories, and circular rules
- `FR-4.4` — The plan **must** be deterministic: identical state + identical rules = identical plan, always
- `FR-4.5` — The system **must** provide functionality to view the plan, just like in a `--dry-run` mode
- `FR-4.6` — The system **must** provide a diff for the plan, which shows the state of the directory before and after the plan is applied

### FR-5: Applying Operations
- `FR-5.1` — The system **must** only execute a plan after explicit user confirmation (`--yes` flag or interactive prompt)
- `FR-5.2` — On a conflict, the system **must** follow the configured `conflict_policy`: `skip`, `overwrite`, or `rename_with_suffix`
- `FR-5.3` — The system **must** write every executed operation to the journal **before** executing it (write-ahead logging)
- `FR-5.4` — If an `apply` is interrupted mid-run, the system **must** leave the journal in a state from which `undo` can recover all completed operations

### FR-6: Undo / Journal
- `FR-6.1` — The system **must** maintain an append-only `journal.jsonl` file inside `.dirctl/`
- `FR-6.2` — Each journal entry **must** contain: timestamp, transaction ID, operation type, source path, destination path, and file hash
- `FR-6.3` — `dirctl undo` **must** reverse the last transaction completely
- `FR-6.4` — `dirctl undo --tx <id>` **must** allow reverting a specific past transaction
- `FR-6.5` — The system **must** refuse to undo a transaction if a file's current hash does not match the journal's expected hash (file was externally modified)

### FR-7: Watch Mode
- `FR-7.1` — The system **must** support a `watch` command that monitors a workspace for filesystem events
- `FR-7.2` — On a new file event, the system **must** compute a plan for the new file and log it — but **must not** auto-apply unless `--auto` is explicitly passed
- `FR-7.3` — The watch daemon **must** write activity to a human-readable log at `.dirctl/watch.log`

***

## Non-Functional Requirements (NFR)

These define *how well the system performs its functions*. [altexsoft](https://www.altexsoft.com/blog/non-functional-requirements/)

| ID | Category | Requirement |
|---|---|---|
| NFR-1 | **Safety** | No file operation is irreversible without a journal entry. Delete operations must move to a quarantine folder, never hard-delete, in v0.1.0 |
| NFR-2 | **Performance** | Plan generation for 10,000 files must complete in under 2 seconds |
| NFR-3 | **Portability** | Must compile and run correctly on Windows 10+, macOS 12+, and Linux (Ubuntu 20.04+) without OS-specific code paths in the engine |
| NFR-4 | **Reliability** | An interrupted `apply` must never leave the filesystem in an irrecoverable state |
| NFR-5 | **Usability** | All errors must output a human-readable message, a machine-readable error code, and a suggested fix |
| NFR-6 | **Extensibility** | The core engine (`dirctl-core`) must be a separate Rust library crate with no CLI dependencies, so it can be embedded in a future desktop app |
| NFR-7 | **Maintainability** | Every public function in `dirctl-core` must have a doc comment and unit test coverage ≥ 80% |
| NFR-8 | **Security** | Path traversal attacks must be rejected at the rule evaluation layer (e.g., `../` in destination paths) |
| NFR-9 | **Openness** | Config file formats (`dirctl.yaml`, `journal.jsonl`) must be documented as a public spec so third-party tools can integrate |

***

## System Architecture

The project is structured as a **Cargo workspace** with strict layer separation: [reddit](https://www.reddit.com/r/rust/comments/1lhy0bx/visualizing_architectural_layers_in_rust_projects/)

```
dirctl/                         ← Cargo workspace root
├── crates/
│   ├── dirctl-core/            ← Pure engine library (no CLI/UI)
│   │   ├── scanner/            ← Directory walker + metadata extractor
│   │   ├── rules/              ← Rule parser + evaluator
│   │   ├── planner/            ← Plan computation + conflict resolution
│   │   ├── executor/           ← Applies plans, writes journal
│   │   └── journal/            ← Read/write/undo journal entries
│   ├── dirctl-cli/             ← clap-based CLI, thin wrapper over core
│   └── dirctl-tui/             ← (Future) Ratatui TUI adapter
├── tests/
│   └── fixtures/               ← Golden folder structures for integration tests
└── dirctl.yaml                 ← Example workspace config
```

The **dependency rule is strict and one-directional**: `dirctl-cli` depends on `dirctl-core`. `dirctl-core` has zero knowledge of any UI or CLI. This is the guarantee that makes your Tauri desktop app possible later — it just becomes another adapter on top of the same core.

***

## Core Data Models

These are the primary structs the engine will work with:

```
FileEntry         → path, name, extension, mime, size, hash, created_at, modified_at, tags[]
Rule              → name, conditions[], actions[], conflict_policy, match_all
Condition         → field (enum), operator (enum), value
Action            → type (enum: Move|Rename|Copy|Delete|Tag), parameters
Plan              → id, created_at, workspace, operations[]
Operation         → id, rule_name, file_entry, action, status, conflict_resolution
JournalEntry      → tx_id, timestamp, operation, pre_state(path+hash), post_state(path+hash)
WorkspaceConfig   → version, workspace_root, rules[], default_conflict_policy, ignore_patterns[]
```

***

## Out of Scope for v0.1.0

Being explicit about what you are **not** building yet keeps the MVP clean: [tkxel](https://tkxel.com/blog/7-tips-to-identify-software-requirements-for-a-successful-development-cycle/)

- No AI/LLM classification
- No GUI or TUI
- No cloud sync or remote filesystem support
- No plugin system
- No scheduled/cron-based automation
- No duplicate detection (Phase 2 feature)
- No content-based metadata extraction (PDF, EXIF, audio tags)

***
