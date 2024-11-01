use crate::*;
#[test]
fn test_is_valid_id_number() {
    let valid: bool = is_valid_id_number("110101202311012176");
    assert_eq!(valid, true);
    let unvalid: bool = is_valid_id_number("110101202311012171");
    assert_eq!(unvalid, false);
}
