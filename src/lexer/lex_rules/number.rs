pub fn match_number(content: &str) -> String {
    let mut result = String::new();

    let mut is_dot_exist: bool = false;
    for (i, c) in content.chars().into_iter().enumerate() {
        if c.is_ascii_digit() {
            result.push(c);
        } else if c == '.' && !is_dot_exist {
            // If next character is a digit, then this is a decimal
            if content.chars().nth(i + 1).unwrap().is_ascii_digit() {
                result.push(c);
                is_dot_exist = true;
                continue;
            }

            break;
        } else {
            break;
        }
    }

    return result;
}
