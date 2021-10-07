use crate::apa::addition::add;
use crate::apa::utils::reverse_string;

pub fn multiply(mut multiplier: String, mut multiplicand: String) -> String {
    // reverse multiplicand and multiplier, make them 0-index aligned
    // Make them easier to calculate
    multiplier = reverse_string(multiplier);
    multiplicand = reverse_string(multiplicand);

    let mut result = String::from("0");

    for (mpd_offset,mpd_digit) in multiplicand.chars().enumerate() {
        let mut current_iteration_result = String::new();

        for (mlp_offset, mlp_digit) in multiplier.chars().enumerate() {
            let current_digit_result: u8 = (mpd_digit as u8 - '0' as u8) * (mlp_digit as u8 - '0' as u8);
            let mut t_str = current_digit_result.to_string();
            t_str = format!("{}{}", t_str, std::iter::repeat("0").take(mlp_offset).collect::<String>());

            current_iteration_result = add(current_iteration_result.clone(), t_str);
        }

        current_iteration_result = format!("{}{}",current_iteration_result, std::iter::repeat("0").take(mpd_offset).collect::<String>());

        result = add(result, current_iteration_result);
    }

    return result;
}
