use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::parser::utils::{find_next_semicolon, pair_container, split_comma_expression};
use crate::shared::ast::action::{Action, ActionType, CallAction};
use crate::shared::ast::blocks::expression::Expression;
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::token::{ContainerType, KeywordType};

// Scheme: call <identifier>(<param list>);
pub fn call_action_builder(tokens: Vec<DecoratedToken>) -> (Option<Action>, isize) {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());

    // Check format
    if next_semicolon_pos >= 4 {
        if tokens[0].token_type == DecoratedTokenType::DecoratedKeyword {
            if tokens[0].keyword.unwrap() == KeywordType::KwCall {
                let result = bare_function_call_builder(tokens[1..].to_vec());
                if result.1 != -1 {
                    return (Option::from(Action {
                        action_type: ActionType::CallStatement,
                        declaration_action: None,
                        assignment_action: None,
                        call_action: result.0,
                        return_action: None,
                        if_action: None,
                        while_action: None,
                        loop_action: None,
                        switch_action: None
                    }), next_semicolon_pos + 1);
                }
            }
        }
    }

    return (None, -1);
}

pub fn bare_function_call_builder(tokens: Vec<DecoratedToken>) -> (Option<CallAction>, isize) {
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

                return (Option::from(result), (parameter_zone.len() as isize) + 2);
            }
        }
    }

    return (None, -1);
}
