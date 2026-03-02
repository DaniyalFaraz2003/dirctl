This is a purely design/specification task — no web search needed. Let me produce both specs in full detail.

***

## High Level System Design (For future enhancements)

```txt
┌──────────────────────────────────────────────────┐
│                  THE OUTSIDE WORLD               │
│                                                  │
│   [CLI]   [TUI]   [Desktop GUI]   [AI Agent]     │
│   [LAN Server]    [REST API]      [Plugin]       │
│                                                  │
│         ↕ calls via defined Ports ↕              │
├──────────────────────────────────────────────────┤
│              dirctl-core (The Engine)            │
│                                                  │
│   Scanner → Rules → Planner → Executor → Journal │
│                                                  │
│         ↕ calls via defined Ports ↕              │
├──────────────────────────────────────────────────┤
│               INFRASTRUCTURE LAYER               │
│                                                  │
│  [Local FS]  [SQLite]  [Remote FS]  [AI Model]   │
└──────────────────────────────────────────────────┘
```

Every future feature — AI, LAN sharing, file transfer protocols — slots in as an adapter at the top or bottom layer. The core engine never changes. This is exactly how Rust's tokio ecosystem and tools like Spacedrive are structured.

## 1. Config File Format Specification (`dirctl.yaml`)

### Top-Level Schema

```yaml
# dirctl.yaml — Workspace Configuration
# Schema Version: 1

version: 1                          # (required) Schema version for forward compatibility
workspace: "."                      # (required) Root path this config controls
default_conflict_policy: rename_with_suffix  # skip | overwrite | rename_with_suffix
ignore_patterns:                    # gitignore-style patterns, always excluded from scanning
  - ".git/"
  - "node_modules/"
  - ".dirctl/"
  - "*.tmp"

rules: []                           # Ordered list of rules (top-to-bottom evaluation)
```

***

### Rule Schema

Every rule follows this exact structure:

```yaml
rules:
  - name: "string"             # (required) Unique, kebab-case identifier
    description: "string"      # (optional) Human-readable explanation
    enabled: true              # (optional) Default: true. Set false to disable without deleting
    priority: 0                # (optional) Higher number = evaluated first. Default: 0
    match_all: false           # (optional) false = first-match-wins | true = apply to all matches
    conflict_policy: skip      # (optional) Overrides default_conflict_policy for this rule only
    conditions:                # (required) List of conditions, ALL must match (AND logic)
      - field: extension
        operator: eq
        value: "pdf"
    actions:                   # (required) Executed in order, top to bottom
      - type: move
        to: "./Archives/"
```

***

### Condition Fields & Operators

This is the complete matrix of every valid `field` + `operator` + `value` combination:

| Field | Type | Valid Operators | Value Type | Example |
|---|---|---|---|---|
| `name` | string | `eq` `neq` `contains` `not_contains` `starts_with` `ends_with` `matches` `not_matches` | string (regex for `matches`) | `value: "report"` |
| `extension` | string | `eq` `neq` `in` `not_in` | string or list | `value: ["pdf", "docx"]` |
| `mime_type` | string | `eq` `neq` `starts_with` `in` `not_in` | string or list | `value: "video/"` |
| `size` | bytes (int) | `eq` `gt` `lt` `gte` `lte` `between` | int or `[min, max]` for `between` | `value: 524288000` |
| `age_days` | int | `gt` `lt` `gte` `lte` `between` | int or `[min, max]` | `value: 30` |
| `path` | string | `contains` `not_contains` `starts_with` `ends_with` `matches` `not_matches` | string | `value: "Projects/"` |
| `tag` | string | `has` `not_has` | string | `value: "reviewed"` |
| `modified_at` | ISO date | `before` `after` `between` | `"YYYY-MM-DD"` or `["date1","date2"]` | `value: "2025-01-01"` |
| `created_at` | ISO date | `before` `after` `between` | `"YYYY-MM-DD"` or `["date1","date2"]` | `value: "2025-01-01"` |

**Notes:**
- Multiple conditions within a rule use **AND** logic by default
- To express OR logic, write separate rules with the same actions and `match_all: false`
- `matches` / `not_matches` uses Rust-compatible regex syntax (RE2 semantics)
- `between` for `size` accepts `[min_bytes, max_bytes]`

***

### Action Types

**`move`** — Move file to a destination directory (path template supported)
```yaml
- type: move
  to: "./Archives/{year}/{month}/"   # destination directory (created if not exists)
  create_dirs: true                  # (optional) Default: true
```

**`rename`** — Rename the file in-place using a template pattern
```yaml
- type: rename
  pattern: "{year}-{month}-{day}_{stem}_{hash_short}.{ext}"
```

**`copy`** — Copy file to destination, leave original intact
```yaml
- type: copy
  to: "./Backups/{year}/"
  create_dirs: true
```

**`delete`** — Remove the file (NEVER hard-deletes in v0.1.0)
```yaml
- type: delete
  strategy: quarantine   # quarantine (default) | soft
  # quarantine = moves to .dirctl/quarantine/<tx_id>/
  # soft = marks in journal only, file untouched until confirmed
```

