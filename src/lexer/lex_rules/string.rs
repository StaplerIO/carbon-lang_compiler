pub fn match_string(content: String) -> String {
    let mut result = String::new();

    if content.starts_with('\"') {
        for ch in content[1..].chars() {
            if ch != '\"' {
                result.push(ch);
            } else {
                break;
            }
        }
    }

    return result;
}
