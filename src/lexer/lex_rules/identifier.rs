pub fn match_identifier(content: &str) -> String {
    let mut result = String::new();

    // First character cannot be a number
    if content.chars().nth(0).unwrap() == '_' || content.chars().nth(0).unwrap().is_ascii_alphabetic() {
        result.push(content.chars().nth(0).unwrap());

        let mut clone = content.to_string();
        clone.remove(0);
        // Other characters can be a letter or a digit or an underscore
        for c in clone.chars() {
            if c.is_ascii_digit() || c.is_ascii_alphabetic() || c == '_' {
                result.push(c);
            } else {
                break;
            }
        }
    }

    return result;
}
