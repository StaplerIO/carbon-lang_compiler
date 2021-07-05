use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::ast::action::{IfAction, ActionBlock, ConditionBlock, Action, ActionType};
use crate::shared::token::{KeywordType, ContainerType};
use crate::parser::utils::pair_container;
use crate::shared::ast::blocks::expression::Expression;
use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::parser::builder::blocks::action_block::action_block_builder;

pub fn if_block_builder(tokens: Vec<DecoratedToken>) -> (Option<Action>, isize) {
    if tokens.len() > 6 && tokens[0].token_type == DecoratedTokenType::DecoratedKeyword {
        if tokens[0].keyword.unwrap() == KeywordType::KwIf {
            let mut result = IfAction {
                if_block: ConditionBlock {
                    condition: Expression { postfix_expr: vec![] },
                    body: ActionBlock { actions: vec![] }
                },
                elif_collection: vec![],
                else_action: None
            };

            if tokens[1].token_type == DecoratedTokenType::Container {
                if tokens[1].container.unwrap() == ContainerType::Bracket {
                    // Build `if` statement expression
                    let expression_zone = pair_container(tokens[1..].to_vec());
                    result.if_block.condition.postfix_expr = expression_infix_to_postfix(expression_zone[1..].to_vec());

                    if expression_zone.len() >= 1 {
                        // Build actions inside the statement
                        if tokens[expression_zone.len() + 2].token_type == DecoratedTokenType::Container {
                            if tokens[expression_zone.len() + 2].container.unwrap() == ContainerType::Brace {
                                let action_zone = pair_container(tokens[(expression_zone.len() + 2)..].to_vec());
                                result.if_block.body.actions = action_block_builder(action_zone[1..action_zone.len()].to_vec());

                                // If there's any `elif` or `else` blocks

                                // current_index is the end index of the `if` statement we serialized just now
                                let mut current_index: usize = expression_zone.len() + action_zone.len() + 3;
                                // Check `elif`
                                loop {
                                    let elif_block = detached_elif_block_builder(tokens[current_index..].to_vec());
                                    if elif_block.1 != -1 {
                                        result.elif_collection.push(elif_block.0.unwrap());
                                        current_index += elif_block.1 as usize;
                                    } else {
                                        break;
                                    }
                                }

                                // Check `else`
                                let else_block = detached_else_block_builder(tokens[current_index..].to_vec());
                                if else_block.1 != -1 {
                                    result.else_action = else_block.0;
                                    current_index += else_block.1 as usize;
                                }

                                return (Option::from(Action {
                                    action_type: ActionType::IfStatement,
                                    declaration_action: None,
                                    assignment_action: None,
                                    call_action: None,
                                    return_action: None,
                                    if_action: Option::from(result),
                                    while_action: None,
                                    loop_action: None,
                                    switch_action: None
                                }), current_index as isize);
                            }
                        }
                    }
                }
            }
        }
    }

    return (None, -1);
}

// `elif` block must be a sub-node of `if` block, so this is a private method
fn detached_elif_block_builder(tokens: Vec<DecoratedToken>) -> (Option<ConditionBlock>, isize) {
    if tokens.len() > 6 && tokens[0].token_type == DecoratedTokenType::DecoratedKeyword {
        if tokens[0].keyword.unwrap() == KeywordType::KwElseIf {
            let mut result = ConditionBlock {
                condition: Expression { postfix_expr: vec![] },
                body: ActionBlock { actions: vec![] }
            };

            if tokens[1].token_type == DecoratedTokenType::Container {
                if tokens[1].container.unwrap() == ContainerType::Bracket {
                    // Build `if` statement expression
                    let expression_zone = pair_container(tokens[1..].to_vec());
                    result.condition.postfix_expr = expression_infix_to_postfix(expression_zone.clone());

                    if expression_zone.len() >= 1 {
                        // Build actions inside the statement
                        if tokens[expression_zone.len() + 2].token_type == DecoratedTokenType::Container {
                            if tokens[expression_zone.len() + 2].container.unwrap() == ContainerType::Brace {
                                let action_block_zone = pair_container(tokens[(expression_zone.len() + 2)..].to_vec());
                                result.body.actions = action_block_builder(action_block_zone[1..action_block_zone.len()].to_vec());

                                return (Option::from(result), (expression_zone.len() + action_block_zone.len() + 3) as isize);
                            }
                        }
                    }
                }
            }
        }
    }

    return (None, -1);
}

// `else` block must be a sub-node of `if` block, so this is a private method
fn detached_else_block_builder(tokens: Vec<DecoratedToken>) -> (Option<ActionBlock>, isize) {
    if tokens.len() > 6 && tokens[0].token_type == DecoratedTokenType::DecoratedKeyword {
        if tokens[0].keyword.unwrap() == KeywordType::KwElse {
            let mut result = ActionBlock {
                actions: vec![]
            };

            // Build actions inside the statement
            if tokens[1].token_type == DecoratedTokenType::Container {
                if tokens[1].container.unwrap() == ContainerType::Brace {
                    let action_block_zone = pair_container(tokens[1..].to_vec());
                    result.actions = action_block_builder(action_block_zone[1..action_block_zone.len()].to_vec());

                    return (Option::from(result), (action_block_zone.len() + 1) as isize);
                }
            }
        }
    }

    return (None, -1);
}
