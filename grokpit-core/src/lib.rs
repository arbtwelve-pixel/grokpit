//! GROKPIT Core Library
//!
//! Shared types and logic between the service binary and future components
//! (UI, plugins, tests, etc.).

pub mod policy;
pub mod task;
pub mod acp;
pub mod mcp;
pub mod http_server;

// Re-export key types
pub use policy::PolicyEngine;
pub use task::TaskRegistry;