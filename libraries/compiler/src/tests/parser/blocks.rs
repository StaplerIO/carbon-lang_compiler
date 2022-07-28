use crate::lexer::tokenize::tokenize;
use crate::parser::builder::blocks::action_block::action_block_builder;
use crate::parser::builder::blocks::assignment::assignment_block_builder;
use crate::parser::builder::blocks::call::call_action_builder;
use crate::parser::builder::blocks::condition::if_block_builder;
use crate::parser::builder::blocks::declaration::declaration_action_builder;
use crate::parser::builder::blocks::loops::while_action_builder;
use crate::parser::builder::blocks::return_expression::return_action_builder;
use crate::parser::builder::blocks::short_actions::short_statements_builder;
use crate::parser::builder::function_builder::function_builder;
use crate::parser::decorator::decorate_token;
use crate::shared::ast::action::ActionContent;
use crate::shared::token::operator::CalculationOperator;

#[test]
fn assignment() {
    let tokens = tokenize("a = 1 + 2;", true).unwrap();
    let raw = assignment_block_builder(&decorate_token(tokens.clone()));

    let result = raw.clone().ok().unwrap().0.get_assignment_action().unwrap().clone();
    assert_eq!(raw.ok().unwrap().1, tokens.len());

    assert_eq!(result.identifier, String::from("a"));

    let expr = &result.eval_expression.postfix_expr;
    assert_eq!(expr.len(), 3);
    assert_eq!(
        *expr[0].content.get_data_term().unwrap().get_number().unwrap(),
        String::from("1")
    );
    assert_eq!(
        *expr[1].content.get_data_term().unwrap().get_number().unwrap(),
        String::from("2")
    );
    assert_eq!(
        expr[2].content.get_operator().unwrap().get_calc_op().unwrap(),
        CalculationOperator::Addition
    );
}

#[test]
fn variable_declaration() {
    let tokens = tokenize("decl var number foo;", true).unwrap();
    let raw = declaration_action_builder(&decorate_token(tokens.clone()));

    let result = raw.clone().ok().unwrap().0.get_declaration_action().unwrap().clone();
    assert_eq!(raw.ok().unwrap().1, tokens.len());

    assert_eq!(result.identifier, String::from("foo"));
    assert_eq!(result.data_type, String::from("number"));
    assert_eq!(result.is_variable, true);
}

#[test]
fn function_call() {
    let tokens = tokenize("call func_1(5, 2.66, var1, 3 - 2);", true).unwrap();
    let raw = call_action_builder(&decorate_token(tokens.clone()));

    let result = raw.clone().ok().unwrap().0.get_call_action().unwrap().clone();
    assert_eq!(raw.ok().unwrap().1, tokens.len());

    assert_eq!(result.function_name, String::from("func_1"));
    assert_eq!(result.arguments.len(), 4);
    assert_eq!(result.arguments[0].postfix_expr.len(), 1);
    assert_eq!(result.arguments[1].postfix_expr.len(), 1);
    assert_eq!(result.arguments[2].postfix_expr.len(), 1);
    assert_eq!(
        *result.arguments[2].postfix_expr[0]
            .content
            .get_data_term()
            .unwrap()
            .get_identifier()
            .unwrap(),
        String::from("var1")
    );
    // Postfix expression: 3 2 -
    assert!(result.arguments.last().unwrap().postfix_expr.last().unwrap().content.get_operator().is_some());
}

#[test]
fn return_from_function_no_value() {
    let tokens = tokenize("return;", true).unwrap();
    let raw = return_action_builder(&decorate_token(tokens.clone()));

    let result = raw.clone().ok().unwrap().0.get_return_action().unwrap().clone();
    assert_eq!(raw.ok().unwrap().1, tokens.len());

    assert_eq!(result.value.unwrap().postfix_expr.len(), 0);
}

