use crate::shared::token::KeywordType;
use crate::shared::ast::action::DeclarationAction;
use crate::parser::utils::find_next_semicolon;
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};

pub fn declare_data(tokens: Vec<DecoratedToken>) -> (Option<DeclarationAction>, isize) {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());
    // Each block owns 4 tokens only
    if next_semicolon_pos == 4 {
        if tokens[0].token_type == DecoratedTokenType::DecoratedKeyword &&
            tokens[1].token_type == DecoratedTokenType::DecoratedKeyword &&
            tokens[2].is_valid_type() &&
            tokens[3].is_valid_identifier() {
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
                    panic!("Require keyword \'var\' or \'const\'");
                }

                return (Option::from(result), next_semicolon_pos);
            }
        }
    }

    return (None, -1);
}