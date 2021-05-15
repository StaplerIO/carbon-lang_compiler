mod tests {
    use crate::lexer::tokenize::tokenize;
    use crate::parser::builder::blocks::assignment::assignment_block;
    use crate::shared::token::CalculationOperator;
    use crate::parser::builder::blocks::declaration::declare_data;
    use crate::parser::decorator::decorate_token;
    use crate::parser::builder::blocks::call::call_function;
    use std::any::Any;
    use crate::shared::ast::decorated_token::DecoratedTokenType;

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

    #[test]
    fn function_call() {
        let tokens = tokenize(String::from("call func_1(5, 2.66, var1, 3 - 2);"));
        let result = call_function(decorate_token(tokens.clone())).0.unwrap();

        assert_eq!(result.function_name, String::from("func_1"));
        assert_eq!(result.arguments.len(), 4);
        assert_eq!(result.arguments[0].postfix_expr.len(), 1);
        assert_eq!(result.arguments[1].postfix_expr.len(), 1);
        assert_eq!(result.arguments[2].postfix_expr.len(), 1);
        assert_eq!(result.arguments[2].postfix_expr[0].clone().data.unwrap().clone().identifier.unwrap(), String::from("var1"));
        // Postfix expression: 3 2 -
        assert_eq!(result.arguments.last().unwrap().postfix_expr.last().unwrap().token_type, DecoratedTokenType::Operator);
    }
}
