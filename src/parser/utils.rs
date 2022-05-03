use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenContent};
use crate::shared::token::container::ContainerType;
use crate::shared::token::operator::Operator;

// Return the distance to next semicolon token, None to find nothing
pub fn find_next_semicolon(tokens: Vec<DecoratedToken>) -> Option<usize> {
    return tokens
        .iter()
        .position(|t| t.content == DecoratedTokenContent::StatementEndSign);
}

// Return the distance to next comma token, None to find nothing
pub fn find_next_comma(tokens: Vec<DecoratedToken>) -> Option<usize> {
    return tokens.iter().position(|t| {
        t.content.get_operator().is_some()
            && *t.content.get_operator().unwrap_or(&Operator::Invalid) == Operator::Comma
    });
}

pub fn split_comma_expression(tokens: Vec<DecoratedToken>) -> Vec<Vec<DecoratedToken>> {
    let mut result: Vec<Vec<DecoratedToken>> = Vec::new();

    // Initialize result by an empty Vec
    result.push(vec![]);
    for token in tokens {
        if token.content.get_operator().is_some() {
            if *token.content.get_operator().unwrap() == Operator::Comma {
                // Start an new expression
                result.push(vec![]);
                continue;
            }
        }

        // Add new expression term
        let mut current_expression = result.last().unwrap().to_vec();
        current_expression.push(token.clone());
        result.remove(result.len() - 1);
        result.push(current_expression.clone());
    }

    return result;
}

// Ensure the first token is a container before calling this function, or it will return immediately
pub fn pair_container(tokens: Vec<DecoratedToken>) -> Vec<DecoratedToken> {
    let mut container_level = 0;

    let mut container_type = ContainerType::Invalid;
    let mut anti_type = ContainerType::Invalid;
    for (index, token) in tokens.iter().enumerate() {
        if token.content.get_container().is_some() {
            if index == 0 {
                container_type = *token.content.get_container().unwrap();
                anti_type = match container_type {
                    ContainerType::Brace => ContainerType::AntiBrace,
                    ContainerType::Bracket => ContainerType::AntiBracket,
                    ContainerType::Index => ContainerType::AntiIndex,
                    _ => ContainerType::Invalid,
                };
            }

            if *token.content.get_container().unwrap() == container_type {
                container_level += 1;
            } else if *token.content.get_container().unwrap() == anti_type {
                container_level -= 1;
            }
        }

        // top layer from token array start
        if container_level == 0 {
            return tokens[0..index].to_vec();
        }
    }

    return tokens;
}
