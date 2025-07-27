<center>

# china_identification_card

[![](https://img.shields.io/crates/v/china_identification_card.svg)](https://crates.io/crates/china_identification_card)
[![](https://img.shields.io/crates/d/china_identification_card.svg)](https://img.shields.io/crates/d/china_identification_card.svg)
[![](https://docs.rs/china_identification_card/badge.svg)](https://docs.rs/china_identification_card)
[![](https://github.com/eastspire/china_identification_card/workflows/Rust/badge.svg)](https://github.com/eastspire/china_identification_card/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/china_identification_card.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/CHINA_IDENTIFICATION_CARD/)

> A Rust library for validating Chinese identification card numbers based on official rules and checksums.

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

let valid: bool = ChineseIdCard::is_valid_id_number("110101202311012176");
assert_eq!(valid, true);
let un_valid: bool = ChineseIdCard::is_invalid_id_number("110101202311012171");
assert_eq!(un_valid, true);
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
