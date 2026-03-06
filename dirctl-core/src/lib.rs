//! dirctl-core
//!
//! Pure domain logic for the dirctl file organization engine.
//! This crate contains no I/O operations - it defines the core entities,
//! ports (traits), and business rules.

// Error types
pub mod error;

// Re-export error types for convenience
pub use error::{DirctlError, Result};

// This is a skeleton - more modules will be added in phase 2
