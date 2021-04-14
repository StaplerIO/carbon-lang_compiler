use crate::shared::token::{Token, TokenType, OperatorType, ContainerType};
use crate::shared::ast::blocks::expression::{Expression, Term, TermType};
use crate::shared::token::TokenType::{Operator, Container};

/**
 * Parameters:
 * tokens: token serial to build a complete expression, returns nothing when encountered an illegal token
 */
pub fn build_expression(tokens: Vec<Token>) {
    let mut result: Expression = Expression{
        is_single_term: false,
        is_left_nested_expr: false,
        is_right_nested_expr: false,
        left_expr: Box::new(None),
        right_expr: Box::new(None),
        left_term: None,
        right_term: None,
        single_term: None
    };
    let mut index: usize = 0;

    let mut is_operator_required = false;

    // false means left, true means right
    let mut expression_position = false;
    loop {
        let token: &Token = tokens.get(index).unwrap();

        if is_operator_required {
            if token.token_type == TokenType::Operator {}
        } else {
            // We require the token is an identifier or a number, others are the invalid token of the expression
            if token.token_type == TokenType::Identifier || token.token_type == TokenType::Number {
                let mut term = Term {
                    kind: TermType::Unset,
                    identifier: None,
                    number: None,
                    string: None,
                    marked_as_not: None,
                };

                match token.token_type {
                    TokenType::Identifier => {
                        term.kind = TermType::Identifier;
                        term.identifier = token.identifier.clone();
                    }
                    TokenType::Number => {
                        term.kind = TermType::Number;
                        term.number = token.number.clone();
                    }
                    _ => {}
                }

                if expression_position {
                    // Add it to the left side of the expression
                    result.is_left_nested_expr = false;
                    result.left_term = Option::from(term);
                } else {
                    // Add it to the right side of the expression
                    result.is_right_nested_expr = false;
                    result.right_term = Option::from(term);
                }
            }
        }

        index += 1;
    }
}

fn is_valid_expression_term(token_type: TokenType) -> bool {
    return token_type == TokenType::Identifier ||
        token_type == TokenType::Number;
}

fn is_valid_operator(token: Token) -> bool {
    if token.token_type == TokenType::Operator {
        let operator = token.operator.unwrap();

        return operator.operator_type == OperatorType::Calculation ||
            operator.operator_type == OperatorType::Relation ||
            operator.operator_type == OperatorType::Logical;
    } else if token.token_type == TokenType::Container {
        let container = token.container.unwrap();

        return container == ContainerType::Bracket ||
            container == ContainerType::AntiBracket;
    }

    return false;
}
