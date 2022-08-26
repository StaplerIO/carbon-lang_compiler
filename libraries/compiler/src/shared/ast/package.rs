use crate::shared::ast::blocks::function::Function;
use crate::shared::utils::identifier::Identifier;

#[derive(Debug, Clone)]
pub struct ParserPackageStructure {
    pub functions: Vec<Function>,
    // The entry of the package (executable)
    pub entry_point: Identifier,

    // Will be solved on package generation
    pub linked_code_files: Vec<String>,
}
