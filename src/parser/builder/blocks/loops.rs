use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::token::KeywordType;
use crate::parser::builder::templates::condition_block_builder;
use crate::shared::ast::action::{Action, ActionType};

// result.1 : The end of the while statement (the last anti-brace)
pub fn while_action_builder(tokens: Vec<DecoratedToken>) -> (Option<Action>, isize) {
    let result = condition_block_builder(KeywordType::KwWhile, tokens.clone());
    if result.1 != -1 {
        return (Option::from(Action{
            action_type: ActionType::WhileStatement,
            declaration_action: None,
            assignment_action: None,
            call_action: None,
            return_action: None,
            if_action: None,
            while_action: result.0,
            loop_action: None,
            switch_action: None
        }), result.1);
    }

    return (None, -1);
}

// Action of keyword "loop" is in file "./short_actions.rs"
