use crate::shared::ast::blocks::function::Function;

pub struct ParserPackageStructure {
    pub functions: Vec<Function>,
    // The entry of the package (executable)
    pub entry_point: String,

    // Will be solved on package generation
    pub linked_code_files: Vec<String>,
}
