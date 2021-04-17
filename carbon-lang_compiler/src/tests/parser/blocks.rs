mod tests {
    use crate::lexer::tokenize::tokenize;
    use crate::parser::builder::blocks::assignment::assignment_block;
    use crate::shared::token::CalculationOperator;

    #[test]
    fn assignment() {
        let tokens = tokenize(String::from("a = 1 + 2;"));
        let result = assignment_block(tokens).0.unwrap();

        assert_eq!(result.identifier, String::from("a"));

        let expr = result.eval_expression.postfix_expr;
        assert_eq!(expr.len(), 3);
        assert_eq!(expr[0].clone().number.unwrap(), String::from("1"));
        assert_eq!(expr[1].clone().number.unwrap(), String::from("2"));
        assert_eq!(expr[2].clone().operator.unwrap().calculation.unwrap(), CalculationOperator::Plus);
    }
}
