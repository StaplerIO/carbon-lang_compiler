use crate::shared::error::general_issue::FileMatch;

#[derive(Debug, Clone)]
pub struct ParsingIssue {
    pub content: String,
    pub location: FileMatch
}
