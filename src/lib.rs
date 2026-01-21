//! china_identification_card
//!
//! A Rust library for validating Chinese identification card numbers based on official rules and checksums.

mod r#const;
mod r#impl;
mod r#struct;
#[cfg(test)]
mod test;

pub use r#struct::*;

use r#const::*;
