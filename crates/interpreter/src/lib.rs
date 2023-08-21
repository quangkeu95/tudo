//! Tudo interpreter converts the workflow files into a set of workflows

mod interpreter;
pub use interpreter::*;
pub mod alloy_converter;
pub mod step;
pub mod types;
