use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::ast::action::{ReturnAction, Action, ActionType};
use crate::parser::utils::find_next_semicolon;
use crate::shared::ast::blocks::expression::Expression;
use crate::parser::builder::expression_builder::expression_infix_to_postfix;

pub fn return_action_builder(tokens: Vec<DecoratedToken>) -> (Option<Action>, usize) {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());

    if next_semicolon_pos > 0 {
        let mut result: Option<ReturnAction> = None;

        if next_semicolon_pos == 1 {
            // No return value
            result = Option::from(ReturnAction {
                value: Expression { postfix_expr: vec![] }
            });
        } else {
            // With return value
            let mut expression_zone = tokens.clone();
            expression_zone.remove(expression_zone.len() - 1);
            expression_zone.remove(0);

            result = Option::from(ReturnAction {
                value: Expression { postfix_expr: expression_infix_to_postfix(expression_zone.clone()) }
            });
        }

        return (Option::from(Action{
            action_type: ActionType::ReturnStatement,
            declaration_action: None,
            assignment_action: None,
            call_action: None,
            return_action: result,
            if_action: None,
            while_action: None,
            loop_action: None,
            switch_action: None
        }), next_semicolon_pos as usize);
    }

    return (None, 0);
}
