use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::ast::action::ReturnAction;
use crate::parser::utils::find_next_semicolon;
use crate::shared::ast::blocks::expression::Expression;
use crate::parser::builder::expression_builder::expression_infix_to_postfix;

pub fn build_return_statement(tokens: Vec<DecoratedToken>) -> (Option<ReturnAction>, usize) {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());

    if next_semicolon_pos > 0 {
        if next_semicolon_pos == 1 {
            // No return value
            return (Option::from(ReturnAction{
                value: Expression { postfix_expr: vec![] }
            }), next_semicolon_pos as usize);
        } else {
            // With return value
            let mut expression_zone = tokens.clone();
            expression_zone.remove(expression_zone.len() - 1);
            expression_zone.remove(0);

            return (Option::from(ReturnAction{
                value: Expression { postfix_expr: expression_infix_to_postfix(expression_zone.clone()) }
            }), next_semicolon_pos as usize);
        }
    }

    return (None, 0);
}
