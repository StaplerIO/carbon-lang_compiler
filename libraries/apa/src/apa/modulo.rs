use crate::apa::division::divide;
use crate::apa::multiplication::multiply;
use crate::apa::subtraction::subtract;

/// # Return value:
/// The first element in return value tuple is the quotient
///
/// The last element in return value is the real remainder
pub fn modulo(dividend: String, divisor: String) -> (String, String) {
    let integer_result = divide(dividend.clone(), divisor.clone(), 0);
    let remainder = subtract(dividend, multiply(integer_result.clone(), divisor));
    return (
        integer_result,
        remainder.trim_start_matches(|x| x == '0').parse().unwrap(),
    );
}
