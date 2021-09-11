mod tests {
    pub use crate::lexer::tokenize::tokenize;
    pub use crate::package_generator::type_inference::expression::{infer_expression_output_type, infer_expression_term_data_type};
    pub use crate::package_generator::utils::infer_every_expression_data_term_type;
    pub use crate::parser::builder::expression_builder::expression_infix_to_postfix;
    pub use crate::parser::decorator::decorate_token;
    pub use crate::shared::ast::action::VariableDefinition;
    pub use crate::shared::ast::blocks::expression::{Expression, TermType};

    #[test]
    fn expression_data_type() {
        let tokens = tokenize(String::from("1 + 2 - 3.55"));
        let mut expr = Expression {
            postfix_expr: expression_infix_to_postfix(decorate_token(tokens)),
            output_type: "".to_string()
        };

        let defined_vars: Vec<VariableDefinition> = [
            VariableDefinition {
                identifier: String::from("bcd"),
                type_name: String::from("number")
            }
        ].to_vec();

        let defined_types: Vec<String> = [
            String::from("number"),
            String::from("str"),
            String::from("char")
        ].to_vec();

        // Infer every DataTerm's type
        expr = infer_every_expression_data_term_type(expr, vec![], defined_vars);

        assert_eq!(infer_expression_output_type(expr, defined_types).unwrap(), String::from("number"));
    }
}