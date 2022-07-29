use std::fmt::{Display, Formatter};
use crate::shared::error::general_issue::FileMatch;
use crate::shared::error::lexical_analysis_issue::LexicalAnalysisIssue;
use crate::shared::error::parsing_issue::ParsingIssue;

impl Display for LexicalAnalysisIssue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Source code slice not recognized {}", self.location)
    }
}

impl Display for ParsingIssue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl Display for FileMatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "in file {} from position {} to {}", self.file_path, self.start_pos, self.end_pos)
    }
}
