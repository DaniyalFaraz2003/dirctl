//! Core error types for dirctl
//!
//! All errors in the core crate MUST use these types.
//! No unwrap() or expect() allowed in core code.

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
        actual: String,
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

/// Result type alias for dirctl
///
/// Use this instead of std::result::Result for all core functions
pub type Result<T> = std::result::Result<T, DirctlError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_displays_correctly() {
        let err = DirctlError::Config("test error".to_string());
        assert_eq!(err.to_string(), "Configuration error: test error");
    }

    #[test]
    fn result_type_works() {
        fn returns_ok() -> Result<String> {
            Ok("success".to_string())
        }

        fn returns_err() -> Result<String> {
            Err(DirctlError::Config("failed".to_string()))
        }

        assert!(returns_ok().is_ok());
        assert!(returns_err().is_err());
    }
}
