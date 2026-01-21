//! china_identification_card
//!
//! A Rust library for validating Chinese identification card numbers based on official rules and checksums.

pub(crate) mod r#const;
pub(crate) mod r#impl;
pub(crate) mod r#struct;

#[cfg(test)]
mod test;

pub use r#struct::*;

use r#const::*;
