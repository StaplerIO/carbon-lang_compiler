use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::shared::token::operator::{CalculationOperator, Operator};
use crate::parser::builder::blocks::call::bare_function_call_builder;
use crate::shared::ast::blocks::expression::{ExprDataTerm, ExprTerm, RelationExpression, SimpleExpression, TermContent};
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenContent};
use crate::shared::token::container::ContainerType;
use crate::shared::utils::identifier::Identifier;

lazy_static! {
    /**
     * Operator priority:
     * calculation > relation > logical
     */
    static ref CALC_OPERATOR_PRIORITY: HashMap<CalculationOperator, u8> = [
        (CalculationOperator::Addition, 1),
        (CalculationOperator::Subtraction, 1),
        (CalculationOperator::Multiply, 2),
        (CalculationOperator::Division, 2),
        (CalculationOperator::Modulo, 2)
    ].iter().cloned().collect();
}

pub fn expression_term_decorator(tokens: &Vec<DecoratedToken>) -> Vec<ExprTerm> {
    let mut result: Vec<ExprTerm> = vec![];

    let mut index: usize = 0;
    while index < tokens.len() {
        let token = tokens[index].clone();
        match token.content {
            DecoratedTokenContent::Data(d) => {
                // In this situation, there are 2 different branches: normal data or function call

                // 1) function call
                {
                    let function_call = bare_function_call_builder(tokens[index..].to_vec());
                    if function_call.is_ok() {
                        result.push(ExprTerm {
                            content: TermContent::Data(ExprDataTerm::FunctionCall(function_call.clone().ok().unwrap().0)),
                            original_token: vec![]
                        });

                        index += function_call.ok().unwrap().1;
                        continue;
                    }

                    // 2) normal data
                    if d.get_typename().is_none() {
                        result.push(ExprTerm {
                            content: TermContent::Data(ExprDataTerm::from_data_token(&d.clone())),
                            original_token: vec![]
                        });
                    }
                }
            },
            DecoratedTokenContent::Container(x) => {
                if x.is_bracket() {
                    result.push(ExprTerm {
                        content: TermContent::Priority(x == ContainerType::Bracket),
                        original_token: vec![]
                    });
                }
            },
            DecoratedTokenContent::Operator(x) => {
                if is_operator_dt(token.clone()) {
                    result.push(ExprTerm {
                        content: TermContent::Operator(x),
                        original_token: vec![]
                    });
                }
            },
            _ => panic!("Illegal token stream for expression builder encountered!")
        }

        index += 1;
    }

    return result;
}

// dt means DecoratedToken
fn is_operator_dt(token: DecoratedToken) -> bool {
    return match token.content {
        DecoratedTokenContent::Operator(x) => {
            match x {
                Operator::Calculation(_) => true,
                Operator::Relation(_) => true,
                Operator::Logical(_) => true,
                _ => false,
            }
        },
        _ => false
    }
}

// TODO: We leave the postfix expression for code generator (solve the expression later)
pub fn expression_infix_to_postfix(terms: Vec<ExprTerm>) -> Vec<ExprTerm> {
    let mut result: Vec<ExprTerm> = vec![];
    let mut operator_stack: Vec<ExprTerm> = vec![];

    for token in terms {
        match token.content {
            TermContent::Data(_) => {
                // Push all terms into result directly (infix to postfix)
                result.push(token.clone());
            }
            TermContent::Operator(_) => {
                // The previous TermType::Priority must increased the priority level
                while !operator_stack.is_empty()
                    && operator_stack.last().unwrap().content.get_priority().is_none()
                {
                    // Pop if operator priority is higher than current operator
                    if priority_is_higher(operator_stack.last().unwrap().clone(), token.clone()) {
                        result.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }

                operator_stack.push(token.clone());
            }
            TermContent::Priority(x) => {
                if x {
                    operator_stack.push(token.clone());
                } else {
                    // Pop to result until the operator is a bracket (not anti-bracket)
                    while operator_stack.last().unwrap().content.get_priority().is_none() {
                        result.push(operator_stack.pop().unwrap());
                    }

                    // Pop this bracket (it won't be transferred to result)
                    operator_stack.pop();
                }
            }
            _ => {}
        };
    }

    // Push all operators remaining
    while !operator_stack.is_empty() {
        result.push(operator_stack.pop().unwrap());
    }

    return result;
}

// et means ExprTerm
fn is_operator_et(token: ExprTerm) -> bool {
    if token.content.get_operator().is_some() {
        let operator = token.content.get_operator().unwrap();
        return match operator {
            Operator::Calculation(_) => true,
            Operator::Relation(_) => true,
            Operator::Logical(_) => true,
            _ => false,
        }
    }

    return false;
}

// Return true if the priority of "a" is higher than or equal to "b"
fn priority_is_higher(a: ExprTerm, b: ExprTerm) -> bool {
    if is_operator_et(a.clone()) && is_operator_et(b.clone()) {
        return if !a.content.get_operator().unwrap().eq_entry(&b.content.get_operator().unwrap()) {
            get_operator_priority(&a.content.get_operator().unwrap())
                >= get_operator_priority(&b.content.get_operator().unwrap())
        } else if a.content.get_operator().unwrap().eq_entry(&Operator::Calculation(CalculationOperator::Invalid)) {
            // Then they are equal on operator type (ElseIf ~ End If)
            // So we just need to compare with 1 token

            // Get value inside the enum
            let ta = match a.content.get_operator().unwrap() {
                Operator::Calculation(x) => x,
                _ => panic!("Invalid operator"),
            };

            let tb = match b.content.get_operator().unwrap() {
                Operator::Calculation(x) => x,
                _ => panic!("Invalid operator"),
            };

            CALC_OPERATOR_PRIORITY[&ta] >= CALC_OPERATOR_PRIORITY[&tb]
        } else {
            true
        };
    }

    panic!("Token is not an operator!");
}

// Make sure the expression contains a relation operator
pub fn relation_expression_builder(terms: Vec<ExprTerm>) -> RelationExpression {
    // Split expression by the relation operator
    let mut op_position: usize = usize::MAX;
    for (index, term) in terms.iter().enumerate() {
        if term.content.get_operator().is_some() {
            if term.content.get_operator().unwrap().get_relation_op().is_some() {
                op_position = index;
                break;
            }
        }
    }

    // There must be an operator term
    assert_ne!(op_position, usize::MAX);

    let split = terms.split_at(op_position);
    let left_expr = expression_infix_to_postfix(split.0.to_vec());

    let right_expr = expression_infix_to_postfix(split.1.to_vec()[1..].to_vec());

    let rel_op = match terms[op_position].content.get_operator().unwrap() {
        Operator::Relation(x) => x,
        _ => panic!("Invalid operator"),
    };

    return RelationExpression {
        left: SimpleExpression { postfix_expr: left_expr, output_type: Identifier::empty() },
        right: SimpleExpression { postfix_expr: right_expr, output_type: Identifier::empty() },
        expected_relation: *rel_op
    };
}


fn get_operator_priority(op: &Operator) -> u8 {
    return match op {
        Operator::Logical(_) => 1,
        Operator::Relation(_) => 2,
        Operator::Calculation(_) => 3,
        _ => panic!("Invalid operator type!"),
    }
}
