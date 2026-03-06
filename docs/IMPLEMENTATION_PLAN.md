# dirctl Implementation Plan

**Version:** 1.0
**Target Release:** v0.1.0 MVP
**Date:** 2026-03-02
**Status:** Ready for Execution

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture Principles](#architecture-principles)
3. [Development Phases](#development-phases)
4. [Phase 1: Foundation & Infrastructure](#phase-1-foundation--infrastructure)
5. [Phase 2: Core Domain Layer](#phase-2-core-domain-layer)
6. [Phase 3: Configuration System](#phase-3-configuration-system)
7. [Phase 4: Scanner & Metadata](#phase-4-scanner--metadata)
8. [Phase 5: Rule Engine](#phase-5-rule-engine)
9. [Phase 6: Planner](#phase-6-planner)
10. [Phase 7: Journal & Safety](#phase-7-journal--safety)
11. [Phase 8: Executor](#phase-8-executor)
12. [Phase 9: CLI Layer](#phase-9-cli-layer)
13. [Phase 10: Testing & Validation](#phase-10-testing--validation)
14. [Phase 11: Documentation & Release](#phase-11-documentation--release)
15. [Risk Matrix](#risk-matrix)
16. [Progress Tracking](#progress-tracking)

---

## Overview

This implementation plan breaks down the dirctl v0.1.0 MVP into **10 development phases** containing **85+ atomic tasks**. Each task is designed to be completed in 2-4 hours with clear success criteria and validation steps.

### MVP Scope (v0.1.0)

**Included:**
- ✅ Core engine (Scanner → Rules → Planner → Executor → Journal)
- ✅ CLI with dry-run and apply modes
- ✅ YAML configuration system
- ✅ Full rule matching (extensions, size, age, path, name patterns)
- ✅ All actions (move, rename, copy, delete/tag with quarantine)
- ✅ Undo system with hash guards
- ✅ Conflict resolution (skip, overwrite, rename_with_suffix)
- ✅ Template tokens for paths and filenames
- ✅ Journal with append-only JSONL format

**Excluded (Future Phases):**
- ❌ Watch mode (FR-7) → v0.2.0
- ❌ AI rule generation → v0.3.0
- ❌ GUI/TUI → v0.4.0
- ❌ Remote filesystem adapters → v0.5.0

---

## Architecture Principles

### Non-Negotiable Invariants

1. **Clean Architecture Boundaries**
   - `dirctl-core` = Pure domain logic, zero IO
   - `dirctl-fs-*` = Filesystem adapters implementing ports
   - `dirctl-cli` = CLI glue code only
   - Dependency direction: Always inward

2. **Domain-Driven Design**
   - Use ubiquitous language: Workspace, Rule, Condition, Planner, Plan, Operation, Journal, Transaction
   - NO generic modules: utils, helpers, common, shared, manager
   - Each module = Clear domain responsibility

3. **Determinism Contract**
   - Identical inputs = Identical Plan outputs
   - Ordering key: `(rule_priority, file_path_depth, lexicographic_path)`
   - No HashMap iteration affecting semantics
   - All iteration explicitly sorted with `IndexMap`

4. **Safety Model**
   - Journal write BEFORE execution
   - Undo verifies file hash before reversal
   - Delete = quarantine to `.dirctl/quarantine/<tx_id>/`
   - Path traversal rejection in templates

5. **Rust Standards**
   - No `unwrap()` or `expect()` in core
   - Use `thiserror` for error types
   - Functions ≤ 50 lines, files ≤ 200 lines
   - Nesting depth ≤ 3 levels

---

## Development Phases

### Phase Summary

| Phase | Name | Tasks | Est. Time | Critical Path |
|-------|------|-------|-----------|---------------|
| 1 | Foundation & Infrastructure | 8 | 2-3 days | ✅ Yes |
| 2 | Core Domain Layer | 10 | 3-4 days | ✅ Yes |
| 3 | Configuration System | 9 | 2-3 days | ✅ Yes |
| 4 | Scanner & Metadata | 10 | 3-4 days | ✅ Yes |
| 5 | Rule Engine | 8 | 2-3 days | ✅ Yes |
| 6 | Planner | 7 | 2-3 days | ✅ Yes |
| 7 | Journal & Safety | 11 | 3-4 days | ✅ Yes |
| 8 | Executor | 9 | 2-3 days | ✅ Yes |
| 9 | CLI Layer | 8 | 2-3 days | Yes |
| 10 | Testing & Validation | 10 | 3-4 days | Yes |
| 11 | Documentation & Release | 6 | 2 days | No |

**Total:** 85+ tasks, ~35-45 days for solo developer

---

## Phase 1: Foundation & Infrastructure

**Goal:** Set up project structure, tooling, and basic infrastructure.

**Success Criteria:**
- ✅ Cargo workspace compiles with all crates
- ✅ CI/CD pipeline runs tests on push
- ✅ Basic error handling infrastructure in place
- ✅ Code formatting and linting configured

### Tasks

#### 1.1 Initialize Cargo Workspace ✅ COMPLETED

**File:** `Cargo.toml` (root)

**Status:** ✅ Completed 2026-03-03

**What was done:**
- Created workspace using `cargo new` commands
- Set up three crates: dirctl-core (lib), dirctl-fs-local (lib), dirctl-cli (binary)
- Configured workspace Cargo.toml with pinned dependency versions for determinism
- Updated all crate Cargo.toml files to use workspace dependencies
- Validated workspace compiles successfully

**Architecture Compliance:**
- ✅ Clean Architecture boundaries enforced (core → pure, fs-local → adapter, cli → glue)
- ✅ Dependency direction: All point inward (fs-local depends on core, cli depends on core)
- ✅ Core has no IO dependencies (tokio only in fs-local)
- ✅ Using thiserror for error handling
- ✅ Using uuid v7 for time-sortable IDs
- ✅ Using indexmap for deterministic iteration
- ✅ Using serde for serialization

**Validation:**
```bash
cargo check --workspace
# ✅ PASSED - All crates compiled successfully in 47.32s
```

---

#### 1.2 Create Core Crate Skeleton ✅ COMPLETED

**Status:** ✅ Completed 2026-03-03

**What was done:**
- Verified dirctl-core/Cargo.toml has all required dependencies
- Created dirctl-core/src/lib.rs with simple skeleton and documentation
- Clean library structure for pure domain logic (no I/O)

**Validation:**
```bash
cargo check -p dirctl-core
# ✅ PASSED - Compiled successfully in 0.35s
```

**Files:**
- `dirctl-core/Cargo.toml`
- `dirctl-core/src/lib.rs`

**Action:**
```toml
# dirctl-core/Cargo.toml
[package]
name = "dirctl-core"
version.workspace = true
edition.workspace = true

[dependencies]
thiserror.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
chrono.workspace = true
indexmap.workspace = true
regex.workspace = true
tracing.workspace = true
```

**Validation:**
```bash
cargo check -p dirctl-core
```

---

#### 1.3 Create Filesystem Adapter Crate ✅ COMPLETED

**Status:** ✅ Completed 2026-03-03

**What was done:**
- Verified dirctl-fs-local/Cargo.toml has all required dependencies
- Created dirctl-fs-local/src/lib.rs with simple skeleton
- Adapter structure ready to implement ports from dirctl-core

**Validation:**
```bash
cargo check -p dirctl-fs-local
# ✅ PASSED - Compiled successfully in 3.14s
```

**Files:**
- `dirctl-fs-local/Cargo.toml`
- `dirctl-fs-local/src/lib.rs`

**Action:**
```toml
# dirctl-fs-local/Cargo.toml
[package]
name = "dirctl-fs-local"
version.workspace = true
edition.workspace = true

[dependencies]
dirctl-core = { path = "../dirctl-core" }
tokio = { version = "1.40", features = ["fs", "io-util"] }
thiserror.workspace = true
serde.workspace = true
uuid.workspace = true
tracing.workspace = true
```

**Validation:**
```bash
cargo check -p dirctl-fs-local
```

---

#### 1.4 Create CLI Crate Skeleton ✅ COMPLETED

**Status:** ✅ Completed 2026-03-03

**What was done:**
- Verified dirctl-cli/Cargo.toml has all required dependencies
- Created dirctl-cli/src/main.rs with clap CLI skeleton
- Binary name "dirctl" configured with --debug flag and --help

**Validation:**
```bash
cargo check -p dirctl-cli
# ✅ PASSED - Compiled successfully in 0.49s

cargo run -p dirctl-cli -- --help
# ✅ PASSED - Help output displays correctly
```

**Files:**
- `dirctl-cli/Cargo.toml`
- `dirctl-cli/src/main.rs`

**Action:**
```toml
# dirctl-cli/Cargo.toml
[package]
name = "dirctl-cli"
version.workspace = true
edition.workspace = true

[[bin]]
name = "dirctl"
path = "src/main.rs"

[dependencies]
dirctl-core = { path = "../dirctl-core" }
dirctl-fs-local = { path = "../dirctl-fs-local" }
clap.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
thiserror.workspace = true
```

**Validation:**
```bash
cargo check -p dirctl-cli
cargo run -p dirctl-cli -- --help
```

---

#### 1.5 Define Core Error Types ✅ COMPLETED

**Status:** ✅ Completed 2026-03-07

**What was done:**
- Created dirctl-core/src/error.rs with comprehensive error types
- Implemented all 13 error variants using thiserror
- Added Result<T> type alias for ergonomic error handling
- Included unit tests for error display and Result type
- Updated dirctl-core/src/lib.rs to expose error module

**Architecture Compliance:**
- ✅ No unwrap() or expect() used
- ✅ Uses thiserror for proper error handling
- ✅ All errors have clear, descriptive messages
- ✅ Result<T> type alias for consistent error handling

**Validation:**
```bash
cargo test -p dirctl-core error
# ✅ PASSED - 2 tests passed in 0.00s
```

**File:** `dirctl-core/src/error.rs`

**Action:**
```rust
use std::path::PathBuf;

/// Core error type for dirctl
///
/// All errors in core MUST use this type - no unwrap() or expect()
#[derive(thiserror::Error, Debug)]
pub enum DirctlError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Rule evaluation failed for rule '{rule}': {reason}")]
    RuleEvalFailed { rule: String, reason: String },

    #[error("Journal error: {0}")]
    Journal(String),

    #[error("Journal corrupt: transaction {tx_id} has invalid format")]
    JournalCorrupt { tx_id: String },

    #[error("Undo hash mismatch for file '{path}': expected {expected}, found {actual}")]
    UndoHashMismatch {
        path: PathBuf,
        expected: String,
        actual: String
    },

    #[error("IO error via port: {0}")]
    PortIo(String),

    #[error("Invalid path template: {0}")]
    InvalidTemplate(String),

    #[error("Path traversal detected in template: {0}")]
    PathTraversal(String),

    #[error("Conflict resolution failed: {0}")]
    ConflictResolution(String),

    #[error("Scanner error: {0}")]
    Scanner(String),

    #[error("Executor error: {0}")]
    Executor(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, DirctlError>;
```

**Validation:**
```bash
cargo test -p dirctl-core error
```

---

#### 1.6 Setup Testing Infrastructure

**Files:**
- `dirctl-core/src/test_utils.rs`
- `.cargo/config.toml`

**Action:**
```rust
// dirctl-core/src/test_utils.rs
/// Test utilities for dirctl-core
///
/// ONLY use in #[cfg(test)] modules - never in production code

use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Creates a temporary workspace for testing
pub fn create_test_workspace() -> TempDir {
    TempDir::new().expect("Failed to create temp dir")
}

/// Creates a test file with content
pub fn create_test_file(path: &Path, content: &[u8]) -> std::io::Result<()> {
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::write(path, content)
}
```

```toml
# .cargo/config.toml
[build]
target-dir = "target"

[target.'cfg(not(windows))']
rustflags = ["--cfg", "tokio_unstable"]

[profile.dev]
opt-level = 0

[profile.test]
opt-level = 1
```

**Validation:**
```bash
cargo test --workspace
```

---

#### 1.7 Configure Code Quality Tools

**Files:**
- `.rustfmt.toml`
- `clippy.toml`
- `deny.toml` (optional)

**Action:**
```toml
# .rustfmt.toml
max_width = 100
hard_tabs = false
tab_spaces = 4
use_small_heuristics = "Default"
merge_derives = true
use_try_shorthand = false
use_field_init_shorthand = false
force_explicit_abi = true
format_code_in_doc_comments = true
format_strings = true
```

```toml
# clippy.toml
# Restrictive settings for core library
too-many-arguments-threshold = 7
type-complexity-threshold = 250
```

**Validation:**
```bash
cargo fmt -- --check
cargo clippy --workspace --all-targets
```

---

#### 1.8 Setup CI/CD Pipeline

**File:** `.github/workflows/test.yml`

**Action:**
```yaml
name: Test

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable]

    steps:
      - uses: actions/checkout@v4

      - uses: actions-rust-lang/setup-rust-toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}

      - name: Cache dependencies
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Check formatting
        run: cargo fmt -- --check

      - name: Run Clippy
        run: cargo clippy --workspace --all-targets -- -D warnings

      - name: Run tests
        run: cargo test --workspace --all-targets

      - name: Run doc tests
        run: cargo test --workspace --doc
```

**Validation:**
- Push to GitHub and verify workflow runs successfully

---

### Phase 1 Validation

```bash
# All checks should pass
cargo build --workspace
cargo test --workspace
cargo fmt -- --check
cargo clippy --workspace --all-targets
```

**Phase 1 Complete When:**
- ✅ All crates compile without warnings
- ✅ CI/CD pipeline passes on all OSes
- ✅ Error types defined and tested
- ✅ Code quality tools configured

---

## Phase 2: Core Domain Layer

**Goal:** Define all domain entities, value objects, and port interfaces.

**Success Criteria:**
- ✅ All domain types defined
- ✅ Port interfaces specified
- ✅ Types serialize/deserialize correctly
- ✅ No domain logic depends on infrastructure

### Tasks

#### 2.1 Define Workspace Domain Entity

**File:** `dirctl-core/src/domain/workspace.rs`

**Action:**
```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A workspace represents a directory tree that dirctl manages
///
/// # Invariants
/// - Path must be absolute
/// - Path must exist
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Workspace {
    /// Absolute path to the workspace root
    pub path: PathBuf,
}

impl Workspace {
    /// Creates a new workspace from a path
    ///
    /// # Errors
    /// Returns error if path is not absolute or doesn't exist
    pub fn new(path: PathBuf) -> crate::Result<Self> {
        if !path.is_absolute() {
            return Err(DirctlError::Config(
                "Workspace path must be absolute".into(),
            ));
        }
        // Note: Existence check happens via Port, not in core
        Ok(Self { path })
    }

    /// Returns the workspace root path
    pub fn root(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_relative_path() {
        let result = Workspace::new(PathBuf::from("relative/path"));
        assert!(result.is_err());
    }

    #[test]
    fn accepts_absolute_path() {
        let result = Workspace::new(PathBuf::from("/absolute/path"));
        assert!(result.is_ok());
    }
}
```

**Validation:**
```bash
cargo test -p dirctl-core workspace
```

---

#### 2.2 Define Rule Domain Entity

**File:** `dirctl-core/src/domain/rule.rs`

**Action:**
```rust
use serde::{Deserialize, Serialize};
use crate::domain::{Condition, Action, ConflictPolicy};

/// A rule defines matching conditions and actions to apply
///
/// # Invariants
/// - Rules have unique names
/// - Rules are evaluated in priority order (higher first)
/// - All conditions must match (AND logic) for rule to apply
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rule {
    /// Unique identifier for this rule (kebab-case)
    pub name: String,

    /// Human-readable description
    pub description: Option<String>,

    /// Whether this rule is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    /// Evaluation priority (higher = evaluated first)
    #[serde(default)]
    pub priority: i32,

    /// Match all files, or stop after first match?
    #[serde(default)]
    pub match_all: bool,

    /// Override default conflict policy for this rule
    pub conflict_policy: Option<ConflictPolicy>,

    /// All conditions must match (AND logic)
    pub conditions: Vec<Condition>,

    /// Actions to execute in order
    pub actions: Vec<Action>,
}

fn default_enabled() -> bool {
    true
}

impl Rule {
    /// Creates a new rule with required fields
    pub fn new(
        name: String,
        conditions: Vec<Condition>,
        actions: Vec<Action>,
    ) -> Self {
        Self {
            name,
            description: None,
            enabled: true,
            priority: 0,
            match_all: false,
            conflict_policy: None,
            conditions,
            actions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_serialization_roundtrip() {
        let rule = Rule::new(
            "test-rule".into(),
            vec![],
            vec![],
        );
        let json = serde_json::to_string(&rule).unwrap();
        let deserialized: Rule = serde_json::from_str(&json).unwrap();
        assert_eq!(rule, deserialized);
    }
}
```

**Validation:**
```bash
cargo test -p dirctl-core rule
```

---

#### 2.3 Define Condition Value Objects

**File:** `dirctl-core/src/domain/condition.rs`

**Action:**
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Field to evaluate in a condition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionField {
    Name,
    Extension,
    MimeType,
    Size,
    AgeDays,
    Path,
    Tag,
    ModifiedAt,
    CreatedAt,
}

/// Operator for condition evaluation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOperator {
    // String operators
    Eq,
    Neq,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    Matches,
    NotMatches,
    In,
    NotIn,

    // Numeric operators
    Gt,
    Lt,
    Gte,
    Lte,
    Between,

    // Date operators
    Before,
    After,

    // Tag operators
    Has,
    NotHas,
}

/// Value for condition comparison
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionValue {
    String(String),
    Integer(i64),
    StringList(Vec<String>),
    IntegerList(Vec<i64>),
}

/// A single condition in a rule
///
/// # Invariants
/// - Field and operator combination must be valid
/// - Value type must match operator requirements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub field: ConditionField,
    pub operator: ConditionOperator,
    pub value: ConditionValue,
}

impl Condition {
    /// Creates a new condition
    pub fn new(
        field: ConditionField,
        operator: ConditionOperator,
        value: ConditionValue,
    ) -> Self {
        Self {
            field,
            operator,
            value,
        }
    }

    /// Validates this condition's type constraints
    pub fn validate(&self) -> crate::Result<()> {
        // Validate field/operator combinations
        match (&self.field, &self.operator) {
            (ConditionField::Size, ConditionOperator::Between) => {
                match &self.value {
                    ConditionValue::IntegerList(v) if v.len() == 2 => Ok(()),
                    _ => Err(DirctlError::Config(
                        "Size::between requires [min, max] integer list".into(),
                    )),
                }
            }
            (ConditionField::Extension, ConditionOperator::In) => {
                match &self.value {
                    ConditionValue::StringList(v) if !v.is_empty() => Ok(()),
                    _ => Err(DirctlError::Config(
                        "Extension::in requires non-empty string list".into(),
                    )),
                }
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_size_between() {
        let cond = Condition::new(
            ConditionField::Size,
            ConditionOperator::Between,
            ConditionValue::IntegerList(vec![100, 1000]),
        );
        assert!(cond.validate().is_ok());
    }

    #[test]
    fn rejects_invalid_size_between() {
        let cond = Condition::new(
            ConditionField::Size,
            ConditionOperator::Between,
            ConditionValue::Integer(100),
        );
        assert!(cond.validate().is_err());
    }
}
```

**Validation:**
```bash
cargo test -p dirctl-core condition
```

---

#### 2.4 Define Action Value Objects

**File:** `dirctl-core/src/domain/action.rs`

**Action:**
```rust
use serde::{Deserialize, Serialize};

/// Action type to execute
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Move,
    Rename,
    Copy,
    Delete,
    Tag,
}

/// Conflict resolution policy
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictPolicy {
    /// Skip this operation if conflict exists
    Skip,

    /// Overwrite existing file
    Overwrite,

    /// Rename with suffix (_1, _2, etc.)
    RenameWithSuffix,
}

/// Delete strategy
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteStrategy {
    /// Move to quarantine directory
    Quarantine,

    /// Mark in journal only (v0.2.0+)
    Soft,
}

/// An action to execute on a matched file
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: ActionType,

    /// For move/copy: destination path template
    pub to: Option<String>,

    /// For move/copy: create parent directories
    #[serde(default = "default_create_dirs")]
    pub create_dirs: bool,

    /// For rename: filename pattern template
    pub pattern: Option<String>,

    /// For delete: strategy to use
    pub strategy: Option<DeleteStrategy>,

    /// For tag: tags to add
    pub tags: Option<Vec<String>>,

    /// For tag: tags to remove
    pub remove_tags: Option<Vec<String>>,
}

fn default_create_dirs() -> bool {
    true
}

impl Action {
    /// Creates a move action
    pub fn move_to(to: String) -> Self {
        Self {
            action_type: ActionType::Move,
            to: Some(to),
            create_dirs: true,
            pattern: None,
            strategy: None,
            tags: None,
            remove_tags: None,
        }
    }

    /// Creates a rename action
    pub fn rename(pattern: String) -> Self {
        Self {
            action_type: ActionType::Rename,
            to: None,
            create_dirs: false,
            pattern: Some(pattern),
            strategy: None,
            tags: None,
            remove_tags: None,
        }
    }

    /// Creates a copy action
    pub fn copy_to(to: String) -> Self {
        Self {
            action_type: ActionType::Copy,
            to: Some(to),
            create_dirs: true,
            pattern: None,
            strategy: None,
            tags: None,
            remove_tags: None,
        }
    }

    /// Creates a delete action
    pub fn delete(strategy: DeleteStrategy) -> Self {
        Self {
            action_type: ActionType::Delete,
            to: None,
            create_dirs: false,
            pattern: None,
            strategy: Some(strategy),
            tags: None,
            remove_tags: None,
        }
    }

    /// Creates a tag action
    pub fn tag(tags: Vec<String>) -> Self {
        Self {
            action_type: ActionType::Tag,
            to: None,
            create_dirs: false,
            pattern: None,
            strategy: None,
            tags: Some(tags),
            remove_tags: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_serialization_roundtrip() {
        let action = Action::move_to("./dest/".into());
        let json = serde_json::to_string(&action).unwrap();
        let deserialized: Action = serde_json::from_str(&json).unwrap();
        assert_eq!(action, deserialized);
    }
}
```

**Validation:**
```bash
cargo test -p dirctl-core action
```

---

#### 2.5 Define File Metadata Value Object

**File:** `dirctl-core/src/domain/metadata.rs`

**Action:**
```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashSet;

/// Metadata about a file in the workspace
///
/// # Invariants
/// - Path is relative to workspace root
/// - Hash is SHA-256 hex string
/// - Size is in bytes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileMetadata {
    /// Path relative to workspace root
    pub path: PathBuf,

    /// File size in bytes
    pub size: u64,

    /// SHA-256 hash of file content
    pub hash: String,

    /// Last modified timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,

    /// Created timestamp (if available)
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// MIME type (if detected)
    pub mime_type: Option<String>,

    /// User-assigned tags
    #[serde(default)]
    pub tags: HashSet<String>,
}

impl FileMetadata {
    /// Creates new file metadata
    pub fn new(
        path: PathBuf,
        size: u64,
        hash: String,
        modified_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            path,
            size,
            hash,
            modified_at,
            created_at: None,
            mime_type: None,
            tags: HashSet::new(),
        }
    }

    /// Returns file extension (without dot)
    pub fn extension(&self) -> Option<&str> {
        self.path
            .extension()
            .and_then(|ext| ext.to_str())
    }

    /// Returns file name with extension
    pub fn name(&self) -> &str {
        self.path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
    }

    /// Returns file name without extension
    pub fn stem(&self) -> &str {
        self.path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("")
    }

    /// Returns age in days (from modified_at to now)
    ///
    /// Note: This requires time injection via Port in core
    pub fn age_days(&self, now: chrono::DateTime<chrono::Utc>) -> i64 {
        let duration = now.signed_duration_since(self.modified_at);
        duration.num_days()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_extension() {
        let meta = FileMetadata::new(
            PathBuf::from("test.txt"),
            100,
            "hash".into(),
            chrono::Utc::now(),
        );
        assert_eq!(meta.extension(), Some("txt"));
    }

    #[test]
    fn extracts_stem() {
        let meta = FileMetadata::new(
            PathBuf::from("document.pdf"),
            100,
            "hash".into(),
            chrono::Utc::now(),
        );
        assert_eq!(meta.stem(), "document");
    }
}
```

**Validation:**
```bash
cargo test -p dirctl-core metadata
```

---

#### 2.6 Define Operation Domain Entity

**File:** `dirctl-core/src/domain/operation.rs`

**Action:**
```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Type of operation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    Move,
    Rename,
    Copy,
    Delete,
    Tag,
}

/// Status of an operation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationStatus {
    Completed,
    Skipped,
    Failed,
}

/// File state before/after operation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileState {
    pub path: PathBuf,
    pub hash: String,
    pub size: u64,
    pub modified_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Tag state (for tag operations)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagState {
    pub path: PathBuf,
    pub tags: Vec<String>,
}

/// A single operation within a transaction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Operation {
    /// Unique operation ID
    pub op_id: Uuid,

    /// Type of operation
    #[serde(rename = "type")]
    pub op_type: OperationType,

    /// Status
    pub status: OperationStatus,

    /// Skip reason (if skipped)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_reason: Option<String>,

    /// Error (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Conflict resolution used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution: Option<String>,

    /// State before operation
    pub pre: FileState,

    /// State after operation
    pub post: FileState,
}

impl Operation {
    /// Creates a new operation
    pub fn new(
        op_type: OperationType,
        pre: FileState,
        post: FileState,
    ) -> Self {
        Self {
            op_id: Uuid::new_v7(),
            op_type,
            status: OperationStatus::Completed,
            skip_reason: None,
            error: None,
            conflict_resolution: None,
            pre,
            post,
        }
    }

    /// Marks operation as skipped
    pub fn skipped(mut self, reason: String) -> Self {
        self.status = OperationStatus::Skipped;
        self.skip_reason = Some(reason);
        self
    }

    /// Marks operation as failed
    pub fn failed(mut self, error: String) -> Self {
        self.status = OperationStatus::Failed;
        self.error = Some(error);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_serialization_roundtrip() {
        let pre = FileState {
            path: PathBuf::from("test.txt"),
            hash: "abc123".into(),
            size: 100,
            modified_at: chrono::Utc::now(),
            tags: None,
        };
        let post = pre.clone();

        let op = Operation::new(OperationType::Move, pre, post);
        let json = serde_json::to_string(&op).unwrap();
        let deserialized: Operation = serde_json::from_str(&json).unwrap();
        assert_eq!(op.op_id, deserialized.op_id);
    }
}
```

**Validation:**
```bash
cargo test -p dirctl-core operation
```

---

#### 2.7 Define Transaction Domain Entity

**File:** `dirctl-core/src/domain/transaction.rs`

**Action:**
```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;
use crate::domain::{Operation, TransactionStatus};

/// Trigger type for transaction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerType {
    Apply,
    Watch,
    Undo,
    UndoPartial,
}

/// Summary of transaction results
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionSummary {
    pub total: usize,
    pub completed: usize,
    pub skipped: usize,
    pub failed: usize,
}

/// A transaction represents a set of operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    /// Schema version for forward compatibility
    pub schema_version: u32,

    /// Unique transaction ID (UUIDv7 for time-sorting)
    pub tx_id: Uuid,

    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// Workspace path (absolute)
    pub workspace: PathBuf,

    /// What triggered this transaction
    #[serde(rename = "trigger")]
    pub trigger_type: TriggerType,

    /// Rule name that generated this (null for undo)
    pub rule_name: Option<String>,

    /// Transaction this undoes (null if not undo)
    pub undoes_tx_id: Option<Uuid>,

    /// Status
    pub status: TransactionStatus,

    /// Operations in this transaction
    pub operations: Vec<Operation>,

    /// Summary
    pub summary: TransactionSummary,
}

impl Transaction {
    /// Creates a new transaction
    pub fn new(
        workspace: PathBuf,
        trigger_type: TriggerType,
        rule_name: Option<String>,
    ) -> Self {
        Self {
            schema_version: 1,
            tx_id: Uuid::new_v7(),
            timestamp: chrono::Utc::now(),
            workspace,
            trigger_type,
            rule_name,
            undoes_tx_id: None,
            status: TransactionStatus::Completed,
            operations: Vec::new(),
            summary: TransactionSummary {
                total: 0,
                completed: 0,
                skipped: 0,
                failed: 0,
            },
        }
    }

    /// Adds an operation to this transaction
    pub fn add_operation(&mut self, op: Operation) {
        match op.status {
            crate::domain::OperationStatus::Completed => {
                self.summary.completed += 1;
            }
            crate::domain::OperationStatus::Skipped => {
                self.summary.skipped += 1;
            }
            crate::domain::OperationStatus::Failed => {
                self.summary.failed += 1;
            }
        }
        self.summary.total += 1;
        self.operations.push(op);
    }

    /// Creates an undo transaction for this transaction
    pub fn create_undo(&self) -> Self {
        let mut undo = Self::new(
            self.workspace.clone(),
            TriggerType::Undo,
            None,
        );
        undo.undoes_tx_id = Some(self.tx_id);

        // Reverse operations will be added by executor
        undo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_serialization_roundtrip() {
        let tx = Transaction::new(
            PathBuf::from("/workspace"),
            TriggerType::Apply,
            Some("test-rule".into()),
        );
        let json = serde_json::to_string(&tx).unwrap();
        let deserialized: Transaction = serde_json::from_str(&json).unwrap();
        assert_eq!(tx.tx_id, deserialized.tx_id);
    }
}
```

**Validation:**
```bash
cargo test -p dirctl-core transaction
```

---

#### 2.8 Define Transaction Status Enum

**File:** `dirctl-core/src/domain/mod.rs` (add to existing)

**Action:**
```rust
/// Status of a transaction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    Completed,
    Partial,
    Failed,
    Undone,
}
```

---

#### 2.9 Define Port: FileSystem Scanner

**File:** `dirctl-core/src/ports/scanner.rs`

**Action:**
```rust
use async_trait::async_trait;
use std::path::Path;
use crate::domain::{FileMetadata, Workspace};

/// Port for filesystem scanning operations
///
/// Implementations MUST:
/// - Return results in deterministic order
/// - Provide complete metadata for each file
/// - Follow ignore patterns
#[async_trait]
pub trait FileSystemScanner: Send + Sync {
    /// Scan workspace and return file metadata
    ///
    /// # Requirements
    /// - Results MUST be sorted by (depth, path) for determinism
    /// - MUST ignore patterns from workspace config
    /// - MUST include all files matching criteria
    async fn scan_workspace(
        &self,
        workspace: &Workspace,
    ) -> crate::Result<Vec<FileMetadata>>;

    /// Scan specific directory
    async fn scan_directory(
        &self,
        path: &Path,
    ) -> crate::Result<Vec<FileMetadata>>;

    /// Check if path exists
    async fn exists(&self, path: &Path) -> bool;

    /// Get file hash
    async fn hash_file(&self, path: &Path) -> crate::Result<String>;
}
```

**Validation:**
```bash
cargo test -p dirctl-core scanner
```

---

#### 2.10 Define Port: File Executor

**File:** `dirctl-core/src/ports/executor.rs`

**Action:**
```rust
use async_trait::async_trait;
use std::path::Path;
use crate::domain::{FileMetadata, Operation, Transaction, ConflictPolicy};

/// Port for file execution operations
///
/// Implementations MUST:
/// - Write journal entry BEFORE executing operations
/// - Verify hash for undo operations
/// - Support all conflict policies
#[async_trait]
pub trait FileExecutor: Send + Sync {
    /// Execute a single operation
    ///
    /// # Safety Requirements
    /// - Journal entry MUST be written first
    /// - MUST support conflict resolution
    async fn execute_operation(
        &self,
        operation: &Operation,
        conflict_policy: ConflictPolicy,
    ) -> crate::Result<Operation>;

    /// Execute all operations in a transaction
    ///
    /// # Safety Requirements
    /// - Write transaction to journal first
    /// - Execute operations in order
    /// - Update operation status
    async fn execute_transaction(
        &self,
        transaction: &mut Transaction,
    ) -> crate::Result<()>;

    /// Undo a transaction
    ///
    /// # Safety Requirements
    /// - MUST verify file hash before reversing
    /// - MUST write undo transaction to journal
    async fn undo_transaction(
        &self,
        tx_id: &str,
    ) -> crate::Result<Transaction>;

    /// Move file to destination
    async fn move_file(
        &self,
        from: &Path,
        to: &Path,
        create_dirs: bool,
    ) -> crate::Result<()>;

    /// Copy file to destination
    async fn copy_file(
        &self,
        from: &Path,
        to: &Path,
        create_dirs: bool,
    ) -> crate::Result<()>;

    /// Delete file (quarantine or actual)
    async fn delete_file(
        &self,
        path: &Path,
        tx_id: &str,
    ) -> crate::Result<()>;
}
```

**Validation:**
```bash
cargo test -p dirctl-core executor
```

---

### Phase 2 Validation

```bash
# All domain types should compile and test
cargo test -p dirctl-core --lib

# Verify serialization works
cargo test -p dirctl-core serialization

# Check no unwrap/expect in core
cargo clippy -p dirctl-core -- -D clippy::unwrap_used -D clippy::expect_used
```

**Phase 2 Complete When:**
- ✅ All domain entities defined
- ✅ All port interfaces specified
- ✅ Serialization tests pass
- ✅ No unwrap/expect in core

---

## Phase 3: Configuration System

**Goal:** Implement YAML configuration parsing and validation.

**Success Criteria:**
- ✅ `dirctl.yaml` parses correctly
- ✅ All rule configurations validated
- ✅ Default values applied correctly
- ✅ Error messages are clear

### Tasks

#### 3.1 Define Configuration Structures

**File:** `dirctl-core/src/config.rs`

**Action:**
```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::domain::{Rule, ConflictPolicy};
use indexmap::IndexMap;

/// Main configuration structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirctlConfig {
    /// Schema version
    pub version: u32,

    /// Workspace path
    pub workspace: PathBuf,

    /// Default conflict policy
    #[serde(default = "default_conflict_policy")]
    pub default_conflict_policy: ConflictPolicy,

    /// Ignore patterns (gitignore-style)
    #[serde(default)]
    pub ignore_patterns: Vec<String>,

    /// Rules (ordered for evaluation)
    #[serde(default)]
    pub rules: Vec<Rule>,
}

fn default_conflict_policy() -> ConflictPolicy {
    ConflictPolicy::Skip
}

impl DirctlConfig {
    /// Creates a new config with defaults
    pub fn new(workspace: PathBuf) -> Self {
        Self {
            version: 1,
            workspace,
            default_conflict_policy: ConflictPolicy::Skip,
            ignore_patterns: vec![
                ".git/".into(),
                "node_modules/".into(),
                ".dirctl/".into(),
            ],
            rules: Vec::new(),
        }
    }

    /// Loads config from YAML file
    pub fn from_yaml(path: &Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| DirctlError::Config(format!("Failed to read config: {}", e)))?;

        let config: Self = serde_yaml::from_str(&content)
            .map_err(|e| DirctlError::Config(format!("Invalid YAML: {}", e)))?;

        config.validate()?;
        Ok(config)
    }

    /// Saves config to YAML file
    pub fn to_yaml(&self, path: &Path) -> crate::Result<()> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| DirctlError::Config(format!("Failed to serialize: {}", e)))?;

        std::fs::write(path, content)
            .map_err(|e| DirctlError::Config(format!("Failed to write config: {}", e)))?;

        Ok(())
    }

    /// Validates configuration
    pub fn validate(&self) -> crate::Result<()> {
        // Check workspace path
        if self.workspace.as_os_str().is_empty() {
            return Err(DirctlError::Config("Workspace path cannot be empty".into()));
        }

        // Check for duplicate rule names
        let mut rule_names = std::collections::HashSet::new();
        for rule in &self.rules {
            if !rule_names.insert(&rule.name) {
                return Err(DirctlError::Config(
                    format!("Duplicate rule name: {}", rule.name)
                ));
            }
            // Validate conditions
            for cond in &rule.conditions {
                cond.validate()?;
            }
        }

        Ok(())
    }

    /// Returns enabled rules sorted by priority
    pub fn enabled_rules(&self) -> Vec<&Rule> {
        let mut rules: Vec<&Rule> = self.rules
            .iter()
            .filter(|r| r.enabled)
            .collect();

        // Sort by priority (descending), then name for determinism
        rules.sort_by(|a, b| {
            b.priority.cmp(&a.priority)
                .then_with(|| a.name.cmp(&b.name))
        });

        rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_roundtrip() {
        let config = DirctlConfig::new(PathBuf::from("."));
        let yaml = serde_yaml::to_string(&config).unwrap();
        let parsed: DirctlConfig = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(config.version, parsed.version);
    }

    #[test]
    fn detects_duplicate_rules() {
        let mut config = DirctlConfig::new(PathBuf::from("."));
        config.rules = vec![
            Rule::new("duplicate".into(), vec![], vec![]),
            Rule::new("duplicate".into(), vec![], vec![]),
        ];
        assert!(config.validate().is_err());
    }
}
```

**Validation:**
```bash
cargo test -p dirctl-core config
```

---

#### 3.2 Add Template Token System

**File:** `dirctl-core/src/template.rs`

**Action:**
```rust
use std::collections::HashMap;
use crate::domain::FileMetadata;
use chrono::{DateTime, Utc};

/// Template token replacement engine
///
/// Replaces tokens like {year}, {month}, {name} in templates
pub struct TemplateEngine;

impl TemplateEngine {
    /// Renders a template with file metadata
    ///
    /// # Supported Tokens
    /// - {name} - Full filename with extension
    /// - {stem} - Filename without extension
    /// - {ext} - Extension without dot
    /// - {year} - 4-digit year from modified_at
    /// - {month} - 2-digit month
    /// - {day} - 2-digit day
    /// - {hour} - 2-digit hour (24h)
    /// - {minute} - 2-digit minute
    /// - {second} - 2-digit second
    /// - {date_modified} - YYYY-MM-DD
    /// - {date_created} - YYYY-MM-DD
    /// - {hash_short} - First 8 chars of SHA-256
    /// - {hash_full} - Full SHA-256
    /// - {size_kb} - Size in KB (floored)
    /// - {size_mb} - Size in MB (floored)
    /// - {parent} - Parent folder name
    pub fn render(template: &str, metadata: &FileMetadata) -> crate::Result<String> {
        let mut result = template.to_string();

        // Extract date components
        let modified = &metadata.modified_at;
        let created = metadata.created_at.as_ref().unwrap_or(modified);

        // Replace date/time tokens
        result = result.replace("{year}", &modified.format("%Y").to_string());
        result = result.replace("{month}", &modified.format("%m").to_string());
        result = result.replace("{day}", &modified.format("%d").to_string());
        result = result.replace("{hour}", &modified.format("%H").to_string());
        result = result.replace("{minute}", &modified.format("%M").to_string());
        result = result.replace("{second}", &modified.format("%S").to_string());

        // Replace date shortcuts
        result = result.replace("{date_modified}", &modified.format("%Y-%m-%d").to_string());
        result = result.replace("{date_created}", &created.format("%Y-%m-%d").to_string());

        // Replace file tokens
        result = result.replace("{name}", metadata.name());
        result = result.replace("{stem}", metadata.stem());
        result = result.replace("{ext}", metadata.extension().unwrap_or(""));

        // Replace hash tokens
        let hash_short = &metadata.hash.chars().take(8).collect::<String>();
        result = result.replace("{hash_short}", hash_short);
        result = result.replace("{hash_full}", &metadata.hash);

        // Replace size tokens
        let size_kb = metadata.size / 1024;
        let size_mb = metadata.size / (1024 * 1024);
        result = result.replace("{size_kb}", &size_kb.to_string());
        result = result.replace("{size_mb}", &size_mb.to_string());

        // Replace parent folder
        if let Some(parent) = metadata.path.parent() {
            if let Some(folder_name) = parent.file_name() {
                result = result.replace("{parent}", folder_name.to_str().unwrap_or(""));
            }
        }

        // Check for path traversal attempts
        if result.contains("..") {
            return Err(DirctlError::PathTraversal(
                template.to_string()
            ));
        }

        Ok(result)
    }

    /// Renders template with counter
    pub fn render_with_counter(
        template: &str,
        metadata: &FileMetadata,
        counter: usize,
        padding: Option<usize>,
    ) -> crate::Result<String> {
        let result = Self::render(template, metadata)?;

        let counter_str = match padding {
            Some(width) => format!("{:0width$}", counter, width = width),
            None => counter.to_string(),
        };

        Ok(result.replace("{counter}", &counter_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_basic_tokens() {
        let meta = FileMetadata::new(
            PathBuf::from("test.txt"),
            1024,
            "abcdef123456".into(),
            DateTime::parse_from_rfc3339("2026-03-02T12:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        );

        let result = TemplateEngine::render("{year}/{month}/{name}", &meta).unwrap();
        assert_eq!(result, "2026/03/test.txt");
    }

    #[test]
    fn detects_path_traversal() {
        let meta = FileMetadata::new(
            PathBuf::from("test.txt"),
            1024,
            "hash".into(),
            Utc::now(),
        );

        let result = TemplateEngine::render("../../../etc/passwd", &meta);
        assert!(result.is_err());
    }
}
```

**Validation:**
```bash
cargo test -p dirctl-core template
```

---

#### 3.3 Add Config Tests with Fixtures

**File:** `dirctl-core/tests/config_fixtures.rs`

**Action:**
```rust
use dirctl_core::config::DirctlConfig;
use std::path::PathBuf;

#[test]
fn test_full_config() {
    let yaml = r#"
version: 1
workspace: "."
default_conflict_policy: rename_with_suffix
ignore_patterns:
  - ".git/"
  - "node_modules/"
  - "*.tmp"

rules:
  - name: "archive-old-pdfs"
    description: "Move PDFs untouched for 30+ days"
    enabled: true
    priority: 10
    match_all: false
    conflict_policy: skip
    conditions:
      - field: extension
        operator: eq
        value: "pdf"
      - field: age_days
        operator: gte
        value: 30
    actions:
      - type: move
        to: "./Archives/PDFs/{year}/{month}/"

  - name: "normalize-screenshots"
    enabled: true
    conditions:
      - field: name
        operator: matches
        value: "^[Ss]creenshot.+\\.png$"
    actions:
      - type: rename
        pattern: "{year}-{month}-{day}_{hash_short}.{ext}"
      - type: move
        to: "./Screenshots/{year}-{month}/"
"#;

    let config: DirctlConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.version, 1);
    assert_eq!(config.rules.len(), 2);
    assert_eq!(config.rules[0].name, "archive-old-pdfs");
}

#[test]
fn test_default_values() {
    let yaml = r#"
version: 1
workspace: "."
rules: []
"#;

    let config: DirctlConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.ignore_patterns.len(), 3); // Has defaults
}
```

**Validation:**
```bash
cargo test -p dirctl-core --test config_fixtures
```

---

#### 3.4 Create Example Config Files

**Files:**
- `examples/basic-config.yaml`
- `examples/advanced-config.yaml`

**Action:**
```yaml
# examples/basic-config.yaml
version: 1
workspace: "."
default_conflict_policy: skip

rules:
  - name: "archive-old-pdfs"
    description: "Move old PDFs to archive"
    enabled: true
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

  - name: "organize-images"
    description: "Organize images by type"
    enabled: true
    conditions:
      - field: extension
        operator: in
        value: ["jpg", "jpeg", "png", "gif", "webp"]
    actions:
      - type: move
        to: "./Images/{extension}/"
```

```yaml
# examples/advanced-config.yaml
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
    priority: 10
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
    description: "Tag files over 2GB"
    enabled: true
    priority: 5
    conditions:
      - field: size
        operator: gte
        value: 2147483648  # 2GB
    actions:
      - type: tag
        tags: ["huge", "needs-review"]

  - name: "normalize-screenshots"
    description: "Standardize screenshot filenames"
    enabled: true
    priority: 0
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
    description: "Quarantine old log files"
    enabled: false
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

---

#### 3.5-3.9 (Remaining Config Tasks)

Due to length, I'll summarize the remaining Phase 3 tasks:

- 3.5: Add config migration support for future schema versions
- 3.6: Add config validation for path templates
- 3.7: Add config merge functionality (user + system configs)
- 3.8: Add config watch/reload detection
- 3.9: Add comprehensive error messages for config failures

**Phase 3 Complete When:**
- ✅ Config parses from YAML
- ✅ All rules validate correctly
- ✅ Templates render safely
- ✅ Example configs provided

---

## Phase 4: Scanner & Metadata

**Goal:** Implement filesystem scanning with metadata extraction.

**Success Criteria:**
- ✅ Scanner walks directories correctly
- ✅ Metadata extracted accurately
- ✅ Results returned in deterministic order
- ✅ Ignore patterns respected

### Tasks

#### 4.1 Implement Local Filesystem Scanner

**File:** `dirctl-fs-local/src/scanner.rs`

**Action:**
```rust
use std::path::{Path, PathBuf};
use async_trait::async_trait;
use tokio::fs;
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};
use dirctl_core::{ports::FileSystemScanner, domain::{FileMetadata, Workspace}};
use ignore::WalkBuilder;
use tracing::{debug, trace};

pub struct LocalFileSystemScanner {
    max_depth: Option<usize>,
}

impl LocalFileSystemScanner {
    pub fn new() -> Self {
        Self { max_depth: None }
    }

    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    async fn extract_metadata(&self, path: &Path) -> FileMetadata {
        let metadata = fs::metadata(path).await.unwrap();
        let modified = fs::metadata(path).await.unwrap()
            .modified()
            .unwrap()
            .into();

        let hash = self.hash_file(path).await.unwrap_or_else(|_| "unknown".into());

        FileMetadata::new(
            path.to_path_buf(),
            metadata.len(),
            hash,
            DateTime::from(modified),
        )
    }

    async fn hash_file(&self, path: &Path) -> dirctl_core::Result<String> {
        let content = fs::read(path).await
            .map_err(|e| dirctl_core::DirctlError::PortIo(
                format!("Failed to read {}: {}", path.display(), e)
            ))?;

        let mut hasher = Sha256::new();
        hasher.update(&content);
        let result = hasher.finalize();

        Ok(format!("{:x}", result))
    }
}
```

---

#### 4.2-4.10 (Remaining Scanner Tasks)

Due to length constraints, here are the remaining Phase 4 tasks:

- 4.2: Implement parallel scanning with rayon
- 4.3: Add MIME type detection
- 4.4: Implement ignore pattern support
- 4.5: Ensure deterministic ordering
- 4.6: Add error handling for permission errors
- 4.7: Add progress reporting
- 4.8: Add caching layer
- 4.9: Implement change detection
- 4.10: Add scanner tests with fixtures

**Phase 4 Complete When:**
- ✅ Scanner walks directories
- ✅ Metadata extracted
- ✅ Results ordered deterministically
- ✅ Ignore patterns work

---

## Phase 5: Rule Engine

**Goal:** Implement rule evaluation and matching logic.

**Success Criteria:**
- ✅ Conditions evaluate correctly
- ✅ All operators work
- ✅ Rules match in priority order
- ✅ match_all logic works

### Tasks

#### 5.1 Implement Condition Evaluator

**File:** `dirctl-core/src/evaluator.rs`

**Key Functions:**
```rust
pub struct ConditionEvaluator;

impl ConditionEvaluator {
    pub fn evaluate(
        condition: &Condition,
        metadata: &FileMetadata,
        now: DateTime<Utc>,
    ) -> bool {
        match (&condition.field, &condition.operator, &condition.value) {
            (ConditionField::Extension, ConditionOperator::Eq, ConditionValue::String(ext)) => {
                metadata.extension() == Some(ext.as_str())
            }
            (ConditionField::Size, ConditionOperator::Gte, ConditionValue::Integer(size)) => {
                metadata.size >= *size as u64
            }
            // ... handle all field/operator combinations
            _ => false,
        }
    }
}
```

---

#### 5.2-5.8 (Remaining Rule Engine Tasks)

- 5.2: Implement all string operators (eq, neq, contains, matches, etc.)
- 5.3: Implement all numeric operators (gt, lt, between, etc.)
- 5.4: Implement date operators (before, after)
- 5.5: Implement tag operators (has, not_has)
- 5.6: Add regex support with caching
- 5.7: Implement rule matching logic with priority
- 5.8: Add evaluator tests

**Phase 5 Complete When:**
- ✅ All operators work
- ✅ Rules evaluate correctly
- ✅ Priority ordering respected
- ✅ Tests pass

---

## Phase 6: Planner

**Goal:** Implement plan generation with deterministic guarantees.

**Success Criteria:**
- ✅ Plans generate deterministically
- ✅ Operations ordered correctly
- ✅ Conflict detection works
- ✅ Plans serialize/deserialize

### Tasks

#### 6.1 Define Plan Structure

**File:** `dirctl-core/src/planner/plan.rs`

```rust
/// A plan is a deterministic set of operations to execute
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plan {
    /// Files to process
    pub files: Vec<FileMetadata>,

    /// Generated operations (deterministically ordered)
    pub operations: Vec<Operation>,

    /// Conflicts detected
    pub conflicts: Vec<Conflict>,
}
```

---

#### 6.2-6.7 (Remaining Planner Tasks)

- 6.2: Implement plan generation algorithm
- 6.3: Add operation ordering logic
- 6.4: Implement conflict detection
- 6.5: Add plan validation
- 6.6: Add plan serialization
- 6.7: Add planner tests

**Phase 6 Complete When:**
- ✅ Plans generate deterministically
- ✅ Operations ordered by (priority, depth, path)
- ✅ Conflicts detected
- ✅ Golden tests pass

---

## Phase 7: Journal & Safety

**Goal:** Implement append-only journal with undo support.

**Success Criteria:**
- ✅ Journal writes before operations
- ✅ Transactions append-only
- ✅ Undo verifies hashes
- ✅ Journal corruption handled

### Tasks

#### 7.1 Implement Journal Format

**File:** `dirctl-core/src/journal/mod.rs`

```rust
pub struct Journal {
    path: PathBuf,
}

impl Journal {
    /// Append transaction to journal
    pub async fn append(&self, tx: &Transaction) -> Result<()> {
        // Write atomically
    }

    /// Read all transactions
    pub async fn read_all(&self) -> Result<Vec<Transaction>> {
        // Parse JSONL
    }

    /// Find transaction by ID
    pub async fn find_tx(&self, tx_id: &str) -> Result<Option<Transaction>> {
        // Search journal
    }
}
```

---

#### 7.2-7.11 (Remaining Journal Tasks)

- 7.2: Implement atomic line writes
- 7.3: Add journal validation
- 7.4: Implement undo transaction creation
- 7.5: Add hash verification for undo
- 7.6: Handle journal corruption
- 7.7: Add journal compaction
- 7.8: Implement quarantine tracking
- 7.9: Add journal recovery
- 7.10: Add journal tests
- 7.11: Add safety invariant tests

**Phase 7 Complete When:**
- ✅ Journal writes atomically
- ✅ Undo verifies hashes
- ✅ Corruption handled gracefully
- ✅ All safety tests pass

---

## Phase 8: Executor

**Goal:** Implement operation execution with journal integration.

**Success Criteria:**
- ✅ Operations execute correctly
- ✅ Journal written first
- ✅ Conflicts resolved
- ✅ Undo works

### Tasks

#### 8.1 Implement Local File Executor

**File:** `dirctl-fs-local/src/executor.rs`

```rust
pub struct LocalFileExecutor {
    journal: Arc<Journal>,
    workspace: PathBuf,
}

#[async_trait]
impl FileExecutor for LocalFileExecutor {
    async fn execute_operation(
        &self,
        operation: &Operation,
        conflict_policy: ConflictPolicy,
    ) -> Result<Operation> {
        // 1. Check for conflicts
        // 2. Execute operation
        // 3. Return result
    }
}
```

---

#### 8.2-8.9 (Remaining Executor Tasks)

- 8.2: Implement move operation
- 8.3: Implement copy operation
- 8.4: Implement delete with quarantine
- 8.5: Implement rename operation
- 8.6: Implement tag operation
- 8.7: Add conflict resolution
- 8.8: Implement undo logic
- 8.9: Add executor tests

**Phase 8 Complete When:**
- ✅ All operations work
- ✅ Conflicts resolved
- ✅ Undo functional
- ✅ Integration tests pass

---

## Phase 9: CLI Layer

**Goal:** Implement user-facing CLI with all commands.

**Success Criteria:**
- ✅ All commands work
- ✅ Output is clear
- ✅ Dry-run works
- ✅ Error messages helpful

### Tasks

#### 9.1 Define CLI Structure

**File:** `dirctl-cli/src/cli.rs`

```rust
#[derive(Parser, Debug)]
#[command(name = "dirctl")]
#[command(about = "Deterministic file organization engine", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Apply rules to workspace (dry-run by default)
    Apply {
        /// Workspace path
        #[arg(short, long)]
        workspace: PathBuf,

        /// Actually execute (default: dry-run)
        #[arg(long)]
        execute: bool,
    },

    /// Show planned changes without applying
    Plan {
        workspace: PathBuf,
    },

    /// Undo last transaction
    Undo {
        workspace: PathBuf,
        #[arg(long)]
        tx_id: Option<String>,
    },

    /// Show journal history
    Journal {
        workspace: PathBuf,
        #[arg(long)]
        limit: Option<usize>,
    },

    /// Validate configuration
    Validate {
        workspace: PathBuf,
    },

    /// Initialize new workspace
    Init {
        workspace: PathBuf,
    },
}
```

---

#### 9.2-9.8 (Remaining CLI Tasks)

- 9.2: Implement apply command
- 9.3: Implement plan command
- 9.4: Implement undo command
- 9.5: Implement journal command
- 9.6: Implement validate command
- 9.7: Implement init command
- 9.8: Add output formatting

**Phase 9 Complete When:**
- ✅ All commands work
- ✅ Help text clear
- ✅ Errors actionable
- ✅ Manual tested

---

## Phase 10: Testing & Validation

**Goal:** Comprehensive test coverage with golden fixtures.

**Success Criteria:**
- ✅ Unit tests pass
- ✅ Integration tests pass
- ✅ Golden fixtures validate
- ✅ Performance benchmarks meet targets

### Tasks

#### 10.1-10.10 (Testing Tasks)

- 10.1: Add unit tests for all modules
- 10.2: Create golden fixture tests
- 10.3: Add integration tests
- 10.4: Create performance benchmarks
- 10.5: Add property-based tests
- 10.6: Test edge cases
- 10.7: Test error conditions
- 10.8: Test cross-platform behavior
- 10.9: Add fuzz testing
- 10.10: Validate all requirements

**Phase 10 Complete When:**
- ✅ Test coverage >80%
- ✅ All requirements validated
- ✅ Performance targets met
- ✅ No known bugs

---

## Phase 11: Documentation & Release

**Goal:** Complete documentation and prepare v0.1.0 release.

**Success Criteria:**
- ✅ User documentation complete
- ✅ API documentation complete
- ✅ Examples provided
- ✅ Release prepared

### Tasks

#### 11.1-11.6 (Documentation Tasks)

- 11.1: Write user guide
- 11.2: Write configuration reference
- 11.3: Add code examples
- 11.4: Write contributor guide
- 11.5: Prepare changelog
- 11.6: Create release

**Phase 11 Complete When:**
- ✅ Documentation comprehensive
- ✅ Examples working
- ✅ Release tagged
- ✅ Announcement ready

---

## Risk Matrix

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Performance targets not met | High | Medium | Prototype scanner early, use rayon for parallelism |
| Cross-platform issues | High | Medium | Test on all platforms in CI, use portable libraries |
| Undetected bugs in core logic | High | Low | Comprehensive testing, golden fixtures, property-based tests |
| Schema changes break compatibility | Medium | Low | Version field in config, migration support |
| Undo logic fails catastrophically | High | Low | Hash verification, extensive testing, quarantine default |
| Scanner gets stuck on large trees | Medium | Medium | Progress reporting, cancellation support, depth limits |

---

## Progress Tracking

### Overall Progress: 5/85 tasks (5.9%)

- [ ] Phase 1: Foundation (5/8 tasks) ✅ Tasks 1.1, 1.2, 1.3, 1.4, 1.5 completed
- [ ] Phase 2: Domain Layer (0/10 tasks)
- [ ] Phase 3: Configuration (0/9 tasks)
- [ ] Phase 4: Scanner (0/10 tasks)
- [ ] Phase 5: Rule Engine (0/8 tasks)
- [ ] Phase 6: Planner (0/7 tasks)
- [ ] Phase 7: Journal (0/11 tasks)
- [ ] Phase 8: Executor (0/9 tasks)
- [ ] Phase 9: CLI (0/8 tasks)
- [ ] Phase 10: Testing (0/10 tasks)
- [ ] Phase 11: Documentation (0/6 tasks)

### Next Steps

1. ✅ **Phase 1, Task 1.1**: Initialize Cargo Workspace - COMPLETED
2. ✅ **Phase 1, Task 1.2**: Create Core Crate Skeleton - COMPLETED
3. ✅ **Phase 1, Task 1.3**: Create Filesystem Adapter Crate - COMPLETED
4. ✅ **Phase 1, Task 1.4**: Create CLI Crate Skeleton - COMPLETED
5. ✅ **Phase 1, Task 1.5**: Define Core Error Types - COMPLETED
6. **Next**: Phase 1, Task 1.6: Setup Testing Infrastructure
7. Mark tasks complete as you go
8. Run validation after each phase
9. Update progress tracking

---

## Notes

- This plan is a guide - adapt as needed
- Each task should take 2-4 hours
- If a task takes longer, break it down
- Always run tests before moving on
- Ask questions if stuck
- Focus on completing one phase at a time

**Remember:** The architecture is your constitution. When in doubt, refer back to the architecture-authority skill principles.

---

*This implementation plan was generated with architecture-authority and prompt-improver skills.*
