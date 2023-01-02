use crate::lexer::tokenize::tokenize;
use crate::package_generator::type_inference::expression::infer_expression_output_type;
use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::parser::builder::expression_builder::expression_term_decorator;
use crate::parser::decorator::decorate_token;
use crate::shared::ast::blocks::data::DataType;
use crate::shared::ast::blocks::expression::SimpleExpression;
use crate::shared::utils::identifier::Identifier;

#[test]
fn expression_data_type() {
    let tokens = tokenize("1 + 2 - 3.55", true).unwrap();
    let mut expr = SimpleExpression {
        postfix_expr: expression_infix_to_postfix(expression_term_decorator(&decorate_token(tokens).0)),
        output_type: DataType {
            data_type_id: Identifier::empty(),
            is_array: false,
        },
    };

    let defined_types: Vec<Identifier> = [
        Identifier::single("number"),
        Identifier::single("str"),
        Identifier::single("char"),
    ]
        .to_vec();

    assert_eq!(
        infer_expression_output_type(&expr, &defined_types).unwrap(),
        Identifier::single("number")
    );
}
