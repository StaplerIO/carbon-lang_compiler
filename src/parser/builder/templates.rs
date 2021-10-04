use crate::parser::builder::blocks::action_block::action_block_builder;
use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::parser::utils::pair_container;
use crate::shared::ast::action::{ActionBlock, ConditionBlock};
use crate::shared::ast::blocks::expression::Expression;
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::token::{ContainerType, KeywordType};
use crate::shared::error::general_error::GeneralError;

/// A `ConditionBlock` has 2 parts: `Expression` part and `ActionBlock` part, formatted like this:
/// ` leading_keyword (expression) { action_block }`
/// The function will search the `leading_keyword`, and match `Expression` and `ActionBlock` contained separately
/// About return value: The return value is the length of the whole `ConditionBlock`, pointed to the next element of the token array just after the `ConditionBlock` ends
pub fn condition_block_builder(leading_keyword: KeywordType, tokens: Vec<DecoratedToken>) -> Result<(ConditionBlock, usize), GeneralError<String>> {
    if tokens.len() > 6 && tokens[0].token_type == DecoratedTokenType::DecoratedKeyword {
        if tokens.first().unwrap().keyword.unwrap() == leading_keyword {
            let mut result = ConditionBlock {
                condition: Expression {
                    postfix_expr: vec![],
                    output_type: String::new()
                },
                body: ActionBlock { actions: vec![] }
            };

            // An expression is required next, bracketed
            if tokens[1].token_type == DecoratedTokenType::Container {
                if tokens[1].container.unwrap() == ContainerType::Bracket {
                    // Build expression
                    let expression_zone = pair_container(tokens[1..].to_vec());
                    result.condition.postfix_expr = expression_infix_to_postfix(expression_zone[1..].to_vec());

                    if expression_zone.len() >= 1 {
                        // Build actions inside the while statement
                        // expression_zone.len() + 2 --> Add brackets before and after
                        if tokens[expression_zone.len() + 2].token_type == DecoratedTokenType::Container {
                            if tokens[expression_zone.len() + 2].container.unwrap() == ContainerType::Brace {
                                let mut action_block_zone = pair_container(tokens[(expression_zone.len() + 2)..].to_vec());
                                // Remove the first and the last element
                                action_block_zone = action_block_zone[1..(action_block_zone.len() /*- 1*/)].to_vec();

                                // Build actions
                                result.body.actions = action_block_builder(action_block_zone.clone());

                                // `4` is like a magic number, do not try to change it unless you can explain it in a better way
                                return Ok((result, expression_zone.len() + action_block_zone.len() + 4));
                            }
                        }
                    }
                }
            }
        }
    }

    return Err(GeneralError{ code: "-1".to_string(), description: None });
}
