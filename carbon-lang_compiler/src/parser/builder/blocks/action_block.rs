use crate::shared::ast::action::{Action, ActionType};
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::parser::builder::blocks::declaration::declaration_action_builder;
use crate::parser::builder::blocks::assignment::assignment_block_builder;
use crate::parser::builder::blocks::call::call_action_builder;
use crate::parser::builder::blocks::return_expression::return_action_builder;
use crate::parser::builder::blocks::short_actions::short_statements_builder;

// Lookup lexer/tokenize.rs
pub fn action_block_builder(mut tokens: Vec<DecoratedToken>) -> Vec<Action> {
    let mut result: Vec<Action> = Vec::new();

    while tokens.len() > 0 {
        let decl = declaration_action_builder(tokens.clone());
        if decl.1 != -1 {
            result.push(decl.0.unwrap());

            tokens = tokens[((decl.1 as usize) + 1)..].to_vec();
            continue;
        }

        let mut assign_action = assignment_block_builder(tokens.clone());
        if assign_action.1 != -1 {
            result.push(assign_action.0.unwrap());

            tokens = tokens[((assign_action.1 as usize) + 1)..].to_vec();
            continue;
        }

        let mut call_action = call_action_builder(tokens.clone());
        if call_action.1 > 0 {
            result.push(call_action.0.unwrap());

            tokens = tokens[((call_action.1 as usize) + 1)..].to_vec();
            continue;
        }

        let return_action = return_action_builder(tokens.clone());
        if return_action.1 > 0 {
            result.push(return_action.0.unwrap());

            tokens = tokens[((return_action.1 as usize) + 1)..].to_vec();
            continue;
        }

        let other_action = short_statements_builder(tokens.clone());
        if other_action.1 != -1 {
            result.push(other_action.0.unwrap());

            tokens = tokens[((return_action.1 as usize) + 1)..].to_vec();
            continue;
        }

        panic!("Invalid token stream encountered");
    }

    return result;
}
