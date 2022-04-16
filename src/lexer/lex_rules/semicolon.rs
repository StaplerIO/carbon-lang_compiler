use lazy_static::lazy_static;
use crate::shared::token::token::{Token, TokenContent};
use crate::shared::utils::position::Position;
lazy_static! {
    static ref SEMICOLON_TOKEN: char = ';';
}

pub fn match_semicolon(content: &str, base_pos: usize) -> Token {
    return if content.chars().nth(0).unwrap() == *SEMICOLON_TOKEN{
        Token::new(TokenContent::Semicolon, Position::new(base_pos, 1))
    }else {
        Token::new_invalid()
    };
}
