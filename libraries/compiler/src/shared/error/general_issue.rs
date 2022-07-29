#[derive(Clone, Debug)]
pub struct GeneralIssue<T> {
    pub issues: Vec<IssueBase<T>>,
}

#[derive(Clone, Debug)]
pub enum IssueLevel {
    Info,
    Warning,
    Error
}

#[derive(Clone, Debug)]
pub enum IssuePosition {
    LexicalAnalysis,
    Parsing,
    CodeGeneration,
}

#[derive(Clone, Debug)]
pub struct IssueBase<T> {
    pub level: IssueLevel,
    pub position: IssuePosition,
    pub code: String,
    pub detail: T,
}

#[derive(Clone, Debug)]
pub struct FileMatch {
    pub file_path: String,
    pub start_pos: usize,
    pub end_pos: usize,
}
