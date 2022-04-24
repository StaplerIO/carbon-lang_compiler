use lazy_static::lazy_static;
use regex::Regex;
use crate::lexer::lex_rules::keyword::KEYWORDS;
use crate::shared::token::token::{Token, TokenContent};
use crate::shared::utils::position::Position;

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^([_a-zA-Z][_a-zA-Z0-9]{0,80})[\s\S]*").unwrap();
}

pub fn match_identifier(content: &str, base_pos: usize) -> Token {
    let captures = IDENTIFIER_REGEX.captures(content);
    if captures.is_some() {
        let identifier = captures.unwrap()[1].to_string();
        if !is_keyword(&identifier) {
            return Token::new(TokenContent::Identifier(identifier.clone()), Position::new(base_pos, identifier.len()))
        }
    }

    return Token::new_invalid();
}

fn is_keyword(identifier: &String) -> bool {
    for k_value in KEYWORDS.values() {
        if identifier.eq(k_value) {
            return true;
        }
    }

    return false;
}
