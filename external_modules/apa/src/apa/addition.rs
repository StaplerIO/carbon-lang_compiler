use std::cmp::max;
use std::ops::Index;
use crate::apa::utils::reverse_string;

pub fn add(mut augend: String, mut addend: String) -> String {
    augend = reverse_string(augend);
    addend = reverse_string(addend);

    let mut result = String::new();

    let max_length = max(augend.len(), addend.len());
    let mut carry_flag = false;

    for index in 0..max_length {
        let ta: u8 = if index < augend.len() { augend.chars().nth(index).unwrap() as u8 - '0' as u8 } else { 0 };
        let tb: u8 = if index < addend.len() { addend.chars().nth(index).unwrap() as u8 - '0' as u8 } else { 0 };

        // If previous calculation is bigger than 10, then add 1 to this iteration
        let mut single_digit_result = ta + tb;
        if carry_flag { single_digit_result += 1; }
        carry_flag = false;

        if single_digit_result >= 10 {
            carry_flag = true;
            single_digit_result -= 10;
        }

        result.push((single_digit_result + '0' as u8) as char);
    }

    // Add last carry flag
    if carry_flag { result.push('1'); }
    result = reverse_string(result);

    return result;
}
