# China Identification Card

A Rust library for validating Chinese identification card numbers based on official rules and checksums.

## Features

- Validates the length and format of a Chinese identification card number
- Calculates and verifies the checksum according to official weight factors
- Lightweight and easy to integrate

## Installation

To use this crate, you can run cmd:

```shell
cargo add china_identification_card
```

## Examples

```rust
    use china_identification_card::*;
    let is_valid: bool = is_valid_id_number("110101202311012176");
```
