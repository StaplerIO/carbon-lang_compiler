use crate::shared::token::{ContainerType, CalculationOperator, OperatorType};
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType, DataType};
use crate::shared::ast::blocks::expression::{ExprTerm, TermType};
use crate::parser::builder::blocks::call::bare_function_call_builder;

lazy_static! {
    /**
     * Operator priority:
     * calculation > relation > logical
     */
    static ref CALC_OPERATOR_PRIORITY: HashMap<CalculationOperator, u8> = [
        (CalculationOperator::Plus, 1),
        (CalculationOperator::Minus, 1),
        (CalculationOperator::Times, 2),
        (CalculationOperator::Divide, 2),
        (CalculationOperator::Mod, 2)
    ].iter().cloned().collect();

    static ref OPERATOR_PRIORITY: HashMap<OperatorType, u8> = [
        (OperatorType::Logical, 1),
        (OperatorType::Relation, 2),
        (OperatorType::Calculation, 3)
    ].iter().cloned().collect();
}

pub fn expression_term_decorator(mut tokens: Vec<DecoratedToken>) -> Vec<ExprTerm> {
    let mut result: Vec<ExprTerm> = vec![];

    while !tokens.is_empty() {
        let token = tokens.first().unwrap();
        if token.token_type == DecoratedTokenType::Data {
            // In this situation, there are 2 different branches: normal data or function call

            // 1) function call
            let function_call = bare_function_call_builder(tokens.clone());
            if function_call.1 != -1 {
                result.push(ExprTerm {
                    term_type: TermType::FunctionCall,
                    data: None,
                    function_call: function_call.0,
                    operator: None,
                    priority: None
                });

                tokens = tokens[(function_call.1 as usize)..].to_vec();
                continue;
            }

            // 2) normal data
            let data = token.clone().data.unwrap();
            if data.data_type != DataType::Type {
                result.push(ExprTerm {
                    term_type: TermType::Data,
                    data: Option::from(data),
                    function_call: None,
                    operator: None,
                    priority: None
                });

                tokens = tokens[1..].to_vec();
                continue;
            }
        } else if is_bracket(token.clone()) {
            result.push(ExprTerm {
                term_type: TermType::Priority,
                data: None,
                function_call: None,
                operator: None,
                priority: Option::from(token.container.unwrap() == ContainerType::Bracket)
            });

            tokens = tokens[1..].to_vec();
            continue;
        } else if is_operator_dt(token.clone()) {
            result.push(ExprTerm {
                term_type: TermType::Operator,
                data: None,
                function_call: None,
                operator: token.operator,
                priority: None
            });

            tokens = tokens[1..].to_vec();
            continue;
        }

        panic!("Illegal token stream for expression builder encountered!");
    }

    return result;
}

// dt means DecoratedToken
fn is_operator_dt(token: DecoratedToken) -> bool {
    if token.token_type == DecoratedTokenType::Operator {
        let operator = token.operator.unwrap();
        return operator.operator_type == OperatorType::Calculation ||
            operator.operator_type == OperatorType::Relation ||
            operator.operator_type == OperatorType::Logical;
    }

    return false;
}

fn is_bracket(token: DecoratedToken) -> bool {
    if token.token_type == DecoratedTokenType::Container {
        let container = token.container.unwrap();

        return container == ContainerType::Bracket ||
            container == ContainerType::AntiBracket;
    }

    return false;
}

// We leave the postfix expression for code generator (solve the expression later)
pub fn expression_infix_to_postfix(tokens: Vec<DecoratedToken>) -> Vec<ExprTerm> {
    let decorated_tokens = expression_term_decorator(tokens.clone());

    let mut result: Vec<ExprTerm> = vec![];
    let mut operator_stack: Vec<ExprTerm> = vec![];

    for token in decorated_tokens {
        match token.term_type {
            TermType::Data => {
                // Push all terms into result directly (infix to postfix)
                result.push(token.clone());
            },
            TermType::FunctionCall => {
                // FunctionCall == Data, because the result of a FunctionCall is a value
                result.push(token.clone());
            },
            TermType::Operator => {
                // The previous TermType::Priority must increased the priority level
                while !operator_stack.is_empty() &&
                    operator_stack.last().unwrap().term_type != TermType::Priority {
                    // Pop if operator priority is higher than current operator
                    if priority_is_higher(operator_stack.last().unwrap().clone(), token.clone()) {
                        result.push(operator_stack.pop().unwrap());
                    } else { break; }
                }

                operator_stack.push(token.clone());
            },
            TermType::Priority => {
                if token.priority.unwrap() {
                    operator_stack.push(token.clone());
                } else {
                    // Pop to result until the operator is a bracket (not anti-bracket)
                    while operator_stack.last().unwrap().term_type != TermType::Priority {
                        result.push(operator_stack.pop().unwrap());
                    }

                    // Pop this bracket (it won't be transferred to result)
                    operator_stack.pop();
                }
            },
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
    if token.term_type == TermType::Operator {
        let operator = token.operator.unwrap();
        return operator.operator_type == OperatorType::Calculation ||
            operator.operator_type == OperatorType::Relation ||
            operator.operator_type == OperatorType::Logical;
    }

    return false;
}

// Return true if the priority of "a" is higher than or equal to "b"
fn priority_is_higher(a: ExprTerm, b: ExprTerm) -> bool {
    if is_operator_et(a.clone()) && is_operator_et(b.clone()) {
        return if a.operator.unwrap().operator_type != b.operator.unwrap().operator_type {
            OPERATOR_PRIORITY[&a.operator.unwrap().operator_type] >= OPERATOR_PRIORITY[&b.operator.unwrap().operator_type]
        } else if a.operator.unwrap().operator_type == OperatorType::Calculation {
            // Then they are equal on operator type (ElseIf ~ End If)
            // So we just need to compare with 1 token

            CALC_OPERATOR_PRIORITY[&a.operator.unwrap().calculation.unwrap()] >= CALC_OPERATOR_PRIORITY[&b.operator.unwrap().calculation.unwrap()]
        } else {
            true
        };
    }

    panic!("Token is not an operator!");
}
