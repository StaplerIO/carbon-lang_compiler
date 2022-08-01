use crate::parser::builder::expression_builder::{expression_infix_to_postfix, expression_term_decorator};
use crate::parser::utils::{find_next_semicolon, pair_container, split_comma_expression};
use crate::shared::ast::action::{Action, ActionContent, CallAction};
use crate::shared::ast::blocks::expression::SimpleExpression;
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::error::general_issue::{GeneralIssue, IssueBase, IssueLevel, IssuePosition};
use crate::shared::token::container::ContainerType;
use crate::shared::token::keyword::KeywordType;

// Scheme: call <identifier>(<param list>);
pub fn call_action_builder(
    tokens: &Vec<DecoratedToken>,
) -> Result<(Action, usize), GeneralIssue<String>> {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());

    // Check format
    if next_semicolon_pos.unwrap_or(0) >= 4 {
        if tokens[0].content.get_decorated_keyword().is_some() {
            if *tokens[0].content.get_decorated_keyword().unwrap() == KeywordType::KwCall {
                let result = bare_function_call_builder(tokens[1..].to_vec());
                if result.is_ok() {
                    return Ok((
                        Action::new(ActionContent::CallStatement(result.unwrap().0), vec![]),
                        next_semicolon_pos.unwrap() + 1,
                    ));
                }
            }
        }
    }

    return Err(GeneralIssue {
        issues: vec![IssueBase {
            level: IssueLevel::Info,
            position: IssuePosition::Parsing,
            code: "".to_string(),
            detail: "".to_string(),
        }]
    });
}

pub fn bare_function_call_builder(
    tokens: Vec<DecoratedToken>,
) -> Result<(CallAction, usize), GeneralIssue<String>> {
    if tokens.len() >= 3 {
        if tokens[0].content.is_valid_identifier() && tokens[1].content.get_container().is_some()
        {
            if *tokens[1].content.get_container().unwrap() == ContainerType::Bracket {
                let mut result = CallAction {
                    function_name: tokens[0].content.get_data().unwrap().get_identifier().unwrap().clone(),
                    arguments: vec![],
                };

                let parameter_zone = pair_container(tokens[1..].to_vec());
                for param in
                    split_comma_expression(parameter_zone[1..parameter_zone.len()].to_vec())
                {
                    result.arguments.push(SimpleExpression {
                        postfix_expr: expression_infix_to_postfix(expression_term_decorator(&param)),
                        output_type: String::new(),
                    });
                }

                return Ok((result, parameter_zone.len() + 2));
            }
        }
    }

    return Err(GeneralIssue {
        issues: vec![IssueBase {
            level: IssueLevel::Info,
            position: IssuePosition::Parsing,
            code: "".to_string(),
            detail: "".to_string(),
        }]
    });
}
