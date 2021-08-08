use crate::parser::builder::blocks::link::link_statement_builder;
use crate::parser::builder::function_builder::function_builder;
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::ast::package::ParserPackageStructure;
use crate::shared::token::KeywordType;

pub fn build_whole_file(tokens: Vec<DecoratedToken>, entry_point: String) -> Option<ParserPackageStructure> {
    let mut result = ParserPackageStructure {
        functions: vec![],
        entry_point,
        linked_code_files: vec![],
    };

    // Build Link part
    let mut current_index = 0;
    loop {
        let current_link = link_statement_builder(tokens[current_index..].to_vec());
        if current_link.1 == -1 {
            break;
        }

        result.linked_code_files.push(current_link.0.unwrap());
        current_index += current_link.1 as usize + 1;
    }

    if tokens[current_index].token_type != DecoratedTokenType::DecoratedKeyword {
        panic!("Invalid token stream encountered!");
    } else {
        if tokens[current_index].keyword.unwrap() != KeywordType::KwDeclare {
            panic!("Invalid token stream encountered!");
        }
    }

    // Build Function part
    loop {
        // Break if current index points to the last element of the token stream
        if current_index == tokens.len() {
            break;
        }

        let current_function = function_builder(tokens[current_index..].to_vec());
        if current_function.1 == -1 {
            break;
        }

        result.functions.push(current_function.0.unwrap());
        current_index += current_function.1 as usize;
    }

    return Option::from(result);
}
