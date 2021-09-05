use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::parser::utils::find_next_semicolon;
use crate::shared::ast::action::{Action, ActionType, ReturnAction};
use crate::shared::ast::blocks::expression::Expression;
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::token::KeywordType;
use crate::shared::error::general_error::GeneralError;

pub fn return_action_builder(tokens: Vec<DecoratedToken>) -> Result<(Action, usize), GeneralError<String>> {
    // Minimum: return ; (2 tokens in total)
    if tokens.len() >= 2 {
        if tokens[0].token_type == DecoratedTokenType::DecoratedKeyword {
            if tokens[0].keyword.unwrap() == KeywordType::KwReturn {
                let next_semicolon_pos = find_next_semicolon(tokens.clone());

                if next_semicolon_pos > 0 {
                    #[allow(unused_assignments)]
                        let mut result: Option<ReturnAction> = None;

                    if next_semicolon_pos == 1 {
                        // No return value
                        result = Option::from(ReturnAction {
                            value: Expression {
                                postfix_expr: vec![],
                                output_type: String::new()
                            }
                        });
                    } else {
                        // With return value
                        let expression_zone = tokens[1..(next_semicolon_pos as usize)].to_vec();

                        result = Option::from(ReturnAction {
                            value: Expression {
                                postfix_expr: expression_infix_to_postfix(expression_zone.clone()),
                                output_type: String::new()
                            }
                        });
                    }

                    return Ok((Action {
                        action_type: ActionType::ReturnStatement,
                        declaration_action: None,
                        assignment_action: None,
                        call_action: None,
                        return_action: result,
                        if_action: None,
                        while_action: None,
                        loop_action: None,
                        switch_action: None,
                    }, next_semicolon_pos as usize + 1));
                }
            }
        }
    }
    return Err(GeneralError{ code: "-1".to_string(), decription: None });
}
