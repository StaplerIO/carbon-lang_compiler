use crate::parser::builder::expression_builder::{expression_infix_to_postfix, expression_term_decorator};
use crate::parser::utils::find_next_semicolon;
use crate::shared::ast::action::{Action, ActionContent, ReturnAction};
use crate::shared::ast::blocks::expression::SimpleExpression;
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::error::general_issue::{GeneralIssue, IssueLevel};
use crate::shared::token::keyword::KeywordType;

pub fn return_action_builder(
    tokens: &Vec<DecoratedToken>,
) -> Result<(Action, usize), GeneralIssue<String>> {
    // Minimum: return ; (2 tokens in total)
    if tokens.len() >= 2 {
        if tokens[0].content.get_decorated_keyword().is_some() {
            if *tokens[0].content.get_decorated_keyword().unwrap() == KeywordType::KwReturn {
                let next_semicolon_pos = find_next_semicolon(tokens.clone());

                if next_semicolon_pos.unwrap_or(0) > 0 {
                    #[allow(unused_assignments)]
                    let mut result: Option<ReturnAction> = None;

                    if next_semicolon_pos.unwrap() == 1 {
                        // No return value
                        result = Option::from(ReturnAction {
                            value: Some(SimpleExpression {
                                postfix_expr: vec![],
                                output_type: String::new(),
                            }),
                        });
                    } else {
                        // With return value
                        let expression_zone = tokens[1..next_semicolon_pos.unwrap()].to_vec();

                        result = Option::from(ReturnAction {
                            value: Some(SimpleExpression {
                                postfix_expr: expression_infix_to_postfix(expression_term_decorator(expression_zone.clone())),
                                output_type: String::new(),
                            }),
                        });
                    }

                    return Ok((
                        Action::new(ActionContent::ReturnStatement(result.unwrap()), vec![]),
                        next_semicolon_pos.unwrap() + 1,
                    ));
                }
            }
        }
    }
    return Err(GeneralIssue {
        level: IssueLevel::Error,
        code: "-1".to_string(),
        description: String::new(),
    });
}
