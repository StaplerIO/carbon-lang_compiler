use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^([_a-zA-Z][_a-zA-Z0-9]{0,80})[\\s\\S]*").unwrap();
}

pub fn match_identifier(content: &str) -> String {
    let captures = IDENTIFIER_REGEX.captures(content);
    if captures.is_some() {
        return captures.unwrap()[1].to_string();
    }

    return String::new();
}
