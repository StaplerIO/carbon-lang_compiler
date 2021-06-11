use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::token::{OperatorType, ContainerType};
use crate::shared::token::TokenType::Container;

// Return -1 if there's no semicolon token
pub fn find_next_semicolon(tokens: Vec<DecoratedToken>) -> isize {
    for (index, token) in tokens.iter().enumerate() {
        if token.token_type == DecoratedTokenType::StatementEndSign {
            return index as isize;
        }
    }

    return -1;
}

// Return -1 if there's no semicolon token
pub fn find_next_comma(tokens: Vec<DecoratedToken>) -> isize {
    for (index, token) in tokens.iter().enumerate() {
        if token.token_type == DecoratedTokenType::Operator {
            if token.operator.unwrap().operator_type == OperatorType::Comma {
                return index as isize;
            }
        }
    }

    return -1;
}

pub fn split_comma_expression(tokens: Vec<DecoratedToken>) -> Vec<Vec<DecoratedToken>> {
    let mut result: Vec<Vec<DecoratedToken>> = Vec::new();

    // Initialize result by an empty Vec
    result.push(vec![]);
    for token in tokens {
        if token.token_type == DecoratedTokenType::Operator {
            if token.operator.unwrap().operator_type == OperatorType::Comma {
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

    let container_type: Option<ContainerType> = None;
    let anti_type: Option<ContainerType> = None;
    for (index, token) in tokens.iter().enumerate() {
        if token.token_type == DecoratedTokenType::Container {
            if index == 0 {
                container_type == token.container.clone();
                anti_type == match container_type.unwrap() {
                    ContainerType::Brace => Option::from(ContainerType::AntiBrace),
                    ContainerType::Bracket => Option::from(ContainerType::AntiBracket),
                    ContainerType::Index => Option::from(ContainerType::AntiIndex),
                    _ => Option::from(ContainerType::Unset)
                };
            }

            if token.container == container_type {
                container_level += 1;
            } else if token.container == anti_type {
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
