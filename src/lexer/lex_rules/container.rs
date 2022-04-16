use crate::shared::token::container::ContainerType;
use crate::shared::token::token::{Token, TokenContent};
use crate::shared::utils::position::Position;

// Convert character to token
pub fn match_container(content: &str, base_pos: usize) -> Token {
    let result = match content.chars().nth(0).unwrap() {
        '{' => ContainerType::Brace,
        '}' => ContainerType::AntiBrace,
        '[' => ContainerType::Index,
        ']' => ContainerType::AntiIndex,
        '(' => ContainerType::Bracket,
        ')' => ContainerType::AntiBracket,
        _ => ContainerType::Invalid,
    };

    return if result != ContainerType::Invalid {
        Token::new(TokenContent::Container(result), Position::new(base_pos, 1))
    } else {
        Token::new_invalid()
    };
}
