use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::parser::utils::{find_next_semicolon, pair_container, split_comma_expression};
use crate::shared::ast::action::{Action, CallAction};
use crate::shared::ast::blocks::expression::Expression;
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::token::{ContainerType, KeywordType};
use crate::shared::error::general_error::GeneralError;

// Scheme: call <identifier>(<param list>);
pub fn call_action_builder(tokens: &Vec<DecoratedToken>) -> Result<(Action, usize), GeneralError<String>> {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());

    // Check format
    if next_semicolon_pos.unwrap_or(0) >= 4 {
        if tokens[0].token_type == DecoratedTokenType::DecoratedKeyword {
            if tokens[0].keyword.unwrap() == KeywordType::KwCall {
                let result = bare_function_call_builder(tokens[1..].to_vec());
                if result.is_ok() {
                    return Ok((Action::new_call(result.unwrap().0), next_semicolon_pos.unwrap() + 1));
                }
            }
        }
    }

    return Err(GeneralError{ code: "-1".to_string(), decription: None });
}

pub fn bare_function_call_builder(tokens: Vec<DecoratedToken>) -> Result<(CallAction, usize), GeneralError<String>> {
    if tokens.len() >= 3 {
        if tokens[0].is_valid_identifier() && tokens[1].token_type == DecoratedTokenType::Container {
            if tokens[1].container.unwrap() == ContainerType::Bracket {
                let mut result = CallAction {
                    function_name: tokens[0].clone().data.unwrap().identifier.unwrap().clone(),
                    arguments: vec![]
                };

                let parameter_zone = pair_container(tokens[1..].to_vec());
                for param in split_comma_expression(parameter_zone[1..parameter_zone.len()].to_vec()) {
                    result.arguments.push(Expression {
                        postfix_expr: expression_infix_to_postfix(param.clone()),
                        output_type: String::new()
                    });
                }

                return Ok((result, parameter_zone.len() + 2));
            }
        }
    }

    return Err(GeneralError{ code: "-1".to_string(), decription: None });
}
