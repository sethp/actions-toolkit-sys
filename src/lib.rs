//! Raw bindings to the GitHub actions toolkit API for projects using wasm-bindgen.

#![deny(clippy::all)]
#![deny(missing_docs)]

/// Core functions for setting results, logging, registering secrets and exporting variables.
pub mod core;
/// Execute your tools on the command line in a cross-platform way.
pub mod exec;
/// Core functions for CLI filesystem scenarios.
pub mod io;
