#![doc = include_str!("../README.md")]

mod error;
mod macros;

// Re-export the main types and traits
// The macros are defined in the macros module and exported automatically

// Re-export commonly used anyhow types that don't conflict
pub use anyhow::Chain;
pub use error::{Context, Error, Result};
