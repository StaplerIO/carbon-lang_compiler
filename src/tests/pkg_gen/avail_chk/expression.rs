use crate::lexer::tokenize::tokenize;
use crate::package_generator::availability_check::expression::expr_sequence::check_expression_sequence;
use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::parser::builder::expression_builder::expression_term_decorator;
use crate::parser::decorator::decorate_token;
use crate::shared::ast::blocks::expression::SimpleExpression;

#[test]
fn sequence() {
    // A legal expression
    let mut tokens = tokenize(String::from("1 * (2 + 3)"));
    let mut expr = expression_infix_to_postfix(expression_term_decorator(decorate_token(tokens.clone())));

    assert!(check_expression_sequence(SimpleExpression {
        postfix_expr: expr.clone(),
        output_type: String::new()
    }));

    // An illegal expression
    tokens = tokenize(String::from("8 * (2 + 3) -"));
    expr = expression_infix_to_postfix(expression_term_decorator(decorate_token(tokens.clone())));
    assert!(!check_expression_sequence(SimpleExpression {
        postfix_expr: expr.clone(),
        output_type: String::new()
    }));
}
