Based on everything we have designed together, here is the complete high-level project description — something you can put directly in your `README.md` or use as a living reference document for contributors.

***

# `dirctl` — A Policy-Driven Directory Control Engine

> *"Not a file manager. A file operating system."*

`dirctl` is an open-source, cross-platform **directory control engine** built in Rust. It gives developers, power users, and system administrators a declarative, safe, and extensible way to define *how their directories behave* — not just how they look. You write rules that describe your intent. `dirctl` computes a plan, shows you exactly what will happen, and only acts when you say so. Every action is logged, every action is reversible.

Think of it as **Terraform for your filesystem** — but designed to evolve into an AI-native file operating system.

***

## What Problem It Solves

Every developer has a Downloads folder that becomes a graveyard. Every team has project directories that drift into chaos. Every system has logs, archives, and assets that pile up without policy. Existing tools offer two extremes: a GUI file manager that requires manual work, or a shell script that is brittle and irreversible.

`dirctl` sits in the middle: a programmable, safe, and auditable control layer that operates on **any directory** — not just the usual suspects like Downloads or Documents. It is general-purpose by design. You define the rules. It enforces them.

***

## Core Philosophy

These principles govern every design and implementation decision in the project:

- **Safety first, always.** No operation is irreversible. No file is hard-deleted. Every action is logged before it executes. The undo system is a first-class feature, not an afterthought.
- **Determinism over magic.** Identical rules applied to identical directory state must always produce an identical plan. No surprises, no randomness, no hidden state.
- **Plan before you act.** The user always sees a human-readable diff of what will happen before anything touches the disk. Automation is opt-in, never default.
- **The engine knows nothing about the UI.** The core library has zero knowledge of whether it is being called by a CLI, a desktop app, an AI agent, or a network daemon. This is the guarantee that makes the project infinitely extensible.
- **Local-first, privacy by default.** No telemetry, no cloud dependency. AI features use local models by default. Your files never leave your machine unless you explicitly configure it.
- **Open by design.** Config formats, journal formats, and plugin interfaces are documented public specifications. Third-party tools are first-class citizens.

***

## What We Are Building

`dirctl` is not a single tool. It is a **layered system** where each layer builds cleanly on the one below it.

### Layer 1: The Core Engine (`dirctl-core`)
The heart of the project — a pure Rust library with no I/O of its own. It speaks entirely in data structures. It receives a workspace configuration and a filesystem state, computes what should happen, and returns a plan. It never calls `std::fs` directly. It communicates with the outside world exclusively through well-defined traits (ports). This is what every other layer is built on top of.

The engine has five internal modules:
- **Scanner** — Walks directories, extracts metadata (name, extension, MIME type, size, content hash, dates), and builds an indexed snapshot of the workspace state
- **Rule Evaluator** — Parses `dirctl.yaml`, matches each file against its conditions, and resolves conflicts between overlapping rules
- **Planner** — Takes the rule evaluation results and produces a deterministic, ordered `Plan` — a list of operations with full conflict analysis
- **Executor** — Takes a `Plan` and a `FileSystemPort` implementation, writes each operation to the journal before executing it, and handles partial failures gracefully
- **Journal** — Manages the append-only `journal.jsonl` transaction log, enforces hash-guard safety on undo, and supports selective transaction reversal

### Layer 2: Infrastructure Adapters
Concrete implementations of the core engine's traits. The engine calls `self.fs.move_file(...)` — these adapters decide what that actually means:
- `dirctl-fs-local` — Wraps `std::fs` for local filesystem operations (ships with v0.1.0)
- `dirctl-fs-lan` — Wraps LAN transfer for cross-machine operations (planned Phase 4)
- `dirctl-fs-sftp` — Wraps SFTP for remote filesystem operations (planned Phase 4)

### Layer 3: User Interfaces (Adapters on top of core)
Ways humans and machines interact with the engine — each one is independently developed and never baked into the core:
- `dirctl-cli` — A `clap`-powered terminal CLI (Phase 1, ships first)
- `dirctl-tui` — A `ratatui`-powered interactive terminal UI with live plan diffing (Phase 2)
- `dirctl-desktop` — A Tauri-based cross-platform desktop app with a visual rule editor (Phase 3)

### Layer 4: AI Adapters
Modules that implement the `RuleGenerator` trait — they translate natural language into native `Rule` structs that the core engine already understands. The AI layer never executes file operations; it only generates proposals:
- `dirctl-ai-ollama` — Local model inference via Ollama (Phase 5)
- `dirctl-ai-openai` — Cloud fallback via OpenAI/Anthropic API (Phase 5, optional)

### Layer 5: Network & Sharing Layer
A LAN discovery and file transfer daemon that uses the same `FileSystemPort` and `Journal` abstractions as local operations. Peer-to-peer `dirctl` instances synchronize using the journal as a distributed log (Phase 4):
- `dirctl-lan-server` — mDNS-based peer discovery + custom or SFTP-based transfer protocol

