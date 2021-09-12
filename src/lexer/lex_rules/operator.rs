use crate::shared::token::{Operator, OperatorType, LogicalOperator, RelationOperator, CalculationOperator};

/**
 * Return:
 * A tuple (Operator, number) : The number is the length of the operator
 */
pub fn match_operator(content: String) -> (Operator, usize) {
    return match content.chars().nth(0).unwrap() {
        '+' => {
            (Operator::new_calculation(CalculationOperator::Plus), 1)
        }
        '-' => {
            (Operator::new_calculation(CalculationOperator::Minus), 1)
        }
        '*' => {
            (Operator::new_calculation(CalculationOperator::Times), 1)
        }
        '/' => {
            (Operator::new_calculation(CalculationOperator::Divide), 1)
        }
        '%' => {
            (Operator::new_calculation(CalculationOperator::Mod), 1)
        }
        '<' => {
            match content.chars().nth(1).unwrap() {
                '=' => {
                    // Matches a "<=" sequence
                    (Operator::new_relation(RelationOperator::LessEqual), 2)
                }
                '>' => {
                    // Matches a "<>" sequence
                    (Operator::new_relation(RelationOperator::NotEqual), 2)
                }
                _ => {
                    // Matches a "<" sequence
                    (Operator::new_relation(RelationOperator::Less), 1)
                }
            }
        }
        '>' => {
            match content.chars().nth(1).unwrap() {
                '=' => {
                    // Matches a ">=" sequence
                    (Operator::new_relation(RelationOperator::BiggerEqual), 2)
                }
                _ => {
                    // Matches a ">" sequence
                    (Operator::new_relation(RelationOperator::Bigger), 1)
                }
            }
        }
        '=' => {
            match content.chars().nth(1).unwrap() {
                '=' => {
                    // Matches a "==" sequence
                    (Operator::new_relation(RelationOperator::Equal), 2)
                }
                _ => {
                    // Matches a "=" sequence
                    (Operator {
                        operator_type: OperatorType::Assignment,
                        calculation: None,
                        relation: None,
                        logical: None,
                    }, 1)
                }
            }
        }
        '!' => {
            (Operator::new_logical(LogicalOperator::Not), 1)
        }
        ',' => {
            (Operator {
                operator_type: OperatorType::Comma,
                calculation: None,
                relation: None,
                logical: None,
            }, 1)
        }
        _ => {
            let capture = String::from(&content[0..2]);
            if capture.eq("&&") {
                (Operator::new_logical(LogicalOperator::And), 2)
            } else if capture.eq("||") {
                (Operator::new_logical(LogicalOperator::Or), 2)
            } else if capture.eq("::") {
                (Operator {
                    operator_type: OperatorType::Scope,
                    calculation: None,
                    relation: None,
                    logical: Option::from(LogicalOperator::Unset),
                }, 2)
            } else {
                (Operator {
                    operator_type: OperatorType::Unset,
                    calculation: None,
                    relation: None,
                    logical: None,
                }, 0)
            }
        }
    };
}
