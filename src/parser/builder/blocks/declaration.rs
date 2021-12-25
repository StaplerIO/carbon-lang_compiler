use crate::parser::utils::find_next_semicolon;
use crate::shared::ast::action::{Action, DeclarationAction};
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::error::general_error::GeneralError;
use crate::shared::token::KeywordType;

pub fn declaration_action_builder(
    tokens: &Vec<DecoratedToken>,
) -> Result<(Action, usize), GeneralError<String>> {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());
    // Each block owns 4 tokens only
    if next_semicolon_pos.unwrap_or(0) == 4 {
        if tokens[0].token_type == DecoratedTokenType::DecoratedKeyword
            && tokens[1].token_type == DecoratedTokenType::DecoratedKeyword
            && tokens[2].is_valid_type()
            && tokens[3].is_valid_identifier()
        {
            // Match declaration statement format: decl <var|const> <data_type> <identifier>

            let mut result = DeclarationAction {
                is_variable: false,
                identifier: tokens[3].clone().data.unwrap().identifier.unwrap().clone(),
                data_type: tokens[2].clone().data.unwrap().type_name.unwrap().clone(),
            };

            // Lead the Declaration statement
            if tokens[0].keyword.unwrap() == KeywordType::KwDeclare {
                if tokens[1].keyword.unwrap() == KeywordType::KwVar {
                    result.is_variable = true;
                } else if tokens[1].keyword.unwrap() == KeywordType::KwConst {
                    result.is_variable = false;
                } else {
                    return Err(GeneralError {
                        code: "-1".to_string(),
                        description: Option::from(
                            "Require keyword \'var\' or \'const\'".to_string(),
                        ),
                    });
                }

                return Ok((Action::new_decl(result), next_semicolon_pos.unwrap() + 1));
            }
        }
    }

    return Err(GeneralError {
        code: "-1".to_string(),
        description: None,
    });
}
