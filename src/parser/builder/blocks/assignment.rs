use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::parser::utils::find_next_semicolon;
use crate::shared::ast::action::{Action, AssignmentAction};
use crate::shared::ast::blocks::expression::Expression;
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::error::general_error::GeneralError;
use crate::shared::token::OperatorType;

pub fn assignment_block_builder(
    tokens: &Vec<DecoratedToken>,
) -> Result<(Action, usize), GeneralError<String>> {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());
    if next_semicolon_pos.is_some() {
        if tokens[0].is_valid_identifier() && tokens[1].token_type == DecoratedTokenType::Operator {
            if tokens[1].operator.unwrap().operator_type == OperatorType::Assignment {
                // Convert expression
                let postfix_expr = expression_infix_to_postfix(
                    tokens.clone()[2..next_semicolon_pos.unwrap()].to_vec(),
                );

                return Ok((
                    Action::new_assignment(AssignmentAction {
                        identifier: tokens[0]
                            .data
                            .clone()
                            .unwrap()
                            .clone()
                            .identifier
                            .unwrap()
                            .clone(),
                        eval_expression: Expression {
                            postfix_expr,
                            output_type: String::new(),
                        },
                    }),
                    next_semicolon_pos.unwrap() + 1,
                ));
            }
        }
    }

    return Err(GeneralError {
        code: "-1".to_string(),
        description: None,
    });
}