**`tag`** — Attach metadata tags (stored in `.dirctl/tags.db`, never alters filename)
```yaml
- type: tag
  tags: ["archived", "large", "reviewed"]
  remove_tags: ["inbox"]             # (optional) Tags to strip simultaneously
```

***

### Rename & Move Template Tokens

These tokens are valid inside `rename.pattern` and `move.to` values:

| Token | Description | Example Output |
|---|---|---|
| `{name}` | Full filename with extension | `report_Q1.pdf` |
| `{stem}` | Filename without extension | `report_Q1` |
| `{ext}` | Extension without leading dot | `pdf` |
| `{year}` | 4-digit year (from `modified_at`) | `2026` |
| `{month}` | 2-digit month | `02` |
| `{day}` | 2-digit day | `22` |
| `{hour}` | 2-digit hour (24h) | `14` |
| `{minute}` | 2-digit minute | `05` |
| `{second}` | 2-digit second | `33` |
| `{date_modified}` | Shorthand for `{year}-{month}-{day}` | `2026-02-22` |
| `{date_created}` | Created date as `YYYY-MM-DD` | `2025-11-10` |
| `{hash_short}` | First 8 chars of SHA-256 | `a3f9c12d` |
| `{hash_full}` | Full 64-char SHA-256 | `a3f9c12d...` |
| `{size_kb}` | File size in KB, floored to integer | `1024` |
| `{size_mb}` | File size in MB, floored to integer | `1` |
| `{parent}` | Immediate parent folder name | `Downloads` |
| `{counter}` | Auto-increment index in batch ops (1, 2, 3...) | `7` |
| `{counter:3}` | Zero-padded counter with width N | `007` |

**Example combining move + rename:**
```yaml
actions:
  - type: rename
    pattern: "{date_modified}_{stem}_{counter:3}.{ext}"
  - type: move
    to: "./Sorted/{year}/{month}/"
```
`report_Q1.pdf` → renamed to `2026-02-22_report_Q1_001.pdf` → moved to `./Sorted/2026/02/`

***

### Full `dirctl.yaml` Example

```yaml
version: 1
workspace: "."
default_conflict_policy: rename_with_suffix
ignore_patterns:
  - ".git/"
  - "node_modules/"
  - ".dirctl/"
  - "*.tmp"
  - "*.lock"

rules:
  - name: "archive-old-pdfs"
    description: "Move PDFs untouched for 30+ days out of root"
    enabled: true
    conflict_policy: skip
    conditions:
      - field: extension
        operator: eq
        value: "pdf"
      - field: age_days
        operator: gte
        value: 30
      - field: path
        operator: not_contains
        value: "Archives"
    actions:
      - type: move
        to: "./Archives/PDFs/{year}/{month}/"

  - name: "tag-and-quarantine-huge-files"
    description: "Tag files over 2GB, flag for review"
    conditions:
      - field: size
        operator: gte
        value: 2147483648
    actions:
      - type: tag
        tags: ["huge", "needs-review"]

  - name: "normalize-screenshots"
    description: "Standardize screenshot filenames and move to Screenshots folder"
    conditions:
      - field: name
        operator: matches
        value: "^[Ss]creenshot.+\\.png$"
    actions:
      - type: rename
        pattern: "{year}-{month}-{day}_{hour}{minute}_{hash_short}.{ext}"
      - type: move
        to: "./Screenshots/{year}-{month}/"

  - name: "clean-up-old-logs"
    description: "Quarantine log files older than 90 days"
    conditions:
      - field: extension
        operator: in
        value: ["log", "txt"]
      - field: name
        operator: contains
        value: "log"
      - field: age_days
        operator: gte
        value: 90
    actions:
      - type: delete
        strategy: quarantine
```

***

## 2. Journal Format Specification (`journal.jsonl`)

Each line in the file is one **self-contained, valid JSON object** representing a single transaction. Lines are never modified after writing — only appended. The undo system works by reading backwards and writing **new** undo-transaction entries.

### Top-Level Transaction Object

```jsonc
{
  "schema_version": 1,
  "tx_id": "01952b3a-4f1e-7c2d-9e0a-3b8f56d12e44",  // UUIDv7 (time-sortable)
  "timestamp": "2026-02-22T23:05:31.442Z",             // ISO 8601, UTC
  "workspace": "/home/user/projects/my-workspace",     // Absolute path, always
  "trigger": "apply",              // apply | watch | undo | undo-partial
  "rule_name": "archive-old-pdfs", // null if trigger is "undo"
  "undoes_tx_id": null,            // UUIDv7 of the transaction being reversed; null if not an undo
  "status": "completed",           // completed | partial | failed | undone
  "operations": [ /* see below */ ],
  "summary": {
    "total":     5,
    "completed": 4,
    "skipped":   1,
    "failed":    0
  }
}
```

**`status` values explained:**
- `completed` — All operations in the transaction succeeded
- `partial` — Some operations succeeded before a failure or interruption
- `failed` — Transaction failed before any operation completed
- `undone` — This transaction has been fully reversed by a later undo transaction

