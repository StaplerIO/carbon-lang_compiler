use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::ast::action::{Action, ActionType, ActionBlock};
use crate::parser::utils::find_next_semicolon;
use crate::shared::token::{KeywordType, ContainerType};
use crate::parser::builder::blocks::action_block::action_block_builder;
use crate::shared::error::general_error::GeneralError;

// Build "continue", "break" and "loop" action
pub fn short_statements_builder(tokens: Vec<DecoratedToken>) -> Result<(Action, usize), GeneralError<String>> {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());

    if tokens[0].token_type == DecoratedTokenType::DecoratedKeyword {
        if next_semicolon_pos.unwrap_or(0) == 1 {
            let keyword = tokens[0].keyword.unwrap();

            // "break" or "continue"
            match keyword {
                KeywordType::KwContinue => {
                    return Ok((Action {
                        action_type: ActionType::ContinueStatement,
                        declaration_action: None,
                        assignment_action: None,
                        call_action: None,
                        return_action: None,
                        if_action: None,
                        while_action: None,
                        loop_action: None,
                        switch_action: None
                    }, next_semicolon_pos.unwrap() + 1));
                }
                KeywordType::KwBreak => {
                    return Ok((Action {
                        action_type: ActionType::BreakStatement,
                        declaration_action: None,
                        assignment_action: None,
                        call_action: None,
                        return_action: None,
                        if_action: None,
                        while_action: None,
                        loop_action: None,
                        switch_action: None
                    }, next_semicolon_pos.unwrap() + 1));
                }
                _ => {}
            }

            // Match a "loop" action
            if tokens[0].keyword.unwrap() == KeywordType::KwLoop &&
                tokens[1].token_type == DecoratedTokenType::Container &&
                tokens[next_semicolon_pos.unwrap() - 1].token_type == DecoratedTokenType::Container {
                // The shortest token stream: loop { }
                // Check if the statement is lead by keyword "loop"
                if tokens[1].container.unwrap() == ContainerType::Brace &&
                    tokens[next_semicolon_pos.unwrap() - 1].container.unwrap() == ContainerType::AntiBrace {
                    let container_content = tokens[2..(next_semicolon_pos.unwrap() - 2)].to_vec();

                    return Ok((Action{
                        action_type: ActionType::LoopStatement,
                        declaration_action: None,
                        assignment_action: None,
                        call_action: None,
                        return_action: None,
                        if_action: None,
                        while_action: None,
                        loop_action: Option::from(ActionBlock { actions: action_block_builder(container_content.clone()) }),
                        switch_action: None
                    }, 0));
                }
            }
        }
    }

    return Err(GeneralError{ code: "-1".to_string(), decription: None });
}
