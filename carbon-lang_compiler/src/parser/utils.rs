use crate::shared::token::{TokenType, Token};

// Return -1 if there's no semicolon token
pub fn find_next_semicolon(tokens: Vec<Token>) -> isize {
    for (index, token) in tokens.iter().enumerate() {
        if token.token_type == TokenType::Semicolon {
            return index as isize;
        }
    }

    return -1;
}
