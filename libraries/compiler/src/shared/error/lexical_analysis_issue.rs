use crate::shared::error::general_issue::FileMatch;

#[derive(Debug, Clone)]
pub struct LexicalAnalysisIssue {
    pub location: FileMatch
}
