use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::ast::action::{ConditionBlock, ActionBlock};
use crate::shared::token::{KeywordType, ContainerType};
use crate::parser::utils::pair_container;
use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::shared::ast::blocks::expression::Expression;
use crate::parser::builder::blocks::action_block::action_block_builder;

// result.1 : The end of the while statement (the last anti-brace)
pub fn while_statement_builder(tokens: Vec<DecoratedToken>) -> (Option<ConditionBlock>, isize) {
    // while ( expr ) { } <-- 6 tokens in total
    if tokens.len() > 6 && tokens.first().unwrap().token_type == DecoratedTokenType::DecoratedKeyword {
        if tokens.first().unwrap().keyword.unwrap() == KeywordType::KwWhile {
            let mut result = ConditionBlock {
                condition: Expression { postfix_expr: vec![] },
                body: ActionBlock { actions: vec![] }
            };

            // An expression is required next, bracketed
            if tokens[1].token_type == DecoratedTokenType::Container {
                if tokens[1].container.unwrap() == ContainerType::Bracket {
                    // Build expression
                    let expression_zone = pair_container(tokens[1..].to_vec());
                    result.condition.postfix_expr = expression_infix_to_postfix(expression_zone[1..].to_vec());

                    // Build actions inside the while statement
                    // expression_zone.len() + 2 --> Add brackets before and after
                    if tokens[expression_zone.len() + 2].token_type == DecoratedTokenType::Container {
                        if tokens[expression_zone.len() + 2].container.unwrap() == ContainerType::Brace {
                            let mut action_block_zone = pair_container(tokens[(expression_zone.len() + 2)..].to_vec());
                            // Remove the first and the last element
                            action_block_zone = action_block_zone[1..(action_block_zone.len() /*- 1*/)].to_vec();

                            // Build actions
                            result.body.actions = action_block_builder(action_block_zone.clone());

                            return (Option::from(result), (expression_zone.len() + action_block_zone.len()) as isize);
                        }
                    }
                }
            }
        }
    }

    return (None, -1);
}
