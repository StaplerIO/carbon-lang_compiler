use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /*
    static ref NUMBER_RULES: Vec<(Regex, &'static str)> = vec![
        (Regex::new(r"^0x[0-9a-fA-F]+").unwrap(), "hex"),
        (Regex::new(r"^0b[01]+").unwrap(), "bin"),
        (Regex::new(r"^0o[0-7]+").unwrap(), "oct"),
        (Regex::new(r"^0[0-7]+").unwrap(), "oct"),
        (Regex::new(r"^[0-9]+").unwrap(), "dec"),
    ];
    */

    static ref NUMBER_REGEX: Regex = Regex::new("^([+-]?\\d+(\\.?\\d+)?)[\\s\\S]*").unwrap();
}

pub fn match_number(content: &str) -> String {
    let captures = NUMBER_REGEX.captures(content);
    if captures.is_some() {
        return captures.unwrap()[1].to_string();
    }

    return String::new();
}
