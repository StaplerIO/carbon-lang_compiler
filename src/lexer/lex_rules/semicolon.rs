use lazy_static::lazy_static;
lazy_static! {
    static ref SEMICOLON_TOKEN: char = ';';
}

pub fn match_semicolon(content: &str) -> bool {
    return content.chars().nth(0).unwrap() == *SEMICOLON_TOKEN;
}
