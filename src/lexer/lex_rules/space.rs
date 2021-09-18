pub fn match_spaces(content: &str) -> String {
    let mut result = String::new();

    for c in content.chars() {
        if c == ' ' || c == '\n' || c == '\r' || c == '\t' {
            result.push(c);
        } else {
            // End sequence
            break;
        }
    }

    return result;
}

