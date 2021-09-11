use crate::shared::token::ContainerType;

// Convert character to token
pub fn match_container(content: String) -> ContainerType {
    return match content.chars().nth(0).unwrap() {
        '{' => ContainerType::Brace,
        '}' => ContainerType::AntiBrace,
        '[' => ContainerType::Index,
        ']' => ContainerType::AntiIndex,
        '(' => ContainerType::Bracket,
        ')' => ContainerType::AntiBracket,
        _ => ContainerType::Unset,
    };
}
