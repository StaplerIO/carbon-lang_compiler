use crate::apa::division::divide;
use crate::apa::multiplication::multiply;
use crate::apa::subtraction::subtract;

pub fn modulo(dividend: String, divisor: String) -> String {
    let integer_result = divide(dividend.clone(), divisor.clone(), 0);
    let remainder = subtract(dividend, multiply(integer_result, divisor));
    return remainder.trim_start_matches(|x| x == '0').parse().unwrap();
}
