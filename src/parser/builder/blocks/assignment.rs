use crate::parser::builder::expression_builder::{expression_infix_to_postfix, expression_term_decorator};
use crate::parser::utils::find_next_semicolon;
use crate::shared::ast::action::{Action, ActionContent, AssignmentAction};
use crate::shared::ast::blocks::expression::SimpleExpression;
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::error::general_error::GeneralError;
use crate::shared::token::operator::Operator;

pub fn assignment_block_builder(
    tokens: &Vec<DecoratedToken>,
) -> Result<(Action, usize), GeneralError<String>> {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());
    if next_semicolon_pos.is_some() {
        if tokens[0].is_valid_identifier() && tokens[1].token_type == DecoratedTokenType::Operator {
            if tokens[1].operator.unwrap() == Operator::Assignment {
                // Convert expression
                let postfix_expr = expression_infix_to_postfix(
                    expression_term_decorator(tokens.clone()[2..next_semicolon_pos.unwrap()].to_vec()),
                );

                return Ok((
                    Action::new(ActionContent::AssignmentStatement(AssignmentAction {
                        identifier: tokens[0].data.clone().unwrap().identifier.unwrap(),
                        eval_expression: SimpleExpression {
                            postfix_expr,
                            output_type: "".to_string()
                        }
                    }), vec![]),
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
