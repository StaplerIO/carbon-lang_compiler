use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::ast::action::{Action, ActionType, ActionBlock};
use crate::parser::utils::find_next_semicolon;
use crate::shared::token::{KeywordType, ContainerType};
use crate::parser::builder::blocks::action_block::action_block_builder;

// Build "continue", "break" and "loop" action
pub fn short_statements_builder(tokens: Vec<DecoratedToken>) -> (Option<Action>, isize) {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());

    if tokens[0].token_type == DecoratedTokenType::DecoratedKeyword {
        if next_semicolon_pos == 1 {
            let keyword = tokens[0].keyword.unwrap();

            // "break" or "continue"
            match keyword {
                KeywordType::KwContinue => {
                    return (Option::from(Action {
                        action_type: ActionType::ContinueStatement,
                        declaration_action: None,
                        assignment_action: None,
                        call_action: None,
                        return_action: None,
                        if_action: None,
                        while_action: None,
                        loop_action: None,
                        switch_action: None
                    }), next_semicolon_pos);
                }
                KeywordType::KwBreak => {
                    return (Option::from(Action {
                        action_type: ActionType::BreakStatement,
                        declaration_action: None,
                        assignment_action: None,
                        call_action: None,
                        return_action: None,
                        if_action: None,
                        while_action: None,
                        loop_action: None,
                        switch_action: None
                    }), next_semicolon_pos);
                }
                _ => {}
            }

            // Match a "loop" action
            if tokens[0].keyword.unwrap() == KeywordType::KwLoop &&
                tokens[1].token_type == DecoratedTokenType::Container &&
                tokens[next_semicolon_pos as usize - 1].token_type == DecoratedTokenType::Container {
                // The shortest token stream: loop { }
                // Check if the statement is lead by keyword "loop"
                if tokens[1].container.unwrap() == ContainerType::Brace &&
                    tokens[next_semicolon_pos as usize - 1].container.unwrap() == ContainerType::AntiBrace {
                    let container_content = tokens[2..(next_semicolon_pos as usize - 2)].to_vec();

                    return (Option::from(Action{
                        action_type: ActionType::LoopStatement,
                        declaration_action: None,
                        assignment_action: None,
                        call_action: None,
                        return_action: None,
                        if_action: None,
                        while_action: None,
                        loop_action: Option::from(ActionBlock { actions: action_block_builder(container_content.clone()) }),
                        switch_action: None
                    }), -1);
                }
            }
        }
    }

    return (None, -1);
}
