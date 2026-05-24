use crate::*;

#[test]
fn test_is_valid_id_number() {
    let valid: bool = ChineseIdCard::is_valid_id_number("110101202311012176");
    assert!(valid);
    let un_valid: bool = ChineseIdCard::is_invalid_id_number("110101202311012171");
    assert!(un_valid);
}
