use crate::parser::utils::find_next_semicolon;
use crate::shared::ast::action::{Action, ActionContent, DeclarationAction};
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::error::general_issue::{GeneralIssue, IssueBase, IssueLevel, IssuePosition};
use crate::shared::token::keyword::KeywordType;

pub fn declaration_action_builder(
    tokens: &Vec<DecoratedToken>,
) -> Result<(Action, usize), GeneralIssue<String>> {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());
    // Each block owns 4 tokens only
    if next_semicolon_pos.unwrap_or(0) == 4 {
        if tokens[0].content.get_decorated_keyword().is_some()
            && tokens[1].content.get_decorated_keyword().is_some()
            && tokens[2].content.is_valid_identifier()
            && tokens[3].content.is_valid_identifier()
        {
            // Match declaration statement format: decl <var|const> <data_type> <identifier>

            let mut result = DeclarationAction {
                is_variable: false,
                identifier: tokens[3].content.get_data().unwrap().get_identifier().unwrap().clone(),
                data_type: tokens[2].content.get_data().unwrap().get_identifier().unwrap().clone(),
            };

            // Lead the Declaration statement
            if *tokens[0].content.get_decorated_keyword().unwrap() == KeywordType::KwDeclare {
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
                            detail: "".to_string(),
                        }]
                    });
                }

                return Ok((Action::new(ActionContent::DeclarationStatement(result), vec![]), next_semicolon_pos.unwrap() + 1));
            }
        }
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
