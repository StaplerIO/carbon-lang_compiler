pub fn match_semicolon(content: String) -> bool {
    return content.chars().nth(0).unwrap() == ';';
}
