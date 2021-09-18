pub fn match_semicolon(content: &str) -> bool {
    return content.chars().nth(0).unwrap() == ';';
}
