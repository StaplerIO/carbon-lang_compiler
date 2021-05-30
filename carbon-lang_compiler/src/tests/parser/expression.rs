mod tests {
    pub use crate::lexer::tokenize::tokenize;
    use crate::parser::builder::expression_builder::expression_infix_to_postfix;
    use crate::shared::token::CalculationOperator;
    use crate::parser::decorator::decorate_token;

    #[test]
    fn simple_expression() {
        let token_list = tokenize(String::from("1+2*3"));
        let result = expression_infix_to_postfix(decorate_token(token_list.clone()));

        assert_eq!(result.len(), 5);

        assert_eq!(result[0].clone().data.unwrap().number.unwrap(), String::from("1"));
        assert_eq!(result[1].clone().data.unwrap().number.unwrap(), String::from("2"));
        assert_eq!(result[2].clone().data.unwrap().number.unwrap(), String::from("3"));
        assert_eq!(result[3].clone().operator.unwrap().calculation.unwrap(), CalculationOperator::Times);
        assert_eq!(result[4].clone().operator.unwrap().calculation.unwrap(), CalculationOperator::Plus);
    }

    #[test]
    fn expression_with_bracket() {
        let token_list = tokenize(String::from("2*(3+5)-7"));
        let result = expression_infix_to_postfix(decorate_token(token_list.clone()));

        assert_eq!(result.len(), 7);

        assert_eq!(result[0].clone().data.unwrap().number.unwrap(), String::from("2"));
        assert_eq!(result[1].clone().data.unwrap().number.unwrap(), String::from("3"));
        assert_eq!(result[2].clone().data.unwrap().number.unwrap(), String::from("5"));
        assert_eq!(result[3].clone().operator.unwrap().calculation.unwrap(), CalculationOperator::Plus);
        assert_eq!(result[4].clone().operator.unwrap().calculation.unwrap(), CalculationOperator::Times);
        assert_eq!(result[5].clone().data.unwrap().number.unwrap(), String::from("7"));
        assert_eq!(result[6].clone().operator.unwrap().calculation.unwrap(), CalculationOperator::Minus);
    }
}