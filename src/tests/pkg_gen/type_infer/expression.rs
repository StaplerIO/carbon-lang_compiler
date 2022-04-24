use crate::lexer::tokenize::tokenize;
use crate::package_generator::type_inference::expression::infer_expression_output_type;
use crate::package_generator::utils::infer_every_expression_data_term_type;
use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::parser::builder::expression_builder::expression_term_decorator;
use crate::parser::decorator::decorate_token;
use crate::shared::ast::action::VariableDefinition;
use crate::shared::ast::blocks::expression::SimpleExpression;

#[test]
fn expression_data_type() {
    let tokens = tokenize("1 + 2 - 3.55", true);
    let mut expr = SimpleExpression {
        postfix_expr: expression_infix_to_postfix(expression_term_decorator(decorate_token(tokens.clone()))),
        output_type: "".to_string(),
    };

    let defined_vars: Vec<VariableDefinition> = [VariableDefinition {
        identifier: String::from("bcd"),
        type_name: String::from("number"),
    }]
        .to_vec();

    let defined_types: Vec<String> = [
        String::from("number"),
        String::from("str"),
        String::from("char"),
    ]
        .to_vec();

    // Infer every DataTerm's type
    expr = infer_every_expression_data_term_type(&expr, &vec![], &defined_vars);

    assert_eq!(
        infer_expression_output_type(&expr, &defined_types).unwrap(),
        String::from("number")
    );
}
