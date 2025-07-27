use crate::*;

/// Implementation of validation methods for Chinese ID card numbers.
impl ChineseIdCard {
    /// Validates whether a given string represents a valid Chinese ID card number.
    ///
    /// # Arguments
    ///
    /// - `id_number` - A value that can be converted to string representing the ID number.
    ///
    /// # Returns
    ///
    /// - `bool` - Returns true if the ID number is valid, false otherwise.
    pub fn is_valid_id_number<T: ToString>(id_number: T) -> bool {
        let id_number_string: String = id_number.to_string();
        if id_number_string.len() != 18 || !id_number_string[..17].chars().all(|c| c.is_digit(10)) {
            return false;
        }
        let last_char: char = id_number_string.chars().last().unwrap();
        if !(last_char.is_digit(10) || last_char == 'X') {
            return false;
        }
        let sum: i32 = id_number_string[..17]
            .chars()
            .zip(WEIGHTS.iter())
            .map(|(c, &w)| c.to_digit(10).unwrap() as i32 * w)
            .sum();
        let check_code: char = CHECK_CODES[(sum % 11) as usize];
        check_code == last_char
    }

    /// Checks whether a given string represents an invalid Chinese ID card number.
    ///
    /// # Arguments
    ///
    /// - `id_number` - A value that can be converted to string representing the ID number.
    ///
    /// # Returns
    ///
    /// - `bool` - Returns true if the ID number is invalid, false otherwise.
    pub fn is_invalid_id_number<T: ToString>(id_number: T) -> bool {
        !Self::is_valid_id_number(id_number)
    }
}
