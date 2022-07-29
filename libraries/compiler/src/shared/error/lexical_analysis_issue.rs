use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct LexicalAnalysisIssue {
    pub file_path: String,
    pub start_index: usize,
    pub end_index: usize,

}

impl Display for LexicalAnalysisIssue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Source code slice not recognized in file \"{}\" from position {} to {}", self.file_path, self.start_index, self.end_index)
    }
}
