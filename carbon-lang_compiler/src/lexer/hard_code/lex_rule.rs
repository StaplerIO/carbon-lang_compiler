use crate::shared::token::{ContainerType, Operator, LogicalOperator, RelationOperator, CalculationOperator, OperatorType};

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

pub fn match_semicolon(content: String) -> bool {
    return content.chars().nth(0).unwrap() == ';';
}

pub fn match_string(content: String) -> String {
    let mut result = String::new();

    if content.starts_with('\"') {
        for ch in content[1..].chars() {
            if ch != '\"' {
                result.push(ch);
            } else {
                break;
            }
        }
    }

    return result;
}
