use crate::parser::builder::blocks::link::link_statement_builder;
use crate::parser::builder::function_builder::function_builder;
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::ast::package::ParserPackageStructure;
use crate::shared::error::general_issue::{GeneralIssue, IssueLevel};
use crate::shared::token::keyword::KeywordType;

pub fn build_whole_file(
    tokens: Vec<DecoratedToken>,
    entry_point: String,
) -> Result<ParserPackageStructure, GeneralIssue<String>> {
    let mut result = ParserPackageStructure {
        functions: vec![],
        entry_point,
        linked_code_files: vec![],
    };

    // Build Link part
    let mut current_index = 0;
    loop {
        let current_link = link_statement_builder(&tokens[current_index..].to_vec());
        if current_link.is_err() {
            break;
        }

        result
            .linked_code_files
            .push(current_link.clone().ok().unwrap().0);
        current_index += current_link.ok().unwrap().1 + 1;
    }

    if tokens[current_index].content.get_decorated_keyword().is_none() {
        return Err(GeneralIssue {
            level: IssueLevel::Error,
            code: "-1".to_string(),
            description:"Invalid token stream encountered!".to_string(),
        });
    } else {
        if *tokens[current_index].content.get_decorated_keyword().unwrap() != KeywordType::KwDeclare {
            return Err(GeneralIssue {
                level: IssueLevel::Error,
                code: "-1".to_string(),
                description: "Invalid token stream encountered!".to_string(),
            });
        }
    }

    // Build Function part
    loop {
        // Break if current index points to the last element of the token stream
        if current_index == tokens.len() {
            break;
        }

        let current_function = function_builder(&tokens[current_index..].to_vec());
        if current_function.is_err() {
            break;
        }

        result
            .functions
            .push(current_function.clone().ok().unwrap().0);
        current_index += current_function.ok().unwrap().1;
    }

    return Ok(result);
}
