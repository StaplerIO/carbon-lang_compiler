#[derive(Clone, Debug)]
pub struct GeneralIssue<T> {
    pub level: IssueLevel,
    pub code: String,
    pub description: T,
}

#[derive(Clone, Debug)]
pub enum IssueLevel {
    Info,
    Warning,
    Error
}
