mod tests {
    pub use crate::lexer::tokenize::tokenize;
    pub use crate::parser::builder::blocks::action_block::action_block_builder;
    pub use crate::parser::builder::blocks::assignment::assignment_block_builder;
    pub use crate::parser::builder::blocks::call::call_action_builder;
    pub use crate::parser::builder::blocks::condition::if_block_builder;
    pub use crate::parser::builder::blocks::declaration::declaration_action_builder;
    pub use crate::parser::builder::blocks::loops::while_action_builder;
    pub use crate::parser::builder::blocks::return_expression::return_action_builder;
    pub use crate::parser::builder::blocks::short_actions::short_statements_builder;
    pub use crate::parser::builder::function_builder::function_builder;
    pub use crate::parser::decorator::decorate_token;
    pub use crate::shared::ast::action::ActionType;
    pub use crate::shared::ast::blocks::expression::TermType;
    pub use crate::shared::ast::decorated_token::{DataType, DecoratedTokenType};
    pub use crate::shared::token::CalculationOperator;

    #[test]
    fn assignment() {
        let tokens = tokenize(String::from("a = 1 + 2;"));
        let raw = assignment_block_builder(decorate_token(tokens.clone()));

        let result = raw.0.unwrap().assignment_action.unwrap();
        assert_eq!(raw.1 as usize, tokens.len());

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
        let raw = declaration_action_builder(decorate_token(tokens.clone()));

        let result = raw.0.unwrap().declaration_action.unwrap();
        assert_eq!(raw.1 as usize, tokens.len());

        assert_eq!(result.identifier, String::from("foo"));
        assert_eq!(result.data_type, String::from("decimal"));
        assert_eq!(result.is_variable, true);
    }

    #[test]
    fn function_call() {
        let tokens = tokenize(String::from("call func_1(5, 2.66, var1, 3 - 2);"));
        let raw = call_action_builder(decorate_token(tokens.clone()));

        let result = raw.0.unwrap().call_action.unwrap();
        assert_eq!(raw.1 as usize, tokens.len());

        assert_eq!(result.function_name, String::from("func_1"));
        assert_eq!(result.arguments.len(), 4);
        assert_eq!(result.arguments[0].postfix_expr.len(), 1);
        assert_eq!(result.arguments[1].postfix_expr.len(), 1);
        assert_eq!(result.arguments[2].postfix_expr.len(), 1);
        assert_eq!(result.arguments[2].postfix_expr[0].clone().data.unwrap().clone().identifier.unwrap(), String::from("var1"));
        // Postfix expression: 3 2 -
        assert_eq!(result.arguments.last().unwrap().postfix_expr.last().unwrap().term_type, TermType::Operator);
    }

    #[test]
    fn return_from_function_no_value() {
        let tokens = tokenize(String::from("return;"));
        let raw = return_action_builder(decorate_token(tokens.clone()));

        let result = raw.0.unwrap().return_action.unwrap();
        assert_eq!(raw.1 as usize, tokens.len());

        assert_eq!(result.value.postfix_expr.len(), 0);
    }

    #[test]
    fn return_from_function_with_value() {
        let tokens = tokenize(String::from("return 1 + 2 * tb_234;"));
        let raw = return_action_builder(decorate_token(tokens.clone()));

        let result = raw.0.unwrap().return_action.unwrap();
        assert_eq!(raw.1 as usize, tokens.len());

        assert_eq!(result.value.postfix_expr.len(), 5);
        // Value expression: 1 2 tb_234 * +
        assert_eq!(result.value.postfix_expr[0].clone().data.unwrap().clone().data_type, DataType::Number);
    }

    #[test]
    fn single_token_statement_break() {
        let tokens = tokenize(String::from("break;"));
        let raw = short_statements_builder(decorate_token(tokens.clone()));

        let result = raw.0.unwrap();
        assert_eq!(raw.1 as usize, tokens.len());

        assert_eq!(result.action_type, ActionType::BreakStatement);
    }

    #[test]
    fn single_token_statement_continue() {
        let tokens = tokenize(String::from("continue;"));
        let raw = short_statements_builder(decorate_token(tokens.clone()));

        let result = raw.0.unwrap();
        assert_eq!(raw.1 as usize, tokens.len());

        assert_eq!(result.action_type, ActionType::ContinueStatement);
    }

    #[test]
    fn action_block() {
        let tokens = tokenize(String::from("decl var decimal foo;\
                                                                    foo = 1;
                                                                    foo = foo + 1;
                                                                    call func_1(5, 2.66, var1, 3 - 2);\
                                                                    if (foo == 2) { return; }\
                                                                    return 1 + 2 * tb_234;\
                                                                    while (1) { call func_1(0); }"));

        let result = action_block_builder(decorate_token(tokens.clone()));

        assert_eq!(result.len(), 7);

        assert_eq!(result[0].action_type, ActionType::DeclarationStatement);
        assert_eq!(result[1].action_type, ActionType::AssignmentStatement);
        assert_eq!(result[3].action_type, ActionType::CallStatement);
        assert_eq!(result[4].action_type, ActionType::IfStatement);
        assert_eq!(result[5].action_type, ActionType::ReturnStatement);
    }

    #[test]
    fn while_block() {
        let tokens = tokenize(String::from("while (1 + 1 == 2) { a = a + 1; return; }"));
        let raw = while_action_builder(decorate_token(tokens.clone()));

        let result = raw.0.unwrap().while_action.unwrap();
        assert_eq!(raw.1 as usize, tokens.len());

        assert_eq!(result.condition.postfix_expr.len(), 5);
        assert_eq!(result.body.actions.len(), 2);
    }

    #[test]
    fn if_block() {
        let tokens = tokenize(String::from("if (1 + 2 == 3) { a = a + 1; } elif (t2 == 5) { return; } elif (1) { call setup(); } else { decl var decimal foo; }"));
        let raw = if_block_builder(decorate_token(tokens.clone()));

        let result = raw.0.unwrap().if_action.unwrap();
        assert_eq!(raw.1 as usize, tokens.len() - 1);

        assert_eq!(result.if_block.condition.postfix_expr.len(), 5);
        assert_eq!(result.elif_collection.len(), 2);
        assert_eq!(result.else_action.unwrap().actions.len(), 1);
    }

    #[test]
    fn function_block() {
        let tokens = tokenize(String::from("decl func main(int a, decimal b)[decimal] { return a + b; }"));
        let raw = function_builder(decorate_token(tokens.clone()));

        let result = raw.0.unwrap();
        assert_eq!(raw.1 as usize, tokens.len());

        assert_eq!(result.name, String::from("main"));
        assert_eq!(result.return_type, String::from("decimal"));
        assert_eq!(result.parameters.len(), 2);
        assert_eq!(result.body.len(), 1);
    }
}
