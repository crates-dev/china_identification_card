//! china_identification_card
//!
//! A Rust library for validating Chinese identification card numbers based on official rules and checksums.

mod cfg;

pub(crate) mod chinese_id_card;

pub use chinese_id_card::*;
