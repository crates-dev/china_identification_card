#![allow(warnings)]

mod cfg;

/// Checks if the given Chinese ID number is valid.
///
/// # Arguments
/// - `id_number`: The ID number to validate.
///
/// # Returns
/// A boolean value indicating validity. Returns `true` if valid, `false` otherwise.
///
/// # Examples
/// ```
/// use china_identification_card::*;
/// let is_valid: bool = is_valid_id_number("110101202311012176");
/// ```
#[inline]
pub fn is_valid_id_number(id_number: &str) -> bool {
    // Ensure the length is 18, first 17 characters are digits, and the 18th is a digit or 'X'
    if id_number.len() != 18 || !id_number[..17].chars().all(|c| c.is_digit(10)) {
        return false;
    }
    let last_char: char = id_number.chars().last().unwrap();
    if !(last_char.is_digit(10) || last_char == 'X') {
        return false;
    }
    // Weight factors and checksum table
    const WEIGHTS: [i32; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    const CHECK_CODES: [char; 11] = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];
    // Calculate the checksum
    let sum: i32 = id_number[..17]
        .chars()
        .zip(WEIGHTS.iter())
        .map(|(c, &w)| c.to_digit(10).unwrap() as i32 * w)
        .sum();
    let check_code: char = CHECK_CODES[(sum % 11) as usize];
    check_code == last_char
}
