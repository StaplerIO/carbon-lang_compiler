use crate::parser::builder::blocks::assignment::assignment_block_builder;
use crate::parser::builder::blocks::call::call_action_builder;
use crate::parser::builder::blocks::condition::if_block_builder;
use crate::parser::builder::blocks::declaration::declaration_action_builder;
use crate::parser::builder::blocks::loops::while_action_builder;
use crate::parser::builder::blocks::return_expression::return_action_builder;
use crate::parser::builder::blocks::short_actions::short_statements_builder;
use crate::shared::ast::action::Action;
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::error::general_issue::{FileMatch, GeneralIssue, IssueBase, IssueLevel, IssuePosition};
use crate::shared::error::parsing_issue::ParsingIssue;

// Lookup lexer/tokenize.rs
pub fn action_block_builder(mut tokens: Vec<DecoratedToken>) -> Result<Vec<Action>, GeneralIssue<ParsingIssue>> {
    if tokens.is_empty() {
        return Ok(vec![]);
    }

    let mut result: Vec<Action> = Vec::new();

    // Configure issue handler
    let mut errored = false;
    let mut start_token: DecoratedToken = tokens[0].clone();
    let mut latest_token: DecoratedToken = tokens.last().unwrap().clone();
    let mut issue_list: Vec<IssueBase<ParsingIssue>> = vec![];

    // Needs to be simplified
    while tokens.len() > 0 {
        let decl = declaration_action_builder(&tokens.clone());
        if decl.is_ok() {
            if errored {
                errored = false;
                issue_list.push(IssueBase{
                    level: IssueLevel::Error,
                    position: IssuePosition::Parsing,
                    code: "0002".to_string(),
                    detail: ParsingIssue {
                        content: "Unrecognizable token sequence".to_string(),
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: start_token.original_token.position.start,
                            end_pos: latest_token.original_token.position.start + latest_token.original_token.position.length
                        }
                    }});
            }

            result.push(decl.clone().ok().unwrap().0);

            tokens = tokens[decl.ok().unwrap().1..].to_vec();
            continue;
        }

        let assign_action = assignment_block_builder(&tokens.clone());
        if assign_action.is_ok() {
            if errored {
                errored = false;
                issue_list.push(IssueBase{
                    level: IssueLevel::Error,
                    position: IssuePosition::Parsing,
                    code: "0002".to_string(),
                    detail: ParsingIssue {
                        content: "Unrecognizable token sequence".to_string(),
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: start_token.original_token.position.start,
                            end_pos: latest_token.original_token.position.start + latest_token.original_token.position.length
                        }
                    }});
            }

            result.push(assign_action.clone().ok().unwrap().0);

            tokens = tokens[assign_action.ok().unwrap().1..].to_vec();
            continue;
        }

        let call_action = call_action_builder(&tokens.clone());
        if call_action.is_ok() {
            if errored {
                errored = false;
                issue_list.push(IssueBase{
                    level: IssueLevel::Error,
                    position: IssuePosition::Parsing,
                    code: "0002".to_string(),
                    detail: ParsingIssue {
                        content: "Unrecognizable token sequence".to_string(),
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: start_token.original_token.position.start,
                            end_pos: latest_token.original_token.position.start + latest_token.original_token.position.length
                        }
                    }});
            }

            result.push(call_action.clone().ok().unwrap().0);

            tokens = tokens[call_action.ok().unwrap().1..].to_vec();
            continue;
        }

        let return_action = return_action_builder(&tokens.clone());
        if return_action.is_ok() {
            if errored {
                errored = false;
                issue_list.push(IssueBase{
                    level: IssueLevel::Error,
                    position: IssuePosition::Parsing,
                    code: "0002".to_string(),
                    detail: ParsingIssue {
                        content: "Unrecognizable token sequence".to_string(),
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: start_token.original_token.position.start,
                            end_pos: latest_token.original_token.position.start + latest_token.original_token.position.length
                        }
                    }});
            }

            result.push(return_action.clone().ok().unwrap().0);

            tokens = tokens[return_action.ok().unwrap().1..].to_vec();
            continue;
        }

        let if_action = if_block_builder(&tokens.clone());
        if if_action.is_ok() {
            if errored {
                errored = false;
                issue_list.push(IssueBase{
                    level: IssueLevel::Error,
                    position: IssuePosition::Parsing,
                    code: "0002".to_string(),
                    detail: ParsingIssue {
                        content: "Unrecognizable token sequence".to_string(),
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: start_token.original_token.position.start,
                            end_pos: latest_token.original_token.position.start + latest_token.original_token.position.length
                        }
                    }});
            }

            result.push(if_action.clone().ok().unwrap().0);

            tokens = tokens[if_action.ok().unwrap().1..].to_vec();
            continue;
        }

        let while_action = while_action_builder(&tokens.clone());
        if while_action.is_ok() {
            if errored {
                errored = false;
                issue_list.push(IssueBase{
                    level: IssueLevel::Error,
                    position: IssuePosition::Parsing,
                    code: "0002".to_string(),
                    detail: ParsingIssue {
                        content: "Unrecognizable token sequence".to_string(),
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: start_token.original_token.position.start,
                            end_pos: latest_token.original_token.position.start + latest_token.original_token.position.length
                        }
                    }});
            }

            result.push(while_action.clone().ok().unwrap().0);

            tokens = tokens[while_action.ok().unwrap().1..].to_vec();
            continue;
        }

        let other_action = short_statements_builder(&tokens.clone());
        if other_action.is_ok() {
            if errored {
                errored = false;
                issue_list.push(IssueBase{
                    level: IssueLevel::Error,
                    position: IssuePosition::Parsing,
                    code: "0002".to_string(),
                    detail: ParsingIssue {
                        content: "Unrecognizable token sequence".to_string(),
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: start_token.original_token.position.start,
                            end_pos: latest_token.original_token.position.start + latest_token.original_token.position.length
                        }
                    }});
            }

            result.push(other_action.clone().ok().unwrap().0);

            tokens = tokens[other_action.ok().unwrap().1..].to_vec();
            continue;
        }

        // Initiate error handling
        if !errored {
            errored = true;
            start_token = tokens[0].clone();
        }
        latest_token = tokens[1].clone();
        tokens = tokens[1..].to_vec();
    }

    return if issue_list.is_empty() {
        Ok(result)
    } else {
        Err(GeneralIssue { issues: issue_list })
    };
}
