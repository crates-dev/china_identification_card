//! china_identification_card
//!
//! A Rust library for validating Chinese identification card numbers based on official rules and checksums.

#[cfg(test)]
mod test;

pub(crate) mod r#const;
pub(crate) mod r#impl;
pub(crate) mod r#struct;

pub use r#struct::*;

pub(crate) use r#const::*;
