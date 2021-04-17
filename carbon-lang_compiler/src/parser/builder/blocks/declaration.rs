use crate::shared::token::{Token, TokenType, KeywordType};
use crate::shared::ast::action::DeclarationAction;
use crate::parser::utils::find_next_semicolon;

pub fn declare_data(tokens: Vec<Token>) -> (Option<DeclarationAction>, isize) {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());
    // Each block owns 4 tokens only
    if next_semicolon_pos == 4 {
        if tokens[0].token_type == TokenType::Keyword &&
            tokens[1].token_type == TokenType::Keyword &&
            tokens[2].token_type == TokenType::Keyword &&
            tokens[3].token_type == TokenType::Identifier {
            // Match declaration statement format: decl <var|const> <data_type> <identifier>

            let mut result = DeclarationAction {
                is_variable: false,
                identifier: tokens[3].clone().identifier.unwrap().clone(),
                data_type: "".to_string(),
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

                // We support 4 main data types only for now: int, decimal, char and string
                if tokens[2].keyword.unwrap() == KeywordType::KwInt {
                    result.data_type = String::from("int");
                } else if tokens[2].keyword.unwrap() == KeywordType::KwDecimal {
                    result.data_type = String::from("decimal");
                } else if tokens[2].keyword.unwrap() == KeywordType::KwChar {
                    result.data_type = String::from("char");
                } else if tokens[2].keyword.unwrap() == KeywordType::KwStr {
                    result.data_type = String::from("str");
                } else {
                    panic!("Illegal data type encountered!");
                }

                return (Option::from(result), next_semicolon_pos);
            }
        }
    }

    return (None, -1);
}
