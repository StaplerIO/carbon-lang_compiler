use crate::shared::token::ContainerType;
use std::borrow::Borrow;

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn is_letter(c: char) -> bool {
    return (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z');
}

pub fn match_identifier(content: String) -> String {
    let mut result = String::new();

    // First character cannot be a number
    if content.chars().nth(0).unwrap() == '_' || is_letter(content.chars().nth(0).unwrap()) {
        result.push(content.chars().nth(0).unwrap());

        let mut clone = content.clone();
        clone.remove(0);
        // Other characters can be a letter or a digit or an underscore
        for c in clone.chars() {
            if is_digit(c) || is_letter(c) || c == '_' {
                result.push(c);
            } else {
                break;
            }
        }
    }

    return result;
}

pub fn match_number(content: String) -> String {
    let mut result = String::new();

    let mut is_dot_exist: bool = false;
    for (i, c) in content.chars().into_iter().enumerate() {
        if is_digit(c) {
            result.push(c);
        } else if c == '.' && !is_dot_exist {
            // If next character is a digit, means this is a decimal
            if is_digit(content.chars().nth(i + 1).unwrap()) {
                result.push(c);
                is_dot_exist = true;
                continue;
            }

            break;
        } else {
            break;
        }
    }

    return result;
}

pub fn match_spaces(content: String) -> String {
    let mut result = String::new();

    for c in content.chars() {
        if c == ' ' || c == '\n' || c == '\r' || c == '\t' {
            result.push(c);
        } else {
            // End sequence
            break;
        }
    }

    return result;
}

pub fn match_container(content: String) -> ContainerType {
    return match content.chars().nth(0).unwrap() {
        '{' => ContainerType::Brace,
        '}' => ContainerType::AntiBrace,
        '[' => ContainerType::Index,
        ']' => ContainerType::AntiIndex,
        '(' => ContainerType::Bracket,
        ')' => ContainerType::AntiBracket,
        _ => ContainerType::Unset
    };
}
