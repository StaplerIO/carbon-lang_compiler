use crate::parser::builder::blocks::link::link_statement_builder;
use crate::parser::builder::function_builder::function_builder;
use crate::parser::builder::group::declaration::group_declaration_builder;
use crate::parser::builder::group::implementation::group_implementation_builder;
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::ast::package::ParserPackageStructure;
use crate::shared::error::general_issue::{GeneralIssue, IssueBase, IssueLevel, IssuePosition};
use crate::shared::token::keyword::KeywordType;
use crate::shared::utils::identifier::Identifier;

pub fn build_whole_file(
    tokens: Vec<DecoratedToken>,
    entry_point: Identifier,
) -> Result<ParserPackageStructure, GeneralIssue<String>> {
    let mut result = ParserPackageStructure {
        functions: vec![],
        entry_point,
        linked_code_files: vec![],
        declared_groups: vec![],
        declared_implementations: vec![],
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

    // Build Function or group part
    while current_index < tokens.len() {
        match tokens[current_index].content.get_decorated_keyword().unwrap() {
            KeywordType::KwDeclare => {
                let current_function = function_builder(&tokens[current_index..].to_vec());
                if current_function.is_err() {
                    break;
                }

                result.functions
                      .push(current_function.clone().ok().unwrap().0);
                current_index += current_function.ok().unwrap().1;
            }
            KeywordType::KwGroup => {
                let current_group = group_declaration_builder(&tokens[current_index..].to_vec());
                if current_group.is_err() {
                    break;
                }

                result.declared_groups
                      .push(current_group.clone().ok().unwrap().0);
                current_index += current_group.ok().unwrap().1;
            }
            KeywordType::KwImplement => {
                let current_group = group_implementation_builder(&tokens[current_index..].to_vec(), &result.declared_groups);
                if current_group.is_err() {
                    break;
                }

                result.declared_implementations
                      .push(current_group.clone().ok().unwrap().0);
                current_index += current_group.ok().unwrap().1;
            }
            _ => {
                return Err(GeneralIssue {
                    issues: vec![IssueBase {
                        level: IssueLevel::Info,
                        position: IssuePosition::Parsing,
                        code: "-1".to_string(),
                        detail: "Invalid token stream encountered!".to_string(),
                    }]
                });
            }
        }
    }

    return Ok(result);
}
