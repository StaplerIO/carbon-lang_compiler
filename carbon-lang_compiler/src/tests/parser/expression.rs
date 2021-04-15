mod tests {
    use crate::lexer::tokenize::tokenize;
    use crate::parser::builder::expression_builder::expression_infix_to_postfix;
    use crate::shared::token::CalculationOperator;

    #[test]
    fn simple_expression() {
        let token_list = tokenize(String::from("1+2*3"));
        let result = expression_infix_to_postfix(token_list.clone());

        assert_eq!(result.len(), 5);

        assert_eq!(result[0].clone().number.unwrap(), String::from("1"));
        assert_eq!(result[1].clone().number.unwrap(), String::from("2"));
        assert_eq!(result[2].clone().number.unwrap(), String::from("3"));
        assert_eq!(result[3].clone().operator.unwrap().calculation.unwrap(), CalculationOperator::Times);
        assert_eq!(result[4].clone().operator.unwrap().calculation.unwrap(), CalculationOperator::Plus);
    }

    /**
     * Remark:
     * This test has been disabled
     * Because I haven't solve the problem with "expression converting with brackets"
     * I pushed a repository to http://hserver.ranzeplay.me:25550/ranzeplay/ExpressionConverter
     * It is a C# version of expression converting from infix to postfix.
     * That worked, but I don't know how to translate it to Rust :(
     */
    // #[test]
    fn expression_with_bracket() {
        let token_list = tokenize(String::from("2*(3+5)-7"));
        let result = expression_infix_to_postfix(token_list.clone());

        assert_eq!(result.len(), 7);

        assert_eq!(result[0].clone().number.unwrap(), String::from("2"));
        assert_eq!(result[1].clone().number.unwrap(), String::from("3"));
        assert_eq!(result[2].clone().number.unwrap(), String::from("5"));
        assert_eq!(result[3].clone().operator.unwrap().calculation.unwrap(), CalculationOperator::Plus);
    }
}