use crate::parser::builder::blocks::action_block::action_block_builder;
use crate::parser::utils::find_next_semicolon;
use crate::shared::ast::action::{Action, ActionContent, LoopBlock};
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::error::general_issue::{GeneralIssue, IssueBase, IssueLevel, IssuePosition};
use crate::shared::token::container::ContainerType;
use crate::shared::token::keyword::KeywordType;

// Build "continue", "break" and "loop" action
pub fn short_statements_builder(
    tokens: &Vec<DecoratedToken>,
) -> Result<(Action, usize), GeneralIssue<String>> {
    let next_semicolon_pos = find_next_semicolon(tokens.clone());

    if tokens[0].content.get_decorated_keyword().is_some(){
        if next_semicolon_pos.unwrap_or(0) == 1 {
            let keyword = *tokens[0].content.get_decorated_keyword().unwrap();

            // "break" or "continue"
            match keyword {
                KeywordType::KwContinue => {
                    return Ok((Action::new(ActionContent::ContinueStatement, vec![]), next_semicolon_pos.unwrap() + 1));
                }
                KeywordType::KwBreak => {
                    return Ok((Action::new(ActionContent::BreakStatement, vec![]), next_semicolon_pos.unwrap() + 1));
                }
                _ => {}
            }

            // Match a "loop" action
            if *tokens[0].content.get_decorated_keyword().unwrap() == KeywordType::KwLoop
                && tokens[1].content.get_container().is_some()
                && tokens[next_semicolon_pos.unwrap() - 1].content.get_container().is_some()
            {
                // The shortest token stream: loop { }
                // Check if the statement is lead by keyword "loop"
                if *tokens[1].content.get_container().unwrap() == ContainerType::Brace
                    && *tokens[next_semicolon_pos.unwrap() - 1].content.get_container().unwrap() == ContainerType::AntiBrace
                {
                    let container_content = tokens[2..(next_semicolon_pos.unwrap() - 2)].to_vec();

                    return Ok((
                        Action::new(ActionContent::LoopBlock(LoopBlock {
                            actions: action_block_builder(container_content.clone()),
                        }), vec![]),
                        0,
                    ));
                }
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
