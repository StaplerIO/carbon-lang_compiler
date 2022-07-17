use crate::parser::builder::blocks::assignment::assignment_block_builder;
use crate::parser::builder::blocks::call::call_action_builder;
use crate::parser::builder::blocks::condition::if_block_builder;
use crate::parser::builder::blocks::declaration::declaration_action_builder;
use crate::parser::builder::blocks::loops::while_action_builder;
use crate::parser::builder::blocks::return_expression::return_action_builder;
use crate::parser::builder::blocks::short_actions::short_statements_builder;
use crate::shared::ast::action::Action;
use crate::shared::ast::decorated_token::DecoratedToken;

// Lookup lexer/tokenize.rs
pub fn action_block_builder(mut tokens: Vec<DecoratedToken>) -> Vec<Action> {
    let mut result: Vec<Action> = Vec::new();

    // Needs to be simplified
    while tokens.len() > 0 {
        let decl = declaration_action_builder(&tokens.clone());
        if decl.is_ok() {
            result.push(decl.clone().ok().unwrap().0);

            tokens = tokens[decl.ok().unwrap().1..].to_vec();
            continue;
        }

        let assign_action = assignment_block_builder(&tokens.clone());
        if assign_action.is_ok() {
            result.push(assign_action.clone().ok().unwrap().0);

            tokens = tokens[assign_action.ok().unwrap().1..].to_vec();
            continue;
        }

        let call_action = call_action_builder(&tokens.clone());
        if call_action.is_ok() {
            result.push(call_action.clone().ok().unwrap().0);

            tokens = tokens[call_action.ok().unwrap().1..].to_vec();
            continue;
        }

        let return_action = return_action_builder(&tokens.clone());
        if return_action.is_ok() {
            result.push(return_action.clone().ok().unwrap().0);

            tokens = tokens[return_action.ok().unwrap().1..].to_vec();
            continue;
        }

        let if_action = if_block_builder(&tokens.clone());
        if if_action.is_ok() {
            result.push(if_action.clone().ok().unwrap().0);

            tokens = tokens[if_action.ok().unwrap().1..].to_vec();
            continue;
        }

        let while_action = while_action_builder(&tokens.clone());
        if while_action.is_ok() {
            result.push(while_action.clone().ok().unwrap().0);

            tokens = tokens[while_action.ok().unwrap().1..].to_vec();
            continue;
        }

        let other_action = short_statements_builder(&tokens.clone());
        if other_action.is_ok() {
            result.push(other_action.clone().ok().unwrap().0);

            tokens = tokens[other_action.ok().unwrap().1..].to_vec();
            continue;
        }

        panic!("Invalid token stream encountered, position: {}, actions built: {}", tokens[0].original_token.position.start, result.len());
    }

    return result;
}
