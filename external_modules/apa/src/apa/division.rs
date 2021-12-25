use crate::apa::multiplication::multiply;
use crate::apa::subtraction::subtract;
use crate::apa::utils::{compare_str_number, remove_more_zeros};

pub fn divide(dividend: String, divisor: String, precision: usize) -> String {
    let mut result = String::from("0");

    let mut cached_number = String::new();
    for digit in dividend.chars() {
        // Remove useless zeros in the front of number
        result = remove_more_zeros(&result);
        cached_number = remove_more_zeros(&cached_number);

        // Send current digit to the number waiting to divide
        cached_number.push(digit);

        // If the digit count in cached_number is less than divisor, it can't be divided.
        if cached_number.len() < divisor.len() {
            result.push('0');
            continue;
        }

        let mut prev_res = String::from("0");
        let mut quotient: usize = 0;
        while quotient <= 10 {
            let current_res = multiply(divisor.clone(), quotient.to_string());
            cached_number = remove_more_zeros(&cached_number);

            if compare_str_number(&current_res, &cached_number) > 0 {
                cached_number = subtract(cached_number, prev_res);
                result.push(((quotient - 1) as u8 + '0' as u8) as char);
                break;
            }

            prev_res = current_res;
            quotient += 1;
        }
    }

    result = remove_more_zeros(&result);
    return result;
}
