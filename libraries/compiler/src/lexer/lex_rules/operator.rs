use crate::shared::token::operator::{
    CalculationOperator, LogicalOperator, Operator, RelationOperator,
};
use crate::shared::token::token::{Token, TokenContent};
use crate::shared::utils::position::Position;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref ROOT_OPERATOR: HashMap<Operator, &'static str> = [
        (Operator::Assignment, "="),
        (Operator::Scope, "::"),
        (Operator::Comma, ","),
        (Operator::Dot, "."),
    ]
    .iter()
    .cloned()
    .collect();
    static ref CALCULATION_OPERATOR: HashMap<CalculationOperator, &'static str> = [
        (CalculationOperator::Addition, "+"),
        (CalculationOperator::Subtraction, "-"),
        (CalculationOperator::Multiply, "*"),
        (CalculationOperator::Division, "/"),
        (CalculationOperator::Modulo, "%"),
    ]
    .iter()
    .cloned()
    .collect();
    static ref LOGICAL_OPERATOR: HashMap<LogicalOperator, &'static str> = [
        (LogicalOperator::And, "&&"),
        (LogicalOperator::Or, "||"),
        (LogicalOperator::Not, "!"),
    ]
    .iter()
    .cloned()
    .collect();
    static ref RELATION_OPERATOR: HashMap<RelationOperator, &'static str> = [
        (RelationOperator::Equal, "=="),
        (RelationOperator::Greater, ">"),
        (RelationOperator::Less, "<"),
        (RelationOperator::NotEqual, "<>"),
        (RelationOperator::GreaterOrEqual, ">="),
        (RelationOperator::LessOrEqual, "<="),
    ]
    .iter()
    .cloned()
    .collect();
}

// TODO: Document this function
pub fn match_operator(content: &str, base_pos: usize) -> Token {
    let calc = match_calculation_operator(content, base_pos);
    if !calc.is_invalid() {
        return calc;
    }

    let logical = match_logical_operator(content, base_pos);
    if !logical.is_invalid() {
        return logical;
    }

    let relation = match_relation_operator(content, base_pos);
    if !relation.is_invalid() {
        return relation;
    }

    let root = match_root_operator(content, base_pos);
    if !root.is_invalid() {
        return root;
    }

    return Token::new_invalid();
}

pub fn match_calculation_operator(content: &str, base_pos: usize) -> Token {
    for (&operator, &operator_char) in CALCULATION_OPERATOR.iter() {
        if content.starts_with(operator_char) {
            return Token::new(
                TokenContent::Operator(Operator::Calculation(operator)),
                Position::new(base_pos, operator_char.len()),
            );
        }
    }

    return Token::new_invalid();
}

pub fn match_logical_operator(content: &str, base_pos: usize) -> Token {
    for (&operator, operator_str) in LOGICAL_OPERATOR.iter() {
        if content.starts_with(operator_str) {
            return Token::new(
                TokenContent::Operator(Operator::Logical(operator)),
                Position::new(base_pos, operator_str.len()),
            );
        }
    }

    return Token::new_invalid();
}

pub fn match_relation_operator(content: &str, base_pos: usize) -> Token {
    for (&operator, operator_str) in RELATION_OPERATOR.iter() {
        if content.starts_with(operator_str) {
            return Token::new(
                TokenContent::Operator(Operator::Relation(operator)),
                Position::new(base_pos, operator_str.len()),
            );
        }
    }

    return Token::new_invalid();
}

pub fn match_root_operator(content: &str, base_pos: usize) -> Token {
    for (&operator, operator_str) in ROOT_OPERATOR.iter() {
        if content.starts_with(operator_str) {
            return Token::new(
                TokenContent::Operator(operator),
                Position::new(base_pos, operator_str.len()),
            );
        }
    }

    return Token::new_invalid();
}
