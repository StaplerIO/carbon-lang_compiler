use crate::parser::builder::blocks::action_block::action_block_builder;
use crate::parser::builder::templates::condition_block_builder;
use crate::parser::utils::pair_container;
use crate::shared::ast::action::{Action, ActionBlock, ElifBlock, IfAction};
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::error::general_error::GeneralError;
use crate::shared::token::{ContainerType, KeywordType};

pub fn if_block_builder(
    tokens: &Vec<DecoratedToken>,
) -> Result<(Action, usize), GeneralError<String>> {
    let if_part = condition_block_builder(KeywordType::KwIf, tokens.clone());
    if if_part.is_ok() {
        let mut result = IfAction {
            if_block: if_part.clone().ok().unwrap().0,
            elif_collection: vec![],
            else_action: None,
        };

        // Then we have an if_block (only), we'll try to find elif and else right after it
        let mut current_index: usize = if_part.clone().ok().unwrap().1 as usize;
        loop {
            let elif_part = detached_elif_block_builder(tokens[current_index..].to_vec());
            if elif_part.1 != -1 {
                result.elif_collection.push(elif_part.0.unwrap());
                current_index += elif_part.1 as usize;
            } else {
                break;
            }
        }

        let else_part = detached_else_block_builder(tokens[current_index..].to_vec());
        if else_part.1 != -1 {
            result.else_action = else_part.0;
            current_index += else_part.1 as usize;
        }

        return Ok((Action::new_if(result), current_index));
    }

    return Err(GeneralError {
        code: "-1".to_string(),
        description: None,
    });
}

// `elif` block must be a sub-node of `if` block, so this is a private method
// Return -1 if there's a problem while building elif block
fn detached_elif_block_builder(tokens: Vec<DecoratedToken>) -> (Option<ElifBlock>, isize) {
    let result = condition_block_builder(KeywordType::KwElseIf, tokens.clone());
    if result.is_ok() {
        return (
            Option::from(result.clone().ok().unwrap().0),
            result.clone().ok().unwrap().1 as isize,
        );
    }

    return (None, -1);
}

// `else` block must be a sub-node of `if` block, so this is a private method
fn detached_else_block_builder(tokens: Vec<DecoratedToken>) -> (Option<ActionBlock>, isize) {
    // Shortest: `else { }`
    if tokens.len() >= 3 && tokens[0].token_type == DecoratedTokenType::DecoratedKeyword {
        if tokens[0].keyword.unwrap() == KeywordType::KwElse {
            let mut result = ActionBlock { actions: vec![] };

            // Build actions inside the statement
            if tokens[1].token_type == DecoratedTokenType::Container {
                if tokens[1].container.unwrap() == ContainerType::Brace {
                    let action_block_zone = pair_container(tokens[1..].to_vec());
                    result.actions = action_block_builder(
                        action_block_zone[1..action_block_zone.len()].to_vec(),
                    );

                    return (Option::from(result), (action_block_zone.len() + 1) as isize);
                }
            }
        }
    }

    return (None, -1);
}
