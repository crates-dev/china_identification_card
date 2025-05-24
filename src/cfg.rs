#[test]
fn test_is_valid_id_number() {
    use crate::*;

    let valid: bool = ChineseIdCard::is_valid_id_number("110101202311012176");
    assert_eq!(valid, true);
    let un_valid: bool = ChineseIdCard::is_invalid_id_number("110101202311012171");
    assert_eq!(un_valid, true);
}