***

## The Pipeline (How It Works)

Every operation in `dirctl` flows through the same five-stage pipeline, regardless of whether it was triggered by a CLI command, an AI suggestion, a watcher event, or a network sync:

```
1. SCAN      → Walk the workspace, build FileEntry index
2. EVALUATE  → Match each FileEntry against rules, record match reasons
3. PLAN      → Produce a deterministic, conflict-resolved Plan (no disk I/O)
4. PREVIEW   → Show the user a human-readable diff of all intended operations
5. APPLY     → Write-ahead to journal, execute operations, report results
              ↕
           UNDO  → Read journal backwards, verify hashes, reverse operations
```

The AI layer plugs in before step 2 (generating rules). The LAN layer plugs in at step 5 (replacing the local `FileSystemPort`). Everything else stays the same.

***

## The Configuration Model

Users describe their intent in `dirctl.yaml` — a human-readable, version-controlled config file that lives in the workspace:

```yaml
version: 1
workspace: "."
default_conflict_policy: rename_with_suffix

rules:
  - name: "archive-old-pdfs"
    conditions:
      - field: extension
        operator: eq
        value: "pdf"
      - field: age_days
        operator: gte
        value: 30
    actions:
      - type: move
        to: "./Archives/{year}/{month}/"
```

Rules are composable, ordered, and testable in isolation with `dirctl rules test <file>`. The config format is a **public spec**, documented so third-party editors and tools can read and write it.

***

## The Safety Model

`dirctl` treats safety as a hard constraint, not a feature toggle:

- **Write-ahead logging** — Every operation is journaled before it executes. A crash mid-apply leaves the journal in a recoverable state.
- **Hash-guard undo** — Before reversing any operation, the engine re-hashes the file and compares it to the journal record. If the file was externally modified, the undo is aborted with an explicit error rather than silently clobbering data.
- **Quarantine, not delete** — In v0.1.0, delete operations move files to `.dirctl/quarantine/<tx_id>/`. Hard deletes are a future opt-in.
- **Sandbox paths** — Destination paths containing `../` are rejected at rule evaluation time, preventing path traversal exploits.
- **Dry run by default** — `dirctl plan` never touches the disk. `dirctl apply` always prompts for confirmation unless `--yes` is explicitly passed.

***

## The Contribution Surface

The project is designed to make contributing easy and safe. The Cargo workspace structure means a contributor can add a new metadata extractor, a new action type, or a new AI adapter without ever touching the core engine. The public interfaces are:

| Extension Point | Trait to Implement | Example Contribution |
|---|---|---|
| New filesystem backend | `FileSystemPort` | Google Drive adapter |
| New AI rule generator | `RuleGenerator` | Gemini API adapter |
| New transport protocol | `TransportPort` | WebDAV transfer |
| New metadata extractor | `MetadataExtractor` | EXIF / audio tags |
| New action type | `Action` | Image resize on move |

Golden test fixtures (sample folder structures with expected plan outputs) ship with the repo so contributors can verify correctness without needing a specific OS or folder setup.

***

## Roadmap at a Glance

| Phase | What Ships | Status |
|---|---|---|
| **Phase 1** | Core engine + CLI (`scan`, `plan`, `apply`, `undo`, `watch`, `rules test`) | 🔨 Building now |
| **Phase 2** | Interactive TUI (plan diffing, rule editor, journal viewer) | Planned |
| **Phase 3** | Tauri desktop app (visual rule builder, activity dashboard) | Planned |
| **Phase 4** | LAN sharing daemon + SFTP adapter | Planned |
| **Phase 5** | Local AI rule generation (Ollama) + semantic search | Planned |
| **Phase 6** | Natural language chat interface — "the Arc browser of file management" | Vision |

***

## Technology Stack

| Concern | Choice | Reason |
|---|---|---|
| Core language | **Rust** | Memory safety for file ops, compile-time correctness, zero-cost abstractions |
| CLI framework | **clap** | Industry standard, great derive macros, shell completion generation |
| Directory traversal | **ignore** crate | Used by `ripgrep`, respects `.gitignore`, parallel walking |
| Config parsing | **serde + serde_yaml** | Zero-boilerplate deserialization into Rust structs |
| Async runtime | **tokio** | Required for watch mode and future network layer |
| TUI framework | **ratatui** | The standard for modern Rust TUIs |
| Desktop GUI | **Tauri 2.0** | Rust backend + web frontend, tiny binary, cross-platform |
| AI inference | **Ollama-compatible** | Local-first, model-agnostic, user brings their own weights |

***

This is the full picture of what `dirctl` is, what it is becoming, and exactly why every design decision was made. The foundation is strong enough to carry every feature on the roadmap without a rewrite — and open enough that the community can help build it.