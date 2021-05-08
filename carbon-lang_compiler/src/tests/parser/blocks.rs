mod tests {
    use crate::lexer::tokenize::tokenize;
    use crate::parser::builder::blocks::assignment::assignment_block;
    use crate::shared::token::CalculationOperator;
    use crate::parser::builder::blocks::declaration::declare_data;
    use crate::parser::decorator::decorate_token;

    #[test]
    fn assignment() {
        let tokens = tokenize(String::from("a = 1 + 2;"));
        let result = assignment_block(decorate_token(tokens.clone())).0.unwrap();

        assert_eq!(result.identifier, String::from("a"));

        let expr = result.eval_expression.postfix_expr;
        assert_eq!(expr.len(), 3);
        assert_eq!(expr[0].clone().data.unwrap().number.unwrap(), String::from("1"));
        assert_eq!(expr[1].clone().data.unwrap().number.unwrap(), String::from("2"));
        assert_eq!(expr[2].clone().operator.unwrap().calculation.unwrap(), CalculationOperator::Plus);
    }

    #[test]
    fn variable_declaration() {
        let tokens = tokenize(String::from("decl var decimal foo;"));
        let result = declare_data(decorate_token(tokens.clone())).0.unwrap();

        assert_eq!(result.identifier, String::from("foo"));
        assert_eq!(result.data_type, String::from("decimal"));
        assert_eq!(result.is_variable, true);
    }
}
