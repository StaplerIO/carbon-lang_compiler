mod tests {
    pub use crate::lexer::tokenize::tokenize;
    pub use crate::parser::builder::expression_builder::expression_infix_to_postfix;
    pub use crate::parser::builder::expression_builder::expression_term_decorator;
    pub use crate::parser::builder::expression_builder::relation_expression_builder;
    pub use crate::parser::decorator::decorate_token;
    pub use crate::shared::token::CalculationOperator;

    #[test]
    fn simple_expression() {
        let tokens = tokenize(String::from("1+2*3"));
        let result = expression_infix_to_postfix(expression_term_decorator(decorate_token(tokens)));

        assert_eq!(result.len(), 5);

        assert_eq!(
            result[0].clone().data.unwrap().number.unwrap(),
            String::from("1")
        );
        assert_eq!(
            result[1].clone().data.unwrap().number.unwrap(),
            String::from("2")
        );
        assert_eq!(
            result[2].clone().data.unwrap().number.unwrap(),
            String::from("3")
        );
        assert_eq!(
            result[3].clone().operator.unwrap().calculation.unwrap(),
            CalculationOperator::Times
        );
        assert_eq!(
            result[4].clone().operator.unwrap().calculation.unwrap(),
            CalculationOperator::Plus
        );
    }

    #[test]
    fn expression_with_bracket() {
        let tokens = tokenize(String::from("2*(3+5)-7"));
        let result = expression_infix_to_postfix(expression_term_decorator(decorate_token(tokens)));

        assert_eq!(result.len(), 7);

        assert_eq!(
            result[0].clone().data.unwrap().number.unwrap(),
            String::from("2")
        );
        assert_eq!(
            result[1].clone().data.unwrap().number.unwrap(),
            String::from("3")
        );
        assert_eq!(
            result[2].clone().data.unwrap().number.unwrap(),
            String::from("5")
        );
        assert_eq!(
            result[3].clone().operator.unwrap().calculation.unwrap(),
            CalculationOperator::Plus
        );
        assert_eq!(
            result[4].clone().operator.unwrap().calculation.unwrap(),
            CalculationOperator::Times
        );
        assert_eq!(
            result[5].clone().data.unwrap().number.unwrap(),
            String::from("7")
        );
        assert_eq!(
            result[6].clone().operator.unwrap().calculation.unwrap(),
            CalculationOperator::Minus
        );
    }

    #[test]
    fn expression_with_function_call() {
        let tokens = tokenize(String::from("11+22.6*demo1(22.5)"));
        let result = expression_infix_to_postfix(expression_term_decorator(decorate_token(tokens)));

        assert_eq!(result.len(), 5);
    }

    #[test]
    fn relation_expression() {
        let tokens = tokenize(String::from("1 + a > 3 + foo(144)"));
        let result = relation_expression_builder(expression_term_decorator(decorate_token(tokens)));

        assert_eq!(result.left.postfix_expr.len(), 3);
        assert_eq!(result.right.postfix_expr.len(), 3);
    }
}
