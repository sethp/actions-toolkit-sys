//! Raw bindings to the GitHub actions toolkit API for projects using wasm-bindgen.

#![deny(clippy::all)]
#![deny(missing_docs)]

/// Core functions for setting results, logging, registering secrets and exporting variables across
/// actions
pub mod core;
