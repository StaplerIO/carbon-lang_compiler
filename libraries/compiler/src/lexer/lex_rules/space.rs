use crate::shared::token::token::Token;
use crate::shared::token::token::TokenContent::Whitespace;
use crate::shared::utils::position::Position;

pub fn match_spaces(content: &str, base_pos: usize) -> Token {
    let mut result = String::new();

    for c in content.chars() {
        if c == ' ' || c == '\n' || c == '\r' || c == '\t' {
            result.push(c);
        } else {
            // End sequence
            break;
        }
    }

    return if result.is_empty() {
        Token::new_invalid()
    } else {
        Token::new(
            Whitespace(result.clone()),
            Position::new(base_pos, result.len()),
        )
    };
}
