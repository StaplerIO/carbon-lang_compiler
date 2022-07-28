use crate::parser::builder::templates::condition_block_builder;
use crate::shared::ast::action::{Action, ActionContent};
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::error::general_issue::{GeneralIssue, IssueLevel};
use crate::shared::token::keyword::KeywordType;

// result.1 : The end of the while statement (the last anti-brace)
pub fn while_action_builder(
    tokens: &Vec<DecoratedToken>,
) -> Result<(Action, usize), GeneralIssue<String>> {
    let result = condition_block_builder(KeywordType::KwWhile, tokens.clone());
    if result.is_ok() {
        return Ok((
            Action::new(ActionContent::WhileStatement(result.clone().ok().unwrap().0), vec![]),
            result.ok().unwrap().1,
        ));
    }

    return Err(GeneralIssue {
        level: IssueLevel::Info,
        code: "-1".to_string(),
        description: String::new(),
    });
}

// Action of keyword "loop" is in file "./short_actions.rs"
