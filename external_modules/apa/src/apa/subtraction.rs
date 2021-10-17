use std::cmp::max;
use crate::apa::utils::reverse_string;

// All terms must be positive for now and minuend > subtrahend
pub fn subtract(mut minuend: String, mut subtrahend: String) -> String {
    // Reverse two numbers, make them 0-index aligned
    minuend = reverse_string(minuend);
    subtrahend = reverse_string(subtrahend);

    let mut result = String::new();

    let max_length = max(minuend.len(), subtrahend.len());
    let mut borrow_flag = false;

    for index in 0..max_length {
        let ta: u8 = if index < minuend.len() { minuend.chars().nth(index).unwrap() as u8 - '0' as u8 } else { 0 };
        let tb: u8 = if index < subtrahend.len() { subtrahend.chars().nth(index).unwrap() as u8 - '0' as u8 } else { 0 };

        // If previous calculation is less than 0, then minus 1 to this iteration
        let mut single_digit_result: i8 = (ta - tb) as i8;
        if borrow_flag { single_digit_result -= 1; }
        borrow_flag = false;

        if single_digit_result < 0 {
            borrow_flag = true;
            single_digit_result += 10;
        }

        result.push((single_digit_result as u8 + '0' as u8) as char);
    }

    result = reverse_string(result);

    return result;
}
