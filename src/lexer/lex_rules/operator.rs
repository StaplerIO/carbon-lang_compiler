use crate::shared::token::{Operator, OperatorType, LogicalOperator, RelationOperator, CalculationOperator};

/**
 * Return:
 * A tuple (Operator, number) : The number is the length of the operator
 */
pub fn match_operator(content: String) -> (Operator, usize) {
    return match content.chars().nth(0).unwrap() {
        '+' => {
            (Operator {
                operator_type: OperatorType::Calculation,
                calculation: Option::from(CalculationOperator::Plus),
                relation: None,
                logical: None,
            }, 1)
        }
        '-' => {
            (Operator {
                operator_type: OperatorType::Calculation,
                calculation: Option::from(CalculationOperator::Minus),
                relation: None,
                logical: None,
            }, 1)
        }
        '*' => {
            (Operator {
                operator_type: OperatorType::Calculation,
                calculation: Option::from(CalculationOperator::Times),
                relation: None,
                logical: None,
            }, 1)
        }
        '/' => {
            (Operator {
                operator_type: OperatorType::Calculation,
                calculation: Option::from(CalculationOperator::Divide),
                relation: None,
                logical: None,
            }, 1)
        }
        '%' => {
            (Operator {
                operator_type: OperatorType::Calculation,
                calculation: Option::from(CalculationOperator::Mod),
                relation: None,
                logical: None,
            }, 1)
        }
        '<' => {
            match content.chars().nth(1).unwrap() {
                '=' => {
                    // Matches a "<=" sequence
                    (Operator {
                        operator_type: OperatorType::Relation,
                        calculation: None,
                        relation: Option::from(RelationOperator::LessEqual),
                        logical: None,
                    }, 2)
                }
                '>' => {
                    // Matches a "<>" sequence
                    (Operator {
                        operator_type: OperatorType::Relation,
                        calculation: None,
                        relation: Option::from(RelationOperator::NotEqual),
                        logical: None,
                    }, 2)
                }
                _ => {
                    // Matches a "<" sequence
                    (Operator {
                        operator_type: OperatorType::Relation,
                        calculation: None,
                        relation: Option::from(RelationOperator::Less),
                        logical: None,
                    }, 1)
                }
            }
        }
        '>' => {
            match content.chars().nth(1).unwrap() {
                '=' => {
                    // Matches a ">=" sequence
                    (Operator {
                        operator_type: OperatorType::Relation,
                        calculation: None,
                        relation: Option::from(RelationOperator::BiggerEqual),
                        logical: None,
                    }, 2)
                }
                _ => {
                    // Matches a ">" sequence
                    (Operator {
                        operator_type: OperatorType::Relation,
                        calculation: None,
                        relation: Option::from(RelationOperator::Bigger),
                        logical: None,
                    }, 1)
                }
            }
        }
        '=' => {
            match content.chars().nth(1).unwrap() {
                '=' => {
                    // Matches a "==" sequence
                    (Operator {
                        operator_type: OperatorType::Relation,
                        calculation: None,
                        relation: Option::from(RelationOperator::Equal),
                        logical: None,
                    }, 2)
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
            (Operator {
                operator_type: OperatorType::Logical,
                calculation: None,
                relation: None,
                logical: Option::from(LogicalOperator::Not),
            }, 1)
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
                (Operator {
                    operator_type: OperatorType::Logical,
                    calculation: None,
                    relation: None,
                    logical: Option::from(LogicalOperator::And),
                }, 2)
            } else if capture.eq("||") {
                (Operator {
                    operator_type: OperatorType::Logical,
                    calculation: None,
                    relation: None,
                    logical: Option::from(LogicalOperator::Or),
                }, 2)
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