#[test]
fn return_from_function_with_value() {
    let tokens = tokenize("return 1 + 2 * tb_234;", true).unwrap();
    let raw = return_action_builder(&decorate_token(tokens.clone()));

    let result = raw.clone().ok().unwrap().0.get_return_action().unwrap().clone();
    assert_eq!(raw.ok().unwrap().1, tokens.len());

    let val = result.value.unwrap();
    assert_eq!(val.postfix_expr.len(), 5);
    // Value expression: 1 2 tb_234 * +
    assert!(
        val.postfix_expr[0]
            .content
            .get_data_term()
            .unwrap()
            .clone()
            .get_number()
            .is_some(),
    );
}

#[test]
fn single_token_statement_break() {
    let tokens = tokenize("break;", true).unwrap();
    let raw = short_statements_builder(&decorate_token(tokens.clone()));

    let result = raw.clone().ok().unwrap().0;
    assert_eq!(raw.ok().unwrap().1, tokens.len());

    assert_eq!(result.content, ActionContent::BreakStatement);
}

#[test]
fn single_token_statement_continue() {
    let tokens = tokenize("continue;", true).unwrap();
    let raw = short_statements_builder(&decorate_token(tokens.clone()));

    let result = raw.clone().ok().unwrap().0;
    assert_eq!(raw.ok().unwrap().1, tokens.len());

    assert_eq!(result.content, ActionContent::ContinueStatement);
}

#[test]
fn action_block() {
    let tokens = tokenize("decl var number foo;\
                                                foo = 1;
                                                foo = foo + 1;
                                                call func_1(5, 2.66, var1, 3 - 2);\
                                                if (foo == 2) { return; }\
                                                return 1 + 2 * tb_234;\
                                                while (1 == 1) { call func_1(0); }",
                          true).unwrap();

    let result = action_block_builder(decorate_token(tokens.clone()));

    assert_eq!(result.len(), 7);

    assert!(result[0].get_declaration_action().is_some());
    assert!(result[1].get_assignment_action().is_some());
    assert!(result[3].get_call_action().is_some());
    assert!(result[4].get_if_action().is_some());
    assert!(result[5].get_return_action().is_some());
}

#[test]
fn while_block() {
    let tokens = tokenize("while (1 + 1 == 2) { a = a + 1; return; }", true).unwrap();
    let raw = while_action_builder(&decorate_token(tokens.clone()));

    let result = raw.clone().ok().unwrap().0.get_while_block().unwrap().clone();
    assert_eq!(raw.ok().unwrap().1, tokens.len());

    assert_eq!(result.condition.left.postfix_expr.len(), 3);
    assert_eq!(result.condition.right.postfix_expr.len(), 1);
    assert_eq!(result.body.actions.len(), 2);
}

#[test]
fn if_block() {
    let tokens = tokenize("if (1 + 2 == 3) \
                                                    { a = a + 1; } \
                                                 elif (t2 == 5) \
                                                    { return; } \
                                                 elif (1 == 1) \
                                                    { call setup(); } \
                                                 else \
                                                    { decl var number foo; }", 
                          true).unwrap();
    let raw = if_block_builder(&decorate_token(tokens.clone()));

    let result = raw.clone().ok().unwrap().0.get_if_action().unwrap().clone();
    assert_eq!(raw.ok().unwrap().1, tokens.len());

    assert_eq!(result.if_block.condition.left.postfix_expr.len(), 3);
    assert_eq!(result.if_block.condition.right.postfix_expr.len(), 1);
    assert_eq!(result.elif_collection.len(), 2);
    assert_eq!(result.else_action.unwrap().actions.len(), 1);
}

#[test]
fn function_block() {
    let tokens = tokenize("decl func main(number a, number b)[number] { return a + b; }", true).unwrap();
    let raw = function_builder(&decorate_token(tokens.clone()));

    let result = raw.clone().ok().unwrap().0;
    assert_eq!(raw.ok().unwrap().1, tokens.len());

    assert_eq!(result.name, String::from("main"));
    assert_eq!(result.return_type, String::from("number"));
    assert_eq!(result.parameters.len(), 2);
    assert_eq!(result.body.len(), 1);
}
