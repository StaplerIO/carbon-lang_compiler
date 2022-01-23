// TODO: Move this step to compiler/parser, check expression sequence right after ExpressionBuilder

use crate::shared::ast::blocks::expression::{ExprTerm, SimpleExpression, TermType};

pub fn check_expression_sequence(expression: SimpleExpression) -> bool {
    let mut expr_sequence = expression.postfix_expr.clone();
    while !(expr_sequence.len() == 1 && expr_sequence[0].term_type == TermType::Validated) {
        // Become true if current loop processed something
        let mut turn_processed = false;
        for index in 0..(expr_sequence.len() - 2) {
            if is_valid_data_term(expr_sequence[index].clone())
                && is_valid_data_term(expr_sequence[index + 1].clone())
                && expr_sequence[index + 2].term_type == TermType::Operator
            {
                // Remove 3 elements from current index
                expr_sequence.remove(index);
                expr_sequence.remove(index);
                expr_sequence.remove(index);

                expr_sequence.insert(
                    index,
                    ExprTerm {
                        term_type: TermType::Validated,
                        data: None,
                        operator: None,
                        priority: None,
                    },
                );

                turn_processed = true;
                break;
            }
        }

        if !turn_processed {
            return false;
        }
    }

    return true;
}

fn is_valid_data_term(term: ExprTerm) -> bool {
    return term.term_type == TermType::Validated || term.term_type == TermType::Data;
}
