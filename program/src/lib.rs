//! Crate defining an example program for creating TPL token groups
//! using the TPL Token Group interface.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
