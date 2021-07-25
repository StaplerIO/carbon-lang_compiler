pub fn remove_comments(source_code: String) -> String {
    let mut is_single_quoted: bool = false;
    let mut is_double_quoted: bool = false;
    let mut is_commented: bool = false;

    let mut result = String::new();

    for c in source_code.chars() {
        if c == '\'' {
            is_single_quoted = !is_single_quoted;
        }
        if c == '\"' {
            is_double_quoted = !is_double_quoted;
        }

        if c == '#' && !is_single_quoted && !is_double_quoted {
            is_commented = !is_commented;
            continue;
        }

        if !is_commented {
            result.push(c);
        }
    }

    return result;
}
