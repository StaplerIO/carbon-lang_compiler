use crate::shared::ast::action::CallAction;
use crate::parser::utils::{find_next_semicolon, find_next_comma, split_comma_expression};
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::token::{KeywordType, ContainerType};
use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::shared::ast::blocks::expression::Expression;

// Scheme: call <identifier>(<param list>);
pub fn call_function(tokens: Vec<DecoratedToken>) -> (Option<CallAction>, usize) {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());

    // Check format
    if next_semicolon_pos >= 4 {
        if tokens[0].token_type == DecoratedTokenType::DecoratedKeyword &&
            tokens[1].is_valid_identifier() &&
            tokens[2].token_type == DecoratedTokenType::Container &&
            tokens[(next_semicolon_pos as usize) - 1].token_type == DecoratedTokenType::Container {
            if tokens[0].keyword.unwrap() == KeywordType::KwCall &&
                tokens[2].container.unwrap() == ContainerType::Bracket &&
                tokens[(next_semicolon_pos as usize) - 1].container.unwrap() == ContainerType::AntiBracket {
                let mut result = CallAction {
                    function_name: tokens[1].clone().data.unwrap().identifier.unwrap().clone(),
                    arguments: vec![]
                };

                // The parameter list that needs to be passed into the function
                let parameter_zone = tokens[3..(next_semicolon_pos as usize) - 1].to_vec().clone();

                // Convert and add them into the parameter list
                for param in split_comma_expression(parameter_zone.clone()) {
                    result.arguments.push(Expression { postfix_expr: expression_infix_to_postfix(param.clone()) });
                }

                return (Option::from(result), next_semicolon_pos as usize);
            }
        }
    }

    return (None, 0);
}
