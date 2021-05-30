mod tests {
    use crate::lexer::tokenize::tokenize;
    use crate::parser::builder::blocks::assignment::assignment_block;
    use crate::shared::token::CalculationOperator;
    use crate::parser::builder::blocks::declaration::declare_data;
    use crate::parser::decorator::decorate_token;
    use crate::parser::builder::blocks::call::call_function;
    use crate::shared::ast::decorated_token::{DecoratedTokenType, DataType};
    use crate::parser::builder::blocks::return_expression::build_return_statement;
    use crate::parser::builder::blocks::short_actions::build_short_statements;
    use crate::shared::ast::action::ActionType;
    use crate::parser::builder::blocks::action_block::action_block_builder;

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

    #[test]
    fn return_from_function_no_value() {
        let tokens = tokenize(String::from("return;"));
        let result = build_return_statement(decorate_token(tokens.clone())).0.unwrap();

        assert_eq!(result.value.postfix_expr.len(), 0);
    }
    #[test]
    fn return_from_function_with_value() {
        let tokens = tokenize(String::from("return 1 + 2 * tb_234;"));
        let result = build_return_statement(decorate_token(tokens.clone())).0.unwrap();

        assert_eq!(result.value.postfix_expr.len(), 5);
        // Value expression: 1 2 tb_234 * +
        assert_eq!(result.value.postfix_expr[0].clone().data.unwrap().clone().data_type, DataType::Number);
    }

    #[test]
    fn single_token_statement_break(){
        let tokens = tokenize(String::from("break;"));
        let result = build_short_statements(decorate_token(tokens.clone())).0.unwrap();

        assert_eq!(result.action_type, ActionType::BreakStatement);
    }

    #[test]
    fn single_token_statement_continue(){
        let tokens = tokenize(String::from("continue;"));
        let result = build_short_statements(decorate_token(tokens.clone())).0.unwrap();

        assert_eq!(result.action_type, ActionType::ContinueStatement);
    }

    #[test]
    fn action_block() {
        let tokens = tokenize(String::from("decl var decimal foo;\
                                                                   foo = 1;
                                                                   foo = foo + 1;
                                                                   call func_1(5, 2.66, var1, 3 - 2);\
                                                                   return 1 + 2 * tb_234;"));

        let result = action_block_builder(decorate_token(tokens.clone()));

        assert_eq!(result.len(), 5);

        assert_eq!(result[0].action_type, ActionType::DeclarationStatement);
        assert_eq!(result[1].action_type, ActionType::AssignmentStatement);
        assert_eq!(result[3].action_type, ActionType::CallStatement);
        assert_eq!(result[4].action_type, ActionType::ReturnStatement);
    }
}
