use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::token::KeywordType;
use crate::parser::builder::templates::condition_block_builder;
use crate::shared::ast::action::{Action, ActionType};
use crate::shared::error::general_error::GeneralError;

// result.1 : The end of the while statement (the last anti-brace)
pub fn while_action_builder(tokens: Vec<DecoratedToken>) -> Result<(Action, usize), GeneralError<String>> {
    let result = condition_block_builder(KeywordType::KwWhile, tokens.clone());
    if result.is_ok() {
        return Ok((Action{
            action_type: ActionType::WhileStatement,
            declaration_action: None,
            assignment_action: None,
            call_action: None,
            return_action: None,
            if_action: None,
            while_action: Option::from(result.clone().ok().unwrap().0),
            loop_action: None,
            switch_action: None
        }, result.ok().unwrap().1));
    }

    return Err(GeneralError{ code: "-1".to_string(), decription: None });
}

// Action of keyword "loop" is in file "./short_actions.rs"
