use crate::lexer::tokenize::tokenize;
use crate::package_generator::availability_check::expression::expr_sequence::check_expression_sequence;
use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::parser::builder::expression_builder::expression_term_decorator;
use crate::parser::decorator::decorate_token;
use crate::shared::ast::blocks::data::DataType;
use crate::shared::ast::blocks::expression::SimpleExpression;
use crate::shared::utils::identifier::Identifier;

#[test]
fn sequence() {
    // A legal expression
    let mut tokens = tokenize("1 * (2 + 3)", true).unwrap();
    let mut expr = expression_infix_to_postfix(expression_term_decorator(&decorate_token(tokens).0));

    assert!(check_expression_sequence(SimpleExpression {
        postfix_expr: expr.clone(),
        output_type: DataType {
            data_type_id: Identifier::empty(),
            is_array: false,
        },
    }));

    // An illegal expression
    tokens = tokenize("8 * (2 + 3) -", true).unwrap();
    expr = expression_infix_to_postfix(expression_term_decorator(&decorate_token(tokens).0));
    assert!(!check_expression_sequence(SimpleExpression {
        postfix_expr: expr.clone(),
        output_type: DataType {
            data_type_id: Identifier::empty(),
            is_array: false,
        },
    }));
}
