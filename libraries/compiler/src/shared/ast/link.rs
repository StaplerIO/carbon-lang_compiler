use std::path::PathBuf;
use crate::shared::utils::identifier::Identifier;

#[derive(Debug, Clone)]
pub enum SourceFileLink {
    SourceFile(PathBuf),
    Identifier(Identifier),
}
