use crate::lexer::tokenize::tokenize;
use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::parser::builder::expression_builder::expression_term_decorator;
use crate::parser::builder::expression_builder::relation_expression_builder;
use crate::parser::decorator::decorate_token;
use crate::shared::token::operator::CalculationOperator;

#[test]
fn simple_expression() {
    let tokens = tokenize("1+2*3", true).unwrap();
    let result = expression_infix_to_postfix(expression_term_decorator(decorate_token(tokens)));

    assert_eq!(result.len(), 5);

    assert_eq!(
        *result[0].content.get_data_term().unwrap().get_number().unwrap(),
        String::from("1")
    );
    assert_eq!(
        *result[1].content.get_data_term().unwrap().get_number().unwrap(),
        String::from("2")
    );
    assert_eq!(
        *result[2].content.get_data_term().unwrap().get_number().unwrap(),
        String::from("3")
    );
    assert_eq!(
        result[3].content.get_operator().unwrap().get_calc_op().unwrap(),
        CalculationOperator::Multiply
    );
    assert_eq!(
        result[4].content.get_operator().unwrap().get_calc_op().unwrap(),
        CalculationOperator::Addition
    );
}

#[test]
fn expression_with_bracket() {
    let tokens = tokenize("2*(3+5)-7", true).unwrap();
    let result = expression_infix_to_postfix(expression_term_decorator(decorate_token(tokens)));

    assert_eq!(result.len(), 7);

    assert_eq!(
        *result[0].content.get_data_term().unwrap().get_number().unwrap(),
        String::from("2")
    );
    assert_eq!(
        *result[1].content.get_data_term().unwrap().get_number().unwrap(),
        String::from("3")
    );
    assert_eq!(
        *result[2].content.get_data_term().unwrap().get_number().unwrap(),
        String::from("5")
    );
    assert_eq!(
        result[3].content.get_operator().unwrap().get_calc_op().unwrap(),
        CalculationOperator::Addition
    );
    assert_eq!(
        result[4].content.get_operator().unwrap().get_calc_op().unwrap(),
        CalculationOperator::Multiply
    );
    assert_eq!(
        *result[5].content.get_data_term().unwrap().get_number().unwrap(),
        String::from("7")
    );
    assert_eq!(
        result[6].content.get_operator().unwrap().get_calc_op().unwrap(),
        CalculationOperator::Subtraction
    );
}

#[test]
fn expression_with_function_call() {
    let tokens = tokenize("11+22.6*demo1(22.5)", true).unwrap();
    let result = expression_infix_to_postfix(expression_term_decorator(decorate_token(tokens)));

    assert_eq!(result.len(), 5);
}

#[test]
fn relation_expression() {
    let tokens = tokenize("1 + a > 3 + foo(144)", true).unwrap();
    let result = relation_expression_builder(expression_term_decorator(decorate_token(tokens)));

    assert_eq!(result.left.postfix_expr.len(), 3);
    assert_eq!(result.right.postfix_expr.len(), 3);
}
