use crate::parser::utils::find_next_semicolon;
use crate::shared::ast::action::{Action, ActionContent, DeclarationAction};
use crate::shared::ast::blocks::data::DataDeclarator;
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::error::general_issue::{GeneralIssue, IssueBase, IssueLevel, IssuePosition};
use crate::shared::token::container::ContainerType;
use crate::shared::token::keyword::KeywordType;
use crate::shared::utils::identifier::Identifier;

pub fn declaration_action_builder(
    tokens: &Vec<DecoratedToken>,
) -> Result<(Action, usize), GeneralIssue<String>> {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());
    if tokens[0].content.get_decorated_keyword().unwrap_or(&KeywordType::Invalid) == &KeywordType::KwDeclare
        && tokens[1].content.get_decorated_keyword().is_some()
        && tokens[2].content.is_valid_identifier()
    {
        // Match declaration statement format: decl <var|const> <data_type>([] for array) <identifier>
        let mut result = DeclarationAction {
            is_variable: false,
            declarator: DataDeclarator {
                data_type: Identifier::empty(),
                identifier: Identifier::empty(),
                is_array: false,
            },
        };

        // Check whether it is a constant or variable
        if *tokens[1].content.get_decorated_keyword().unwrap() == KeywordType::KwVar {
            result.is_variable = true;
        } else if *tokens[1].content.get_decorated_keyword().unwrap() == KeywordType::KwConst {
            result.is_variable = false;
        } else {
            return Err(GeneralIssue {
                issues: vec![IssueBase {
                    level: IssueLevel::Info,
                    position: IssuePosition::Parsing,
                    code: "".to_string(),
                    detail: "Couldn't determine whether this is a variable or a constant".to_string(),
                }]
            });
        }

        let mut current_index = 1;

        let data_type_result = Identifier::from_tokens(&tokens[2..].to_vec());
        if data_type_result.is_none() {
            return Err(GeneralIssue {
                issues: vec![IssueBase {
                    level: IssueLevel::Info,
                    position: IssuePosition::Parsing,
                    code: "".to_string(),
                    detail: "Valid data type required".to_string(),
                }]
            });
        }

        let data_type = data_type_result.unwrap();
        result.declarator.data_type = data_type.0;
        current_index += data_type.1;

        // For array declarations
        if tokens[current_index].original_token.get_container().unwrap_or(ContainerType::Invalid) == ContainerType::Index
            && tokens[current_index + 1].original_token.get_container().unwrap_or(ContainerType::Invalid) == ContainerType::AntiIndex {
            result.declarator.is_array = true;
            current_index += 2;
        }

        // Build identifier
        let identifier_result = Identifier::from_tokens(&tokens[current_index..].to_vec());
        if identifier_result.is_none() {
            return Err(GeneralIssue {
                issues: vec![IssueBase {
                    level: IssueLevel::Info,
                    position: IssuePosition::Parsing,
                    code: "".to_string(),
                    detail: "Invalid identifier".to_string(),
                }]
            });
        }

        return Ok((Action::new(ActionContent::DeclarationStatement(result), vec![]), next_semicolon_pos.unwrap() + 1));
    }

    return Err(GeneralIssue {
        issues: vec![IssueBase {
            level: IssueLevel::Info,
            position: IssuePosition::Parsing,
            code: "".to_string(),
            detail: "".to_string(),
        }]
    });
}
