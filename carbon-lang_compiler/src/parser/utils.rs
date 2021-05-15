use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::token::OperatorType;

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
    let mut result: Vec<Vec<DecoratedToken>> = vec![];

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
