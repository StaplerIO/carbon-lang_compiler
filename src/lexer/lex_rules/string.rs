use crate::shared::token::token::{Token, TokenContent};
use crate::shared::utils::position::Position;

pub fn match_string(content: &str, base_pos: usize) -> Token {
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

    return if result.is_empty() {
        Token::new_invalid()
    } else {
        // Remove quotes
        let bare_string = result
            .trim_start_matches("\"")
            .trim_end_matches("\"")
            .to_string();
        Token::new(TokenContent::String(bare_string), Position::new(base_pos, result.len() + 2))
    }
}

// TODO: Convert escape characters