***

### Operation Object (inside `operations[]`)

```jsonc
{
  "op_id": "01952b3a-5a2f-7c2d-8b1e-4c9d67e23f55",
  "type": "move",                  // move | rename | copy | delete | tag
  "status": "completed",           // completed | skipped | failed
  "skip_reason": null,             // human-readable string if status = "skipped"
  "error": null,                   // error message string if status = "failed"
  "conflict_resolution": null,     // skip | overwrite | rename_with_suffix | null (no conflict)
  "pre": {
    "path": "/home/user/projects/my-workspace/report_Q1.pdf",
    "hash": "a3f9c12d8e4b1c7f9d2a0e5b3f8c6d1a4e7b2f5c8d3e6a9b1c4f7e2d5a8b0c3",
    "size": 204800,
    "modified_at": "2026-01-15T09:22:10.000Z"
  },
  "post": {
    "path": "/home/user/projects/my-workspace/Archives/PDFs/2026/01/report_Q1.pdf",
    "hash": "a3f9c12d8e4b1c7f9d2a0e5b3f8c6d1a4e7b2f5c8d3e6a9b1c4f7e2d5a8b0c3",
    "size": 204800,
    "modified_at": "2026-01-15T09:22:10.000Z"
  }
}
```

**Important rules for `pre` and `post`:**
- For `move` and `rename`: `pre.hash === post.hash` always (content unchanged, path changed)
- For `copy`: both `pre` and `post` are populated with their respective full paths
- For `delete` (quarantine): `post.path` is the quarantine path (`.dirctl/quarantine/<tx_id>/filename`)
- For `tag`: `pre` and `post` only contain `path` and `tags[]` arrays — no hash or size needed

***

### Tag Operation Object (special case)

```jsonc
{
  "op_id": "01952b3a-6c3g-...",
  "type": "tag",
  "status": "completed",
  "pre": {
    "path": "/home/user/projects/my-workspace/video.mp4",
    "tags": ["inbox"]
  },
  "post": {
    "path": "/home/user/projects/my-workspace/video.mp4",
    "tags": ["inbox", "large", "needs-review"]
  }
}
```

***

### Undo Transaction Entry

When `dirctl undo` runs, it does **not** delete or modify any existing journal line. It appends a brand-new transaction that is the mirror-image of the original:

```jsonc
{
  "schema_version": 1,
  "tx_id": "01952b9f-1a2b-7c3d-8e4f-5b6c7d8e9f0a",
  "timestamp": "2026-02-22T23:41:15.001Z",
  "workspace": "/home/user/projects/my-workspace",
  "trigger": "undo",
  "rule_name": null,
  "undoes_tx_id": "01952b3a-4f1e-7c2d-9e0a-3b8f56d12e44",  // points to original
  "status": "completed",
  "operations": [
    {
      "op_id": "01952b9f-2b3c-...",
      "type": "move",              // same type as original
      "status": "completed",
      "pre": {
        // This is the POST state of the original — where the file is NOW
        "path": "/home/user/projects/my-workspace/Archives/PDFs/2026/01/report_Q1.pdf",
        "hash": "a3f9c12d..."
      },
      "post": {
        // This is the PRE state of the original — where the file should go BACK to
        "path": "/home/user/projects/my-workspace/report_Q1.pdf",
        "hash": "a3f9c12d..."
      }
    }
  ],
  "summary": { "total": 1, "completed": 1, "skipped": 0, "failed": 0 }
}
```

***

### Journal Safety Rules (enforced by the engine)

These rules must be hard-coded into `dirctl-core`, not config-driven:

1. **Write-ahead:** A journal entry is written to disk *before* the filesystem operation executes
2. **Hash guard on undo:** Before reversing an operation, the engine re-hashes the file at `post.path`. If it does not match `post.hash`, the undo is **aborted** with error `UNDO_HASH_MISMATCH` — the file was externally modified
3. **Append-only:** No code path in the engine ever modifies or deletes an existing journal line. `undone` status on a transaction is determined dynamically by scanning forward for a matching `undoes_tx_id` — it is never written back
4. **Atomic line writes:** Each JSON line is written as a single `write()` syscall with a trailing newline so partial writes (power loss) produce an invalid JSON line that is skipped and logged as `journal_corrupt_entry`
5. **UUIDv7 for tx_id:** UUIDv7 is time-sortable, so the journal is always in chronological order without a separate index

***

## What You Have Now

At this point the design phase is complete. You have:

- ✅ Functional & non-functional requirements (FR + NFR)
- ✅ Full system architecture (Cargo workspace layers)
- ✅ Core data models
- ✅ `dirctl.yaml` schema — all fields, operators, actions, and template tokens
- ✅ `journal.jsonl` schema — transaction format, operation format, undo format, and safety invariants

**You can now open your editor and start writing code with zero ambiguity.** The natural first file to write is `dirctl-core/src/config.rs` — the Rust structs that deserialize `dirctl.yaml` using `serde`, since every other module depends on the config types being defined first.